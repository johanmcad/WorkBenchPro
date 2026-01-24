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
        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>WorkBench Report - {machine_name}</title>
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
        .score-card {{
            background: white;
            border-radius: 8px;
            padding: 20px;
            margin: 10px;
            box-shadow: 0 1px 3px rgba(0,0,0,0.1);
            display: inline-block;
            min-width: 200px;
            text-align: center;
        }}
        .score {{ font-size: 48px; font-weight: bold; }}
        .rating {{
            display: inline-block;
            padding: 4px 12px;
            border-radius: 4px;
            font-weight: 500;
        }}
        .excellent {{ background: #d1fae5; color: #065f46; }}
        .good {{ background: #dbeafe; color: #1e40af; }}
        .acceptable {{ background: #fef3c7; color: #92400e; }}
        .poor {{ background: #fee2e2; color: #991b1b; }}
        .inadequate {{ background: #fecaca; color: #7f1d1d; }}
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
    </style>
</head>
<body>
    <h1>WorkBench Report</h1>
    <p>Machine: {machine_name} | Date: {timestamp}</p>

    <div class="score-card">
        <div class="score">{overall_score}</div>
        <div>/ {overall_max}</div>
        <div class="rating {rating_class}">{rating}</div>
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

    <h2>Results</h2>
    <p>Full results exported to JSON for detailed analysis.</p>
</body>
</html>"#,
            machine_name = run.machine_name,
            timestamp = run.timestamp.format("%Y-%m-%d %H:%M:%S"),
            overall_score = run.scores.overall,
            overall_max = run.scores.overall_max,
            rating = run.scores.rating.label(),
            rating_class = run.scores.rating.label().to_lowercase(),
            cpu_name = run.system_info.cpu.name,
            cores = run.system_info.cpu.cores,
            threads = run.system_info.cpu.threads,
            memory_gb = run.system_info.memory.total_gb(),
            os_name = run.system_info.os.name,
            os_version = run.system_info.os.version,
        )
    }
}
