// src/gap_statistics.rs

//use std::fmt::Write;
use std::sync::Arc;
use std::fs::File;
use std::io::{BufWriter, Write};

#[derive(Debug)]
pub struct GapInfo {
    sequence_id: Arc<String>,
    start: usize,
    end: usize,
    length: usize,
    gap_id: String,
}

/// 识别序列中的 gap 区段
pub fn find_gaps(sequence_id: &str, sequence: &str) -> Vec<GapInfo> {
    let sequence_id = Arc::new(sequence_id.to_string());
    let mut gaps = Vec::new();
    let mut gap_start = None;
    let mut gap_counter = 1;

    if sequence.is_empty() {
        return gaps;
    }

    for (i, byte) in sequence.as_bytes().iter().enumerate() {
        if *byte == b'N' {
            if gap_start.is_none() {
                gap_start = Some(i);
            }
        } else if let Some(start) = gap_start {
            // gap 结束
            let end = i;
            let length = end - start;
            let gap_id = format!("{}-gap-{}", sequence_id, gap_counter);
            gaps.push(GapInfo {
                sequence_id: Arc::clone(&sequence_id),
                start,
                end,
                length,
                gap_id,
            });
            gap_counter += 1;
            gap_start = None; // 重置 gap_start
        }
    }

    // 若序列以 gap 结尾，需记录最后一个 gap
    if let Some(start) = gap_start {
        let end = sequence.len();
        let length = end - start;
        let gap_id = format!("{}-gap-{}", sequence_id, gap_counter);
        gaps.push(GapInfo {
            sequence_id: Arc::clone(&sequence_id),
            start,
            end,
            length,
            gap_id,
        });
    }

    gaps
}


/// 将所有序列中的 gap 信息写入文件
pub fn write_gaps_to_file(gaps: &[GapInfo], file_path: &str) -> std::io::Result<()> {

    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "SequenceID\tStart\tEnd\tLength\tGapID")?;
    for gap in gaps {
        writeln!(
            writer,
            "{}\t{}\t{}\t{}\t{}",
            gap.sequence_id, gap.start, gap.end, gap.length, gap.gap_id
        )?;
    }

    writer.flush()?;
    Ok(())
}
