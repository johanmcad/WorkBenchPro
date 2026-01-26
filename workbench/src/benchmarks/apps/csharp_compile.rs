use std::fs;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Result;

use crate::benchmarks::{Benchmark, BenchmarkConfig, Category, ProgressCallback};
use crate::core::{CommandExt, Timer};
use crate::models::{TestDetails, TestResult};

/// Native Compiler benchmark - tests build performance using Windows built-in csc.exe
/// This uses the .NET Framework C# compiler that ships with Windows, requiring no
/// additional tooling installation. Measures multi-file compilation with optimization.
pub struct CSharpCompileBenchmark {
    test_dir: PathBuf,
}

impl CSharpCompileBenchmark {
    pub fn new() -> Self {
        Self {
            test_dir: std::env::temp_dir().join("workbench_pro_csharp_compile"),
        }
    }

    /// Find the C# compiler (csc.exe) on the system
    fn find_csc() -> Option<PathBuf> {
        // Check common .NET Framework locations (prefer 64-bit)
        let framework_paths = [
            r"C:\Windows\Microsoft.NET\Framework64\v4.0.30319\csc.exe",
            r"C:\Windows\Microsoft.NET\Framework\v4.0.30319\csc.exe",
            r"C:\Windows\Microsoft.NET\Framework64\v3.5\csc.exe",
            r"C:\Windows\Microsoft.NET\Framework\v3.5\csc.exe",
        ];

        for path in &framework_paths {
            let p = PathBuf::from(path);
            if p.exists() {
                return Some(p);
            }
        }

        // Try to find via PATH
        if let Ok(output) = Command::new("where").arg("csc.exe").hidden().output() {
            if output.status.success() {
                let path_str = String::from_utf8_lossy(&output.stdout);
                if let Some(first_line) = path_str.lines().next() {
                    let p = PathBuf::from(first_line.trim());
                    if p.exists() {
                        return Some(p);
                    }
                }
            }
        }

        None
    }

    fn create_test_files(&self, progress: &dyn ProgressCallback) -> Result<Vec<PathBuf>> {
        // Clean up any existing test directory
        let _ = fs::remove_dir_all(&self.test_dir);
        fs::create_dir_all(&self.test_dir)?;

        progress.update(0.05, "Generating C# source files...");

        let mut source_files = Vec::new();

        // Create main program file
        let main_content = r#"using System;
using System.Collections.Generic;
using System.Linq;

namespace BenchmarkApp
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Benchmark Application");

            var processor = new DataProcessor();
            var data = processor.GenerateData(1000);
            var result = processor.ProcessData(data);

            Console.WriteLine($"Processed {data.Count} items, result: {result}");

            var calculator = new MathCalculator();
            var mathResult = calculator.ComputeAll(100);
            Console.WriteLine($"Math computation result: {mathResult}");
        }
    }
}
"#;
        let main_path = self.test_dir.join("Program.cs");
        fs::write(&main_path, main_content)?;
        source_files.push(main_path);

        progress.update(0.1, "Generating data classes...");

        // Create data model classes
        let models_content = r#"using System;
using System.Collections.Generic;

namespace BenchmarkApp
{
    public class DataItem
    {
        public int Id { get; set; }
        public string Name { get; set; }
        public DateTime Created { get; set; }
        public List<double> Values { get; set; }
        public Dictionary<string, object> Metadata { get; set; }

        public DataItem()
        {
            Values = new List<double>();
            Metadata = new Dictionary<string, object>();
        }

        public DataItem(int id, string name)
        {
            Id = id;
            Name = name;
            Created = DateTime.Now;
            Values = new List<double>();
            Metadata = new Dictionary<string, object>();
        }

        public double ComputeSum()
        {
            double sum = 0;
            foreach (var v in Values)
            {
                sum += v;
            }
            return sum;
        }

        public double ComputeAverage()
        {
            if (Values.Count == 0) return 0;
            return ComputeSum() / Values.Count;
        }

        public override string ToString()
        {
            return $"DataItem[Id={Id}, Name={Name}, Values={Values.Count}]";
        }
    }

    public class DataContainer
    {
        public List<DataItem> Items { get; set; }
        public string ContainerName { get; set; }
        public int Version { get; set; }

        public DataContainer()
        {
            Items = new List<DataItem>();
            Version = 1;
        }

        public void AddItem(DataItem item)
        {
            Items.Add(item);
        }

        public DataItem FindById(int id)
        {
            foreach (var item in Items)
            {
                if (item.Id == id) return item;
            }
            return null;
        }
    }

    public class Result<T>
    {
        public bool Success { get; set; }
        public T Value { get; set; }
        public string ErrorMessage { get; set; }

        public static Result<T> Ok(T value)
        {
            return new Result<T> { Success = true, Value = value };
        }

        public static Result<T> Error(string message)
        {
            return new Result<T> { Success = false, ErrorMessage = message };
        }
    }
}
"#;
        let models_path = self.test_dir.join("Models.cs");
        fs::write(&models_path, models_content)?;
        source_files.push(models_path);

        progress.update(0.15, "Generating processor classes...");

        // Create data processor class
        let processor_content = r#"using System;
using System.Collections.Generic;
using System.Linq;

namespace BenchmarkApp
{
    public class DataProcessor
    {
        private Random _random = new Random(42);

        public List<DataItem> GenerateData(int count)
        {
            var items = new List<DataItem>();

            for (int i = 0; i < count; i++)
            {
                var item = new DataItem(i, $"Item_{i}");

                for (int j = 0; j < 10; j++)
                {
                    item.Values.Add(_random.NextDouble() * 100);
                }

                item.Metadata["category"] = i % 5;
                item.Metadata["priority"] = i % 3;

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
            }

            return total;
        }

        public List<DataItem> FilterByCategory(List<DataItem> items, int category)
        {
            var result = new List<DataItem>();

            foreach (var item in items)
            {
                if (item.Metadata.ContainsKey("category"))
                {
                    if ((int)item.Metadata["category"] == category)
                    {
                        result.Add(item);
                    }
                }
            }

            return result;
        }

        public Dictionary<int, List<DataItem>> GroupByCategory(List<DataItem> items)
        {
            var groups = new Dictionary<int, List<DataItem>>();

            foreach (var item in items)
            {
                if (item.Metadata.ContainsKey("category"))
                {
                    int cat = (int)item.Metadata["category"];
                    if (!groups.ContainsKey(cat))
                    {
                        groups[cat] = new List<DataItem>();
                    }
                    groups[cat].Add(item);
                }
            }

            return groups;
        }

        public List<DataItem> SortBySum(List<DataItem> items)
        {
            var sorted = new List<DataItem>(items);
            sorted.Sort((a, b) => a.ComputeSum().CompareTo(b.ComputeSum()));
            return sorted;
        }

        public Result<double> TryComputeStatistics(List<DataItem> items)
        {
            if (items == null || items.Count == 0)
            {
                return Result<double>.Error("No items provided");
            }

            double sum = 0;
            double min = double.MaxValue;
            double max = double.MinValue;

            foreach (var item in items)
            {
                double itemSum = item.ComputeSum();
                sum += itemSum;
                if (itemSum < min) min = itemSum;
                if (itemSum > max) max = itemSum;
            }

            return Result<double>.Ok(sum / items.Count);
        }
    }
}
"#;
        let processor_path = self.test_dir.join("DataProcessor.cs");
        fs::write(&processor_path, processor_content)?;
        source_files.push(processor_path);

        progress.update(0.2, "Generating math classes...");

        // Create math calculator with many methods to increase compilation work
        let mut math_content = String::from(r#"using System;
using System.Collections.Generic;

namespace BenchmarkApp
{
    public class MathCalculator
    {
        public double ComputeAll(int iterations)
        {
            double result = 0;
"#);

        // Generate method calls
        for i in 0..30 {
            math_content.push_str(&format!(
                "            result += Compute{}(iterations, result);\n",
                i
            ));
        }

        math_content.push_str(r#"            return result;
        }
"#);

        // Generate many computation methods
        for i in 0..30 {
            math_content.push_str(&format!(
                r#"
        public double Compute{0}(int n, double seed)
        {{
            double result = seed + {0};
            for (int i = 0; i < n; i++)
            {{
                result = Math.Sin(result) * Math.Cos(result * {0}.{1});
                result = Math.Sqrt(Math.Abs(result) + 1);
                result += Math.Log(Math.Abs(result) + 1) * {2}.0;
            }}
            return result;
        }}
"#,
                i,
                (i + 1) % 10,
                i + 1
            ));
        }

        // Add utility methods
        math_content.push_str(r#"
        public double[] GenerateSequence(int length)
        {
            var sequence = new double[length];
            for (int i = 0; i < length; i++)
            {
                sequence[i] = Math.Sin(i * 0.1) * Math.Cos(i * 0.2);
            }
            return sequence;
        }

        public double ComputeStandardDeviation(double[] values)
        {
            if (values.Length == 0) return 0;

            double sum = 0;
            foreach (var v in values) sum += v;
            double mean = sum / values.Length;

            double sumSquares = 0;
            foreach (var v in values)
            {
                double diff = v - mean;
                sumSquares += diff * diff;
            }

            return Math.Sqrt(sumSquares / values.Length);
        }

        public Matrix MultiplyMatrices(Matrix a, Matrix b)
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
                    {
                        sum += a.Get(i, k) * b.Get(k, j);
                    }
                    result.Set(i, j, sum);
                }
            }

            return result;
        }
    }

    public class Matrix
    {
        private double[,] _data;
        public int Rows { get; private set; }
        public int Cols { get; private set; }

        public Matrix(int rows, int cols)
        {
            Rows = rows;
            Cols = cols;
            _data = new double[rows, cols];
        }

        public double Get(int row, int col) => _data[row, col];
        public void Set(int row, int col, double value) => _data[row, col] = value;

        public void Fill(double value)
        {
            for (int i = 0; i < Rows; i++)
            {
                for (int j = 0; j < Cols; j++)
                {
                    _data[i, j] = value;
                }
            }
        }
    }
}
"#);

        let math_path = self.test_dir.join("MathCalculator.cs");
        fs::write(&math_path, math_content)?;
        source_files.push(math_path);

        // Add more utility classes
        progress.update(0.25, "Generating utility classes...");

        let utils_content = r#"using System;
using System.Collections.Generic;
using System.Text;

namespace BenchmarkApp
{
    public static class StringUtils
    {
        public static string Reverse(string input)
        {
            if (string.IsNullOrEmpty(input)) return input;

            char[] chars = input.ToCharArray();
            Array.Reverse(chars);
            return new string(chars);
        }

        public static string ToCamelCase(string input)
        {
            if (string.IsNullOrEmpty(input)) return input;

            var sb = new StringBuilder();
            bool capitalizeNext = false;

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
                    sb.Append(char.ToLower(c));
                }
            }

            return sb.ToString();
        }

        public static int CountWords(string input)
        {
            if (string.IsNullOrWhiteSpace(input)) return 0;

            int count = 0;
            bool inWord = false;

            foreach (char c in input)
            {
                if (char.IsWhiteSpace(c))
                {
                    inWord = false;
                }
                else if (!inWord)
                {
                    inWord = true;
                    count++;
                }
            }

            return count;
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
                T temp = result[k];
                result[k] = result[n];
                result[n] = temp;
            }

            return result;
        }

        public static T[] Resize<T>(T[] array, int newSize)
        {
            var result = new T[newSize];
            int copyLength = Math.Min(array.Length, newSize);

            for (int i = 0; i < copyLength; i++)
            {
                result[i] = array[i];
            }

            return result;
        }

        public static Dictionary<K, V> Merge<K, V>(Dictionary<K, V> first, Dictionary<K, V> second)
        {
            var result = new Dictionary<K, V>(first);

            foreach (var kvp in second)
            {
                result[kvp.Key] = kvp.Value;
            }

            return result;
        }
    }

    public class Logger
    {
        private List<string> _messages = new List<string>();
        private string _name;

        public Logger(string name)
        {
            _name = name;
        }

        public void Info(string message)
        {
            Log("INFO", message);
        }

        public void Warning(string message)
        {
            Log("WARN", message);
        }

        public void Error(string message)
        {
            Log("ERROR", message);
        }

        private void Log(string level, string message)
        {
            string formatted = $"[{DateTime.Now:yyyy-MM-dd HH:mm:ss}] [{level}] [{_name}] {message}";
            _messages.Add(formatted);
        }

        public List<string> GetMessages()
        {
            return new List<string>(_messages);
        }

        public void Clear()
        {
            _messages.Clear();
        }
    }
}
"#;
        let utils_path = self.test_dir.join("Utils.cs");
        fs::write(&utils_path, utils_content)?;
        source_files.push(utils_path);

        Ok(source_files)
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
        "Native Compiler"
    }

    fn description(&self) -> &'static str {
        "Multi-file C# compilation with csc.exe"
    }

    fn category(&self) -> Category {
        Category::BuildPerformance
    }

    fn estimated_duration_secs(&self) -> u32 {
        30
    }

    fn run(&self, progress: &dyn ProgressCallback, config: &BenchmarkConfig) -> Result<TestResult> {
        // Find csc.exe
        let csc_path = Self::find_csc()
            .ok_or_else(|| anyhow::anyhow!("C# compiler (csc.exe) not found. .NET Framework may not be installed."))?;

        progress.update(0.02, &format!("Found compiler: {}", csc_path.display()));

        // Create test files
        let source_files = self.create_test_files(progress)?;

        let output_exe = self.test_dir.join("benchmark.exe");
        let mut compile_times: Vec<f64> = Vec::new();

        // Build source files list for command
        let source_args: Vec<String> = source_files
            .iter()
            .map(|p| p.to_string_lossy().to_string())
            .collect();

        progress.update(0.3, "Running compilation benchmarks...");

        // Run multiple compilation iterations
        let iterations = config.iterations as usize;
        for i in 0..iterations {
            if progress.is_cancelled() {
                self.cleanup();
                return Err(anyhow::anyhow!("Cancelled"));
            }

            // Remove previous output to force full recompile
            let _ = fs::remove_file(&output_exe);

            let timer = Timer::new();

            let mut cmd = Command::new(&csc_path);
            cmd.arg("/nologo")
                .arg("/optimize+")
                .arg(&format!("/out:{}", output_exe.display()));

            for src in &source_args {
                cmd.arg(src);
            }

            cmd.hidden();
            let output = cmd.output()?;
            let elapsed = timer.elapsed_secs();

            if !output.status.success() {
                self.cleanup();
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(anyhow::anyhow!("Compilation failed: {}", stderr));
            }

            compile_times.push(elapsed);

            progress.update(
                0.3 + (i as f32 / iterations as f32) * 0.6,
                &format!("Compilation {}/{} ({:.2}s)...", i + 1, iterations, elapsed),
            );
        }

        // Cleanup
        progress.update(0.95, "Cleaning up...");
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
                "{} (avg: {:.2}s, {} files)",
                self.description(),
                avg_time,
                source_files.len()
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
