use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub struct FastaReader {
    pub sequences: Vec<(String, String)>, // (chr, seq)
}

impl FastaReader {
    pub fn from_file<P: AsRef<Path>>(file_path: P, chr_prefix: Option<&str>) -> Result<Self, io::Error> {
        let file = File::open(file_path).map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to open file: {}", e)))?;
        let reader = BufReader::new(file);

        let mut sequences = Vec::new();
        let mut sequence_id = String::new();
        let mut sequence_data = String::with_capacity(1024); // 预分配空间

        for line in reader.lines() {
            let line = line.map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to read line: {}", e)))?;
            if line.starts_with(">") {
                if !sequence_id.is_empty() {
                    if Self::should_include(&sequence_id, chr_prefix) {
                        sequences.push((sequence_id.clone(), sequence_data.clone()));
                    }
                    sequence_data.clear();
                }
                sequence_id = line[1..].split_whitespace().next().unwrap_or("").to_string();
            } else {
                for c in line.chars() {
                    sequence_data.push(c.to_ascii_uppercase());
                }
            }
        }
        if !sequence_id.is_empty() {
            if Self::should_include(&sequence_id, chr_prefix) {
                sequences.push((sequence_id, sequence_data));
            }
        }
        Ok(FastaReader { sequences: sequences })
    }

    fn should_include(sequence_id: &str, chr_prefix: Option<&str>) -> bool {
        if let Some(prefix) = chr_prefix {
            sequence_id.starts_with(prefix)
        } else {
            true
        }
    }
}