use crate::stats::SequenceStats;
use crate::split::SplitPosition;
use crate::fasta::FastaSequence;
use crate::split_bed::BedSplitResult;
use std::fs::File;
use std::io::{self, Write};

/// 将序列统计结果写入指定文件
pub fn write_stats(stats: &Vec<SequenceStats>, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    writeln!(file, "id\ttotal_length\tn_count")?;
    for stat in stats {
        writeln!(file, "{}\t{}\t{}", stat.id, stat.total_length, stat.n_count)?;
    }
    Ok(())
}

/// 将切割位置记录写入指定文件
pub fn write_split_positions(positions: &Vec<SplitPosition>, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    writeln!(file, "id\tstart\tend\tnew_id")?;
    for position in positions {
        writeln!(file, "{}\t{}\t{}\t{}", position.id, position.start, position.end, position.new_id)?;
    }
    Ok(())
}

/// 将新的FASTA序列写入指定文件
pub fn write_new_sequences(sequences: &Vec<FastaSequence>, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    for seq in sequences {
        writeln!(file, ">{} {}", seq.id, seq.description)?;
        writeln!(file, "{}", seq.sequence)?;
    }
    Ok(())
}

/// 将基于BED文件切割的FASTA序列写入指定文件
pub fn write_new_sequences_bed(results: &Vec<BedSplitResult>, file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    for result in results {
        writeln!(file, ">{}", result.new_id)?;
        writeln!(file, "{}", result.sequence)?;
    }
    Ok(())
}