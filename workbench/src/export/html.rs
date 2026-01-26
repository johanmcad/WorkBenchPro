use std::fs::File;
use std::io::Write;
use std::path::Path;

use anyhow::Result;

use crate::models::BenchmarkRun;

pub struct HtmlExporter;

impl HtmlExporter {
    pub fn export(run: &BenchmarkRun, path: &Path) -> Result<()> {
        let html = Self::generate_html(run);
        let mut file = File::create(path)?;
        file.write_all(html.as_bytes())?;
        Ok(())
    }

    fn generate_html(run: &BenchmarkRun) -> String {
        // Count total tests
        let total_tests = run.results.project_operations.len()
            + run.results.build_performance.len()
            + run.results.responsiveness.len();

        // Generate results table rows
        let mut results_html = String::new();

        for result in &run.results.project_operations {
            results_html.push_str(&format!(
                "<tr><td>Project Operations</td><td>{}</td><td>{:.2} {}</td></tr>\n",
                result.name, result.value, result.unit
            ));
        }
        for result in &run.results.build_performance {
            results_html.push_str(&format!(
                "<tr><td>Build Performance</td><td>{}</td><td>{:.2} {}</td></tr>\n",
                result.name, result.value, result.unit
            ));
        }
        for result in &run.results.responsiveness {
            results_html.push_str(&format!(
                "<tr><td>Responsiveness</td><td>{}</td><td>{:.2} {}</td></tr>\n",
                result.name, result.value, result.unit
            ));
        }

        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>WorkBench-Pro Report - {machine_name}</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
            background: #f8fafc;
            color: #1e293b;
        }}
        h1 {{ color: #0f3460; }}
        .summary-card {{
            background: white;
            border-radius: 8px;
            padding: 20px;
            margin: 10px;
            box-shadow: 0 1px 3px rgba(0,0,0,0.1);
            display: inline-block;
            min-width: 200px;
            text-align: center;
        }}
        .test-count {{ font-size: 48px; font-weight: bold; color: #0f3460; }}
        .system-info {{
            background: white;
            border-radius: 8px;
            padding: 20px;
            margin: 20px 0;
        }}
        table {{
            width: 100%;
            border-collapse: collapse;
        }}
        th, td {{
            padding: 8px 12px;
            text-align: left;
            border-bottom: 1px solid #e2e8f0;
        }}
        th {{
            background: #f1f5f9;
            font-weight: 600;
        }}
        .results-section {{
            background: white;
            border-radius: 8px;
            padding: 20px;
            margin: 20px 0;
        }}
    </style>
</head>
<body>
    <h1>WorkBench-Pro Report</h1>
    <p>Machine: {machine_name} | Date: {timestamp}</p>

    <div class="summary-card">
        <div class="test-count">{total_tests}</div>
        <div>Tests Completed</div>
    </div>

    <div class="system-info">
        <h2>System Information</h2>
        <table>
            <tr><th>CPU</th><td>{cpu_name}</td></tr>
            <tr><th>Cores/Threads</th><td>{cores}/{threads}</td></tr>
            <tr><th>Memory</th><td>{memory_gb:.1} GB</td></tr>
            <tr><th>OS</th><td>{os_name} {os_version}</td></tr>
        </table>
    </div>

    <div class="results-section">
        <h2>Benchmark Results</h2>
        <table>
            <thead>
                <tr>
                    <th>Category</th>
                    <th>Test</th>
                    <th>Value</th>
                </tr>
            </thead>
            <tbody>
                {results_html}
            </tbody>
        </table>
    </div>
</body>
</html>"#,
            machine_name = run.machine_name,
            timestamp = run.timestamp.format("%Y-%m-%d %H:%M:%S"),
            total_tests = total_tests,
            cpu_name = run.system_info.cpu.name,
            cores = run.system_info.cpu.cores,
            threads = run.system_info.cpu.threads,
            memory_gb = run.system_info.memory.total_gb(),
            os_name = run.system_info.os.name,
            os_version = run.system_info.os.version,
            results_html = results_html,
        )
    }
}
