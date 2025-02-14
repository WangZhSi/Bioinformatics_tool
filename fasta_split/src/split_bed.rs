use crate::fasta::FastaSequence;
use crate::bed::BedRecord;

/// 代表基于BED文件切割的结果的结构体
pub struct BedSplitResult {
    pub id: String,         // 原序列标识符
    pub start: i64,         // 切割起始位置
    pub end: i64,           // 切割终止位置
    pub new_id: String,     // 新序列标识符
    pub sequence: String,   // 新序列
}

/// 根据BED文件中的区间切割FASTA文件
/// 返回一个包含所有切割结果的向量和错误信息
pub fn split_by_bed(sequences: &Vec<FastaSequence>, bed_records: &Vec<BedRecord>) -> (Vec<BedSplitResult>, Vec<String>) {
    let mut results = Vec::new();
    let mut errors = Vec::new();
    
    for record in bed_records {
        if let Some(seq) = sequences.iter().find(|&s| s.id == record.chrom) {
            let start = record.start as usize;
            let end = record.end as usize;
            let new_id = record.name.clone().unwrap_or_else(|| format!("{}_{}", record.chrom, results.len() + 1));
            
            if start >= end || end > seq.sequence.len() {
                errors.push(format!("Invalid BED record for {}: start {}, end {}", record.chrom, record.start, record.end));
                continue;
            }

            let sequence = seq.sequence[start..end].to_string();
            results.push(BedSplitResult {
                id: record.chrom.clone(),
                start: record.start,
                end: record.end,
                new_id,
                sequence,
            });
        } else {
            errors.push(format!("Chromosome {} not found in FASTA sequences", record.chrom));
        }
    }

    (results, errors)
}
