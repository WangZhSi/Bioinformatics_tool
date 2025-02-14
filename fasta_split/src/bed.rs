use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// 代表BED记录的结构体
pub struct BedRecord {
    pub chrom: String,     // 染色体或序列ID
    pub start: i64,        // 起始位置（左闭）
    pub end: i64,          // 终止位置（右开）
    pub name: Option<String>, // 可选的名称/ID
}

/// 从指定文件路径读取并解析BED文件
/// 返回一个包含所有BED记录的向量
pub fn read_bed(file_path: &str) -> Result<Vec<BedRecord>, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut records = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split('\t').collect();
        if fields.len() < 3 || fields.len() > 4 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid BED format"));
        }
        let record = BedRecord {
            chrom: fields[0].to_string(),
            start: fields[1].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid start position"))?,
            end: fields[2].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid end position"))?,
            name: if fields.len() == 4 { Some(fields[3].to_string()) } else { None },
        };
        records.push(record);
    }

    Ok(records)
}


