use csv::{WriterBuilder};
use crate::fim::models::{CsvLargeFiles, CsvScanStats, ExtensionCount, ScanStats};
use anyhow::Result;
use std::fs;

pub fn export_csv(files:&ScanStats) -> Result<()> {
    let mut extensions_writer = WriterBuilder::new().from_writer(vec![]);

    for (ext,count) in files.extension_count.iter() {
        extensions_writer.serialize(ExtensionCount {
            extension:ext,
            count:*count
        })?;
    }

    let data = String::from_utf8(extensions_writer.into_inner()?)?;
    fs::write("../../../extension_stats.csv", data).expect("Unable to write extension stats csv file");

    let mut largest_writer = WriterBuilder::new().from_writer(vec![]);

    for file in files.largest_files.iter() {
        largest_writer.serialize(CsvLargeFiles{
            path:file.path.clone(),
            size:file.size
        })?;
    }

    let large_files_data = String::from_utf8(largest_writer.into_inner()?)?;

    fs::write("../../../largest_files.csv", large_files_data).expect("Unable to write largest_files csv file");


    let mut other_stats = WriterBuilder::new().from_writer(vec![]);

    other_stats.serialize(CsvScanStats {
        key:"total_files".into(),
        value:files.total_files
    })?;
    other_stats.serialize(CsvScanStats {
        key:"total_dirs".into(),
        value:files.total_dirs
    })?;

    other_stats.serialize(CsvScanStats {
        key:"total_size".into(),
        value:files.total_size
    })?;

    other_stats.serialize(CsvScanStats {
        key:"average_size".into(),
        value:files.average_size
    })?;

    let stats_data = String::from_utf8(other_stats.into_inner()?)?;

    fs::write("../../../scan_stats.csv", stats_data).expect("Unable to write stats_data csv file");

    Ok(())
}