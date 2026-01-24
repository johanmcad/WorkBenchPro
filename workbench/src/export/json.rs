use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use anyhow::Result;

use crate::models::BenchmarkRun;

pub struct JsonExporter;

impl JsonExporter {
    pub fn export(run: &BenchmarkRun, path: &Path) -> Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, run)?;
        Ok(())
    }

    pub fn import(path: &Path) -> Result<BenchmarkRun> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let run: BenchmarkRun = serde_json::from_reader(reader)?;
        Ok(run)
    }
}
