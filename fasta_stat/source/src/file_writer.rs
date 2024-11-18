use crate::base_statistics::{BaseStatistics, OverallStatistics};
use crate::n50_statistics::NStatistics;
use crate::gap_statistics::{GapInfo, write_gaps_to_file};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::fmt::{self, Write};
use num_format::{Locale, ToFormattedString};

fn prepare_file_path<P: AsRef<Path>>(file_path: P) -> io::Result<PathBuf> {
    let path = file_path.as_ref();
    //if path.exists() {
    //    eprintln!("Warning: File {} already exists. It will be overwritten.", path.display());
    //}
    Ok(path.to_path_buf())
}

pub fn write_base_statistics<P: AsRef<Path>>(
    file_path: P,
    base_stats: &[BaseStatistics],
    overall_stats: &OverallStatistics,
) -> Result<(), fmt::Error> {
    let mut output = String::new();
    writeln!(output, "ID\tLength\tA\tT\tC\tG\tN\tG+C\tA%\tT%\tC%\tG%\tN%\tGC%")?;

    for stat in base_stats {
        writeln!(output, "{}", stat.to_string_with_thousands_sep())?;
    }
    writeln!(output, "{}", overall_stats.to_string_with_thousands_sep())?;
    
    fs::write(file_path, output).map_err(|_e| fmt::Error)
}

pub fn write_n50_statistics<P: AsRef<Path>>(
    file_path: P,
    contig_n_stats: &NStatistics,
    scaffold_n_stats: &NStatistics,
) -> Result<(), fmt::Error> {
    let mut output = String::new();
    writeln!(output, "Type\tN50\tN60\tN70\tN80\tN90")?;
    writeln!(
        output,
        "Contig\t{}\t{}\t{}\t{}\t{}",
        contig_n_stats.n50.to_formatted_string(&Locale::en),
        contig_n_stats.n60.to_formatted_string(&Locale::en),
        contig_n_stats.n70.to_formatted_string(&Locale::en),
        contig_n_stats.n80.to_formatted_string(&Locale::en),
        contig_n_stats.n90.to_formatted_string(&Locale::en)
    )?;
    writeln!(
        output,
        "Scaffold\t{}\t{}\t{}\t{}\t{}",
        scaffold_n_stats.n50.to_formatted_string(&Locale::en),
        scaffold_n_stats.n60.to_formatted_string(&Locale::en),
        scaffold_n_stats.n70.to_formatted_string(&Locale::en),
        scaffold_n_stats.n80.to_formatted_string(&Locale::en),
        scaffold_n_stats.n90.to_formatted_string(&Locale::en)
    )?;
    
    fs::write(file_path, output).map_err(|_e| fmt::Error)
}

pub fn write_gap_statistics<P: AsRef<Path>>(file_path: P, gaps: &[GapInfo]) -> io::Result<()> {
    let file_path = prepare_file_path(file_path)?;
    write_gaps_to_file(gaps, file_path.to_str().unwrap())
}

/// 生成 statistics_Basic.txt 文件，提供概览统计
pub fn write_basic_statistics<P: AsRef<Path>>(
    file_path: P,
    base_stats: &[BaseStatistics],
    overall_stats: &OverallStatistics,
    contig_n_stats: &NStatistics,
    scaffold_n_stats: &NStatistics,
    total_gaps: usize,
) -> Result<(), fmt::Error> {
    let mut output = String::new();
    let num_sequences = base_stats.len().to_formatted_string(&Locale::en);
    let non_n_sequences = base_stats.iter().filter(|s| s.count_n == 0).count().to_formatted_string(&Locale::en);
    let longest_sequence = base_stats.iter().map(|s| s.length).max().unwrap_or(0).to_formatted_string(&Locale::en);

    writeln!(output, "Total Sequences Number:\t{}", num_sequences)?;
    writeln!(output, "Total Length (bp):\t{}", overall_stats.total_length.to_formatted_string(&Locale::en))?;
    writeln!(output, "Longest Sequence Length (bp):\t{}", longest_sequence)?;
    writeln!(output, "GC Content(%):\t{:.2}%", overall_stats.total_gc_content)?;
    writeln!(output, "Non-N Sequences Number:\t{}", non_n_sequences)?;
    writeln!(output, "Total Gaps Number:\t{}", total_gaps.to_formatted_string(&Locale::en))?;
    writeln!(output, "Contig N50 (bp):\t{}", contig_n_stats.n50.to_formatted_string(&Locale::en))?;
    writeln!(output, "Scaffold N50 (bp):\t{}", scaffold_n_stats.n50.to_formatted_string(&Locale::en))?;

    fs::write(file_path, output).map_err(|_| fmt::Error)
}