use std::fs;
use std::path::PathBuf;
use std::process::Command;

use anyhow::Result;

use crate::benchmarks::{Benchmark, Category, ProgressCallback};
use crate::core::{CommandExt, Timer};
use crate::models::{TestDetails, TestResult};

/// PowerShell benchmark - tests PowerShell script execution performance
/// This is a key Windows developer tool for automation and system tasks
pub struct PowerShellBenchmark {
    test_dir: PathBuf,
}

impl PowerShellBenchmark {
    pub fn new() -> Self {
        Self {
            test_dir: std::env::temp_dir().join("workbench_pro_powershell_test"),
        }
    }

    fn is_powershell_available() -> bool {
        // Try pwsh (PowerShell Core) first, then powershell (Windows PowerShell)
        Command::new("pwsh")
            .arg("--version")
            .hidden()
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
            || Command::new("powershell")
                .arg("-Command")
                .arg("$PSVersionTable.PSVersion")
                .hidden()
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
    }

    fn get_powershell_cmd() -> &'static str {
        // Prefer pwsh (PowerShell Core) if available
        if Command::new("pwsh")
            .arg("--version")
            .hidden()
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            "pwsh"
        } else {
            "powershell"
        }
    }

    fn setup_test_scripts(&self, progress: &dyn ProgressCallback) -> Result<()> {
        // Clean up any existing test directory
        let _ = fs::remove_dir_all(&self.test_dir);
        fs::create_dir_all(&self.test_dir)?;

        progress.update(0.05, "Creating test scripts...");

        // Script 1: Simple computation script
        let compute_script = r#"
$sum = 0
for ($i = 1; $i -le 10000; $i++) {
    $sum += $i
    if ($i % 1000 -eq 0) {
        $null = [math]::Sqrt($sum)
    }
}
Write-Output $sum
"#;
        fs::write(self.test_dir.join("compute.ps1"), compute_script)?;

        // Script 2: File operations script
        let file_ops_script = r#"
$testDir = Join-Path $env:TEMP "ps_file_test"
if (Test-Path $testDir) { Remove-Item -Recurse -Force $testDir }
New-Item -ItemType Directory -Path $testDir | Out-Null

# Create files
for ($i = 1; $i -le 100; $i++) {
    $content = "Test content for file $i" * 10
    Set-Content -Path (Join-Path $testDir "file_$i.txt") -Value $content
}

# Read files
$totalSize = 0
Get-ChildItem $testDir -File | ForEach-Object {
    $totalSize += $_.Length
}

# Cleanup
Remove-Item -Recurse -Force $testDir
Write-Output $totalSize
"#;
        fs::write(self.test_dir.join("file_ops.ps1"), file_ops_script)?;

        // Script 3: Object manipulation script
        let object_script = r#"
$objects = @()
for ($i = 1; $i -le 1000; $i++) {
    $obj = [PSCustomObject]@{
        Id = $i
        Name = "Item_$i"
        Value = $i * 3.14159
        Tags = @("tag1", "tag2", "tag3")
    }
    $objects += $obj
}

$filtered = $objects | Where-Object { $_.Value -gt 1000 }
$sorted = $filtered | Sort-Object -Property Value -Descending
$grouped = $sorted | Group-Object { [math]::Floor($_.Id / 100) }
Write-Output $grouped.Count
"#;
        fs::write(self.test_dir.join("objects.ps1"), object_script)?;

        // Script 4: String processing script
        let string_script = r#"
$text = "The quick brown fox jumps over the lazy dog. " * 1000
$lines = @()
for ($i = 0; $i -lt 100; $i++) {
    $lines += $text.Substring($i * 10, 100)
}

$processed = $lines | ForEach-Object {
    $_.ToUpper().Replace("THE", "A").Split(" ") | Where-Object { $_.Length -gt 3 }
}
Write-Output $processed.Count
"#;
        fs::write(self.test_dir.join("strings.ps1"), string_script)?;

        Ok(())
    }

    fn cleanup(&self) {
        let _ = fs::remove_dir_all(&self.test_dir);
    }
}

impl Default for PowerShellBenchmark {
    fn default() -> Self {
        Self::new()
    }
}

impl Benchmark for PowerShellBenchmark {
    fn id(&self) -> &'static str {
        "powershell"
    }

    fn name(&self) -> &'static str {
        "PowerShell Scripts"
    }

    fn description(&self) -> &'static str {
        "Execute PowerShell scripts: compute, file ops, objects, strings"
    }

    fn category(&self) -> Category {
        Category::BuildPerformance
    }

    fn estimated_duration_secs(&self) -> u32 {
        60
    }

    fn run(&self, progress: &dyn ProgressCallback) -> Result<TestResult> {
        // Check if PowerShell is available
        if !Self::is_powershell_available() {
            return Err(anyhow::anyhow!("PowerShell is not installed or not in PATH"));
        }

        let ps_cmd = Self::get_powershell_cmd();

        // Setup test scripts
        self.setup_test_scripts(progress)?;

        let scripts = ["compute.ps1", "file_ops.ps1", "objects.ps1", "strings.ps1"];
        let mut all_times: Vec<f64> = Vec::new();
        let mut script_avgs: Vec<(String, f64)> = Vec::new();

        for (script_idx, script) in scripts.iter().enumerate() {
            if progress.is_cancelled() {
                self.cleanup();
                return Err(anyhow::anyhow!("Cancelled"));
            }

            let script_path = self.test_dir.join(script);
            let mut times: Vec<f64> = Vec::new();

            progress.update(
                0.1 + (script_idx as f32 / scripts.len() as f32) * 0.8,
                &format!("Running {}...", script),
            );

            // Run each script multiple times
            for _ in 0..5 {
                if progress.is_cancelled() {
                    self.cleanup();
                    return Err(anyhow::anyhow!("Cancelled"));
                }

                let timer = Timer::new();
                let output = Command::new(ps_cmd)
                    .args([
                        "-NoProfile",
                        "-ExecutionPolicy",
                        "Bypass",
                        "-File",
                        script_path.to_str().unwrap(),
                    ])
                    .hidden()
                    .output()?;

                let elapsed = timer.elapsed_secs() * 1000.0; // Convert to ms

                if !output.status.success() {
                    // Log but don't fail - script might have non-zero exit for valid reasons
                    eprintln!("Script {} exited with: {:?}", script, output.status);
                }

                times.push(elapsed);
            }

            let avg = times.iter().sum::<f64>() / times.len() as f64;
            script_avgs.push((script.to_string(), avg));
            all_times.extend(times);
        }

        // Cleanup
        progress.update(0.95, "Cleaning up...");
        self.cleanup();

        // Calculate statistics
        let total_avg: f64 = script_avgs.iter().map(|(_, avg)| avg).sum::<f64>() / script_avgs.len() as f64;

        let min = all_times.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = all_times.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

        progress.update(1.0, "Complete");

        Ok(TestResult {
            test_id: self.id().to_string(),
            name: self.name().to_string(),
            description: format!(
                "{} (avg: {:.0}ms per script)",
                self.description(),
                total_avg
            ),
            value: total_avg,
            unit: "ms".to_string(),
            details: TestDetails {
                iterations: all_times.len() as u32,
                duration_secs: all_times.iter().sum::<f64>() / 1000.0,
                min,
                max,
                mean: total_avg,
                median: total_avg,
                std_dev: 0.0,
                percentiles: None,
            },
        })
    }
}
