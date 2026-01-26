use std::fs;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Result;

use crate::benchmarks::{Benchmark, BenchmarkConfig, Category, ProgressCallback};
use crate::core::{CommandExt, Timer};
use crate::models::{TestDetails, TestResult};

/// C# Compilation benchmark - tests build performance using dotnet CLI
/// Requires .NET SDK to be installed. Creates a realistic C# project with
/// multiple source files and measures compilation time.
pub struct CSharpCompileBenchmark {
    test_dir: PathBuf,
}

impl CSharpCompileBenchmark {
    pub fn new() -> Self {
        Self {
            test_dir: std::env::temp_dir().join("workbench_pro_csharp_compile"),
        }
    }

    /// Check if dotnet CLI is available
    fn is_dotnet_available() -> bool {
        Command::new("dotnet")
            .arg("--version")
            .hidden()
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    /// Get dotnet version for reporting
    fn get_dotnet_version() -> Option<String> {
        Command::new("dotnet")
            .arg("--version")
            .hidden()
            .output()
            .ok()
            .filter(|o| o.status.success())
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
    }

    fn create_project(&self, progress: &dyn ProgressCallback) -> Result<()> {
        // Clean up any existing test directory
        let _ = fs::remove_dir_all(&self.test_dir);
        fs::create_dir_all(&self.test_dir)?;

        progress.update(0.05, "Creating .NET project...");

        // Create project file
        let csproj_content = r#"<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net8.0</TargetFramework>
    <ImplicitUsings>enable</ImplicitUsings>
    <Nullable>enable</Nullable>
  </PropertyGroup>
</Project>
"#;
        fs::write(self.test_dir.join("BenchmarkApp.csproj"), csproj_content)?;

        progress.update(0.1, "Generating C# source files...");

        // Create main program file
        let main_content = r#"using System;
using System.Collections.Generic;
using BenchmarkApp.Models;
using BenchmarkApp.Processing;
using BenchmarkApp.Math;
using BenchmarkApp.Utils;

namespace BenchmarkApp;

class Program
{
    static void Main(string[] args)
    {
        Console.WriteLine("Benchmark Application - C# Compilation Test");
        Console.WriteLine("============================================");

        // Test data processing
        var processor = new DataProcessor();
        var data = processor.GenerateData(1000);
        var result = processor.ProcessData(data);
        Console.WriteLine($"Processed {data.Count} items, result: {result:F2}");

        // Test math calculations
        var calculator = new MathCalculator();
        var mathResult = calculator.ComputeAll(100);
        Console.WriteLine($"Math computation result: {mathResult:F6}");

        // Test matrix operations
        var matrix = new Matrix(10, 10);
        matrix.FillRandom(42);
        var transposed = matrix.Transpose();
        Console.WriteLine($"Matrix transposed: {transposed.Rows}x{transposed.Cols}");

        // Test string utilities
        var text = "hello_world_benchmark_test";
        var camelCase = StringUtils.ToCamelCase(text);
        Console.WriteLine($"CamelCase: {camelCase}");

        // Test collections
        var numbers = new List<int> { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 };
        var shuffled = CollectionUtils.Shuffle(numbers, new Random(42));
        Console.WriteLine($"Shuffled: {string.Join(", ", shuffled)}");

        // Test async operations
        var asyncProcessor = new AsyncDataProcessor();
        var asyncResult = asyncProcessor.ProcessAsync(data).GetAwaiter().GetResult();
        Console.WriteLine($"Async processing complete: {asyncResult.Success}");

        Console.WriteLine("============================================");
        Console.WriteLine("All tests completed successfully!");
    }
}
"#;
        fs::create_dir_all(self.test_dir.join("src"))?;
        fs::write(self.test_dir.join("src/Program.cs"), main_content)?;

        progress.update(0.15, "Generating model classes...");

        // Create models
        fs::create_dir_all(self.test_dir.join("src/Models"))?;
        let models_content = r#"using System;
using System.Collections.Generic;

namespace BenchmarkApp.Models;

public class DataItem
{
    public int Id { get; set; }
    public string Name { get; set; } = string.Empty;
    public DateTime Created { get; set; }
    public List<double> Values { get; set; } = new();
    public Dictionary<string, object> Metadata { get; set; } = new();

    public DataItem() { }

    public DataItem(int id, string name)
    {
        Id = id;
        Name = name;
        Created = DateTime.Now;
    }

    public double ComputeSum() => Values.Count > 0 ? Values.Sum() : 0;
    public double ComputeAverage() => Values.Count > 0 ? Values.Average() : 0;
    public double ComputeMin() => Values.Count > 0 ? Values.Min() : 0;
    public double ComputeMax() => Values.Count > 0 ? Values.Max() : 0;

    public double ComputeVariance()
    {
        if (Values.Count == 0) return 0;
        var avg = ComputeAverage();
        return Values.Select(v => Math.Pow(v - avg, 2)).Average();
    }

    public double ComputeStdDev() => Math.Sqrt(ComputeVariance());

    public override string ToString() => $"DataItem[Id={Id}, Name={Name}, Values={Values.Count}]";
}

public class DataContainer
{
    public List<DataItem> Items { get; set; } = new();
    public string ContainerName { get; set; } = string.Empty;
    public int Version { get; set; } = 1;
    public DateTime CreatedAt { get; set; } = DateTime.Now;

    public void AddItem(DataItem item) => Items.Add(item);
    public DataItem? FindById(int id) => Items.FirstOrDefault(i => i.Id == id);
    public IEnumerable<DataItem> FindByName(string name) => Items.Where(i => i.Name.Contains(name));
    public int Count => Items.Count;
}

public class Result<T>
{
    public bool Success { get; set; }
    public T? Value { get; set; }
    public string? ErrorMessage { get; set; }
    public Exception? Exception { get; set; }

    public static Result<T> Ok(T value) => new() { Success = true, Value = value };
    public static Result<T> Error(string message) => new() { Success = false, ErrorMessage = message };
    public static Result<T> FromException(Exception ex) => new() { Success = false, Exception = ex, ErrorMessage = ex.Message };
}

public record Person(string FirstName, string LastName, int Age)
{
    public string FullName => $"{FirstName} {LastName}";
    public bool IsAdult => Age >= 18;
}

public record Address(string Street, string City, string Country, string PostalCode);

public record Customer(Person Person, Address Address, string Email)
{
    public string DisplayName => $"{Person.FullName} ({Email})";
}
"#;
        fs::write(self.test_dir.join("src/Models/DataModels.cs"), models_content)?;

        progress.update(0.25, "Generating processing classes...");

        // Create processing classes
        fs::create_dir_all(self.test_dir.join("src/Processing"))?;
        let processor_content = r#"using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using BenchmarkApp.Models;

namespace BenchmarkApp.Processing;

public class DataProcessor
{
    private readonly Random _random = new(42);

    public List<DataItem> GenerateData(int count)
    {
        var items = new List<DataItem>(count);

        for (int i = 0; i < count; i++)
        {
            var item = new DataItem(i, $"Item_{i:D4}");

            for (int j = 0; j < 10; j++)
            {
                item.Values.Add(_random.NextDouble() * 100);
            }

            item.Metadata["category"] = i % 5;
            item.Metadata["priority"] = i % 3;
            item.Metadata["tags"] = new List<string> { "tag1", "tag2", $"tag{i % 10}" };

            items.Add(item);
        }

        return items;
    }

    public double ProcessData(List<DataItem> items)
    {
        double total = 0;
        foreach (var item in items)
        {
            total += item.ComputeSum();
            total += item.ComputeAverage();
            total += item.ComputeStdDev();
        }
        return total;
    }

    public List<DataItem> FilterByCategory(List<DataItem> items, int category)
    {
        return items
            .Where(item => item.Metadata.TryGetValue("category", out var cat) && (int)cat == category)
            .ToList();
    }

    public Dictionary<int, List<DataItem>> GroupByCategory(List<DataItem> items)
    {
        return items
            .Where(item => item.Metadata.ContainsKey("category"))
            .GroupBy(item => (int)item.Metadata["category"])
            .ToDictionary(g => g.Key, g => g.ToList());
    }

    public List<DataItem> SortBySum(List<DataItem> items, bool descending = false)
    {
        return descending
            ? items.OrderByDescending(i => i.ComputeSum()).ToList()
            : items.OrderBy(i => i.ComputeSum()).ToList();
    }

    public Result<double> TryComputeStatistics(List<DataItem> items)
    {
        if (items == null || items.Count == 0)
            return Result<double>.Error("No items provided");

        try
        {
            var sums = items.Select(i => i.ComputeSum()).ToList();
            var avg = sums.Average();
            return Result<double>.Ok(avg);
        }
        catch (Exception ex)
        {
            return Result<double>.FromException(ex);
        }
    }
}

public class AsyncDataProcessor
{
    public async Task<Result<double>> ProcessAsync(List<DataItem> items)
    {
        await Task.Delay(1); // Simulate async work

        var tasks = items.Select(async item =>
        {
            await Task.Yield();
            return item.ComputeSum();
        });

        var results = await Task.WhenAll(tasks);
        return Result<double>.Ok(results.Sum());
    }

    public async IAsyncEnumerable<DataItem> StreamDataAsync(int count)
    {
        var random = new Random(42);
        for (int i = 0; i < count; i++)
        {
            await Task.Yield();
            var item = new DataItem(i, $"Stream_{i}");
            item.Values.Add(random.NextDouble() * 100);
            yield return item;
        }
    }
}

public class ParallelProcessor
{
    public double ProcessParallel(List<DataItem> items)
    {
        var result = 0.0;
        var lockObj = new object();

        Parallel.ForEach(items, item =>
        {
            var sum = item.ComputeSum();
            lock (lockObj)
            {
                result += sum;
            }
        });

        return result;
    }

    public List<double> MapParallel(List<DataItem> items, Func<DataItem, double> selector)
    {
        var results = new double[items.Count];
        Parallel.For(0, items.Count, i =>
        {
            results[i] = selector(items[i]);
        });
        return results.ToList();
    }
}
"#;
        fs::write(self.test_dir.join("src/Processing/DataProcessor.cs"), processor_content)?;

        progress.update(0.35, "Generating math classes...");

        // Create math classes
        fs::create_dir_all(self.test_dir.join("src/Math"))?;
        let mut math_content = String::from(r#"using System;

namespace BenchmarkApp.Math;

public class MathCalculator
{
    public double ComputeAll(int iterations)
    {
        double result = 0;
"#);

        // Generate method calls
        for i in 0..30 {
            math_content.push_str(&format!(
                "        result += Compute{}(iterations, result);\n",
                i
            ));
        }

        math_content.push_str(r#"        return result;
    }
"#);

        // Generate computation methods
        for i in 0..30 {
            math_content.push_str(&format!(
                r#"
    public double Compute{0}(int n, double seed)
    {{
        double result = seed + {0};
        for (int i = 0; i < n; i++)
        {{
            result = System.Math.Sin(result) * System.Math.Cos(result * {0}.{1});
            result = System.Math.Sqrt(System.Math.Abs(result) + 1);
            result += System.Math.Log(System.Math.Abs(result) + 1) * {2}.0;
            result = System.Math.Tanh(result * 0.1);
        }}
        return result;
    }}
"#,
                i,
                (i + 1) % 10,
                i + 1
            ));
        }

        math_content.push_str(r#"
    public double[] GenerateSequence(int length)
    {
        var sequence = new double[length];
        for (int i = 0; i < length; i++)
        {
            sequence[i] = System.Math.Sin(i * 0.1) * System.Math.Cos(i * 0.2);
        }
        return sequence;
    }

    public double ComputeStandardDeviation(double[] values)
    {
        if (values.Length == 0) return 0;
        double mean = values.Average();
        double sumSquares = values.Select(v => System.Math.Pow(v - mean, 2)).Sum();
        return System.Math.Sqrt(sumSquares / values.Length);
    }
}

public class Matrix
{
    private double[,] _data;
    public int Rows { get; }
    public int Cols { get; }

    public Matrix(int rows, int cols)
    {
        Rows = rows;
        Cols = cols;
        _data = new double[rows, cols];
    }

    public double this[int row, int col]
    {
        get => _data[row, col];
        set => _data[row, col] = value;
    }

    public void Fill(double value)
    {
        for (int i = 0; i < Rows; i++)
            for (int j = 0; j < Cols; j++)
                _data[i, j] = value;
    }

    public void FillRandom(int seed)
    {
        var random = new Random(seed);
        for (int i = 0; i < Rows; i++)
            for (int j = 0; j < Cols; j++)
                _data[i, j] = random.NextDouble();
    }

    public Matrix Transpose()
    {
        var result = new Matrix(Cols, Rows);
        for (int i = 0; i < Rows; i++)
            for (int j = 0; j < Cols; j++)
                result[j, i] = _data[i, j];
        return result;
    }

    public static Matrix Multiply(Matrix a, Matrix b)
    {
        if (a.Cols != b.Rows)
            throw new ArgumentException("Matrix dimensions don't match");

        var result = new Matrix(a.Rows, b.Cols);
        for (int i = 0; i < a.Rows; i++)
        {
            for (int j = 0; j < b.Cols; j++)
            {
                double sum = 0;
                for (int k = 0; k < a.Cols; k++)
                    sum += a[i, k] * b[k, j];
                result[i, j] = sum;
            }
        }
        return result;
    }

    public double Determinant()
    {
        if (Rows != Cols)
            throw new InvalidOperationException("Matrix must be square");
        if (Rows == 1) return _data[0, 0];
        if (Rows == 2) return _data[0, 0] * _data[1, 1] - _data[0, 1] * _data[1, 0];

        double det = 0;
        for (int j = 0; j < Cols; j++)
        {
            det += System.Math.Pow(-1, j) * _data[0, j] * Minor(0, j).Determinant();
        }
        return det;
    }

    private Matrix Minor(int rowToRemove, int colToRemove)
    {
        var result = new Matrix(Rows - 1, Cols - 1);
        int ri = 0;
        for (int i = 0; i < Rows; i++)
        {
            if (i == rowToRemove) continue;
            int rj = 0;
            for (int j = 0; j < Cols; j++)
            {
                if (j == colToRemove) continue;
                result[ri, rj] = _data[i, j];
                rj++;
            }
            ri++;
        }
        return result;
    }
}
"#);
        fs::write(self.test_dir.join("src/Math/MathCalculator.cs"), math_content)?;

        progress.update(0.45, "Generating utility classes...");

        // Create utility classes
        fs::create_dir_all(self.test_dir.join("src/Utils"))?;
        let utils_content = r#"using System;
using System.Collections.Generic;
using System.Text;
using System.Text.RegularExpressions;

namespace BenchmarkApp.Utils;

public static partial class StringUtils
{
    public static string Reverse(string input)
    {
        if (string.IsNullOrEmpty(input)) return input;
        var chars = input.ToCharArray();
        Array.Reverse(chars);
        return new string(chars);
    }

    public static string ToCamelCase(string input)
    {
        if (string.IsNullOrEmpty(input)) return input;

        var sb = new StringBuilder();
        bool capitalizeNext = false;
        bool isFirst = true;

        foreach (char c in input)
        {
            if (c == '_' || c == ' ' || c == '-')
            {
                capitalizeNext = true;
            }
            else if (capitalizeNext)
            {
                sb.Append(char.ToUpper(c));
                capitalizeNext = false;
            }
            else
            {
                sb.Append(isFirst ? char.ToLower(c) : c);
            }
            isFirst = false;
        }

        return sb.ToString();
    }

    public static string ToPascalCase(string input)
    {
        var camel = ToCamelCase(input);
        if (string.IsNullOrEmpty(camel)) return camel;
        return char.ToUpper(camel[0]) + camel[1..];
    }

    public static string ToSnakeCase(string input)
    {
        if (string.IsNullOrEmpty(input)) return input;
        var sb = new StringBuilder();

        foreach (char c in input)
        {
            if (char.IsUpper(c) && sb.Length > 0)
            {
                sb.Append('_');
            }
            sb.Append(char.ToLower(c));
        }

        return sb.ToString();
    }

    public static int CountWords(string input)
    {
        if (string.IsNullOrWhiteSpace(input)) return 0;
        return input.Split(new[] { ' ', '\t', '\n', '\r' }, StringSplitOptions.RemoveEmptyEntries).Length;
    }

    public static string Truncate(string input, int maxLength, string suffix = "...")
    {
        if (string.IsNullOrEmpty(input) || input.Length <= maxLength) return input;
        return input[..(maxLength - suffix.Length)] + suffix;
    }

    [GeneratedRegex(@"\s+")]
    private static partial Regex WhitespaceRegex();

    public static string NormalizeWhitespace(string input)
    {
        if (string.IsNullOrEmpty(input)) return input;
        return WhitespaceRegex().Replace(input.Trim(), " ");
    }
}

public static class CollectionUtils
{
    public static List<T> Shuffle<T>(List<T> list, Random random)
    {
        var result = new List<T>(list);
        int n = result.Count;
        while (n > 1)
        {
            n--;
            int k = random.Next(n + 1);
            (result[k], result[n]) = (result[n], result[k]);
        }
        return result;
    }

    public static T[] Resize<T>(T[] array, int newSize)
    {
        var result = new T[newSize];
        Array.Copy(array, result, Math.Min(array.Length, newSize));
        return result;
    }

    public static Dictionary<K, V> Merge<K, V>(Dictionary<K, V> first, Dictionary<K, V> second) where K : notnull
    {
        var result = new Dictionary<K, V>(first);
        foreach (var kvp in second)
        {
            result[kvp.Key] = kvp.Value;
        }
        return result;
    }

    public static IEnumerable<IEnumerable<T>> Batch<T>(IEnumerable<T> source, int batchSize)
    {
        var batch = new List<T>(batchSize);
        foreach (var item in source)
        {
            batch.Add(item);
            if (batch.Count >= batchSize)
            {
                yield return batch;
                batch = new List<T>(batchSize);
            }
        }
        if (batch.Count > 0)
            yield return batch;
    }

    public static IEnumerable<(T Item, int Index)> WithIndex<T>(IEnumerable<T> source)
    {
        int index = 0;
        foreach (var item in source)
        {
            yield return (item, index++);
        }
    }
}

public class Logger
{
    private readonly List<LogEntry> _entries = new();
    private readonly string _name;
    private readonly LogLevel _minLevel;

    public Logger(string name, LogLevel minLevel = LogLevel.Info)
    {
        _name = name;
        _minLevel = minLevel;
    }

    public void Debug(string message) => Log(LogLevel.Debug, message);
    public void Info(string message) => Log(LogLevel.Info, message);
    public void Warning(string message) => Log(LogLevel.Warning, message);
    public void Error(string message) => Log(LogLevel.Error, message);
    public void Error(Exception ex) => Log(LogLevel.Error, $"{ex.GetType().Name}: {ex.Message}");

    private void Log(LogLevel level, string message)
    {
        if (level < _minLevel) return;
        _entries.Add(new LogEntry(DateTime.Now, level, _name, message));
    }

    public IReadOnlyList<LogEntry> GetEntries() => _entries.AsReadOnly();
    public void Clear() => _entries.Clear();
}

public enum LogLevel { Debug, Info, Warning, Error }

public record LogEntry(DateTime Timestamp, LogLevel Level, string Logger, string Message)
{
    public override string ToString() => $"[{Timestamp:yyyy-MM-dd HH:mm:ss}] [{Level}] [{Logger}] {Message}";
}

public class Cache<TKey, TValue> where TKey : notnull
{
    private readonly Dictionary<TKey, CacheEntry<TValue>> _cache = new();
    private readonly TimeSpan _defaultExpiry;

    public Cache(TimeSpan? defaultExpiry = null)
    {
        _defaultExpiry = defaultExpiry ?? TimeSpan.FromMinutes(5);
    }

    public void Set(TKey key, TValue value, TimeSpan? expiry = null)
    {
        _cache[key] = new CacheEntry<TValue>(value, DateTime.Now + (expiry ?? _defaultExpiry));
    }

    public TValue? Get(TKey key)
    {
        if (_cache.TryGetValue(key, out var entry) && entry.ExpiresAt > DateTime.Now)
            return entry.Value;
        return default;
    }

    public bool TryGet(TKey key, out TValue? value)
    {
        if (_cache.TryGetValue(key, out var entry) && entry.ExpiresAt > DateTime.Now)
        {
            value = entry.Value;
            return true;
        }
        value = default;
        return false;
    }

    public void Remove(TKey key) => _cache.Remove(key);

    public void Clear() => _cache.Clear();

    public void Cleanup()
    {
        var now = DateTime.Now;
        var expiredKeys = _cache.Where(kvp => kvp.Value.ExpiresAt <= now).Select(kvp => kvp.Key).ToList();
        foreach (var key in expiredKeys)
            _cache.Remove(key);
    }
}

internal record CacheEntry<T>(T Value, DateTime ExpiresAt);
"#;
        fs::write(self.test_dir.join("src/Utils/Utilities.cs"), utils_content)?;

        Ok(())
    }

    fn cleanup(&self) {
        let _ = fs::remove_dir_all(&self.test_dir);
    }
}

impl Default for CSharpCompileBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for CSharpCompileBenchmark {
    fn id(&self) -> &'static str {
        "csharp_compile"
    }

    fn name(&self) -> &'static str {
        "C# Compilation"
    }

    fn description(&self) -> &'static str {
        "Multi-file C# project compilation with dotnet build"
    }

    fn category(&self) -> Category {
        Category::BuildPerformance
    }

    fn estimated_duration_secs(&self) -> u32 {
        60
    }

    fn run(&self, progress: &dyn ProgressCallback, config: &BenchmarkConfig) -> Result<TestResult> {
        // Check if dotnet is available
        if !Self::is_dotnet_available() {
            return Err(anyhow::anyhow!(
                ".NET SDK not found. Install from https://dotnet.microsoft.com/download"
            ));
        }

        let dotnet_version = Self::get_dotnet_version().unwrap_or_else(|| "unknown".to_string());
        progress.update(0.02, &format!("Found .NET SDK: {}", dotnet_version));

        // Create the test project
        self.create_project(progress)?;

        let mut compile_times: Vec<f64> = Vec::new();
        let iterations = config.iterations as usize;

        progress.update(0.5, "Running compilation benchmarks...");

        // Run multiple compilation iterations
        for i in 0..iterations {
            if progress.is_cancelled() {
                self.cleanup();
                return Err(anyhow::anyhow!("Cancelled"));
            }

            // Clean build artifacts to force full recompile
            let _ = fs::remove_dir_all(self.test_dir.join("bin"));
            let _ = fs::remove_dir_all(self.test_dir.join("obj"));

            let timer = Timer::new();

            let output = Command::new("dotnet")
                .arg("build")
                .arg("--configuration")
                .arg("Release")
                .arg("--verbosity")
                .arg("quiet")
                .current_dir(&self.test_dir)
                .hidden()
                .output()?;

            let elapsed = timer.elapsed_secs();

            if !output.status.success() {
                self.cleanup();
                let stderr = String::from_utf8_lossy(&output.stderr);
                let stdout = String::from_utf8_lossy(&output.stdout);
                return Err(anyhow::anyhow!(
                    "Compilation failed:\nstderr: {}\nstdout: {}",
                    stderr,
                    stdout
                ));
            }

            compile_times.push(elapsed);

            progress.update(
                0.5 + (i as f32 / iterations as f32) * 0.45,
                &format!("Compilation {}/{} ({:.2}s)...", i + 1, iterations, elapsed),
            );
        }

        // Cleanup
        progress.update(0.98, "Cleaning up...");
        self.cleanup();

        // Calculate statistics
        let avg_time = compile_times.iter().sum::<f64>() / compile_times.len() as f64;
        let min = compile_times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = compile_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        // Calculate standard deviation
        let variance = compile_times
            .iter()
            .map(|t| (t - avg_time).powi(2))
            .sum::<f64>()
            / compile_times.len() as f64;
        let std_dev = variance.sqrt();

        // Sort for median
        let mut sorted_times = compile_times.clone();
        sorted_times.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = sorted_times[sorted_times.len() / 2];

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: format!(
                "{} (.NET {}, avg: {:.2}s)",
                self.description(),
                dotnet_version,
                avg_time
            ),
            value: avg_time,
            unit: "s".to_string(),
            details: TestDetails {
                iterations: config.iterations,
                duration_secs: compile_times.iter().sum(),
                min,
                max,
                mean: avg_time,
                median,
                std_dev,
                percentiles: None,
            },
        })
    }
}
