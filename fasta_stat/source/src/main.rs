mod cli;
mod file_reader;
mod base_statistics;
mod n50_statistics;
mod gap_statistics;
mod file_writer;

use base_statistics::{BaseStatistics, OverallStatistics};
use file_reader::FastaReader;
use n50_statistics::calculate_n50_for_sequences;
use gap_statistics::find_gaps;
use file_writer::{write_base_statistics, write_n50_statistics, write_gap_statistics, write_basic_statistics};
use std::time::Instant;
use chrono::Local;

fn main() {
    let start_time = Instant::now();
    let start_datetime = Local::now();
    let args = cli::parse_args();
    let reader = FastaReader::from_file(&args.fasta_file, args.chr_prefix.as_deref()).expect("Error reading FASTA");

    println!("Program started at: {}", start_datetime.format("%Y-%m-%d %H:%M:%S"));

    // 计算每条序列的统计信息
    let stats: Vec<BaseStatistics> = reader
        .sequences
        .iter()
        .map(|(id, seq)| BaseStatistics::from_sequence(id, seq))
        .collect::<Result<Vec<_>, _>>().expect("REASON");

    // 生成总体统计
    let overall = OverallStatistics::from_all_statistics(&stats);

    // 计算 N50 等统计信息
    let (contig_n_stats, scaffold_n_stats) = calculate_n50_for_sequences(&reader.sequences);

    // 识别 gap 信息
    let mut all_gaps = Vec::new();
    for (id, seq) in &reader.sequences {
        all_gaps.extend(find_gaps(id, seq));
    }

    // 输出文件路径
    let base_stats_file = args.outdir.join(format!("{}_BASE.txt", args.prefix));
    let n50_stats_file = args.outdir.join(format!("{}_N50.txt", args.prefix));
    let gap_stats_file = args.outdir.join(format!("{}_GAP.txt", args.prefix));
    let basic_stats_file = args.outdir.join(format!("{}_Basic.txt", args.prefix));

    // 写入统计结果
    write_base_statistics(base_stats_file, &stats, &overall).expect("Error writing BASE statistics");
    write_n50_statistics(n50_stats_file, &contig_n_stats, &scaffold_n_stats).expect("Error writing N50 statistics");
    write_gap_statistics(gap_stats_file, &all_gaps).expect("Error writing GAP statistics");
    write_basic_statistics(basic_stats_file, &stats, &overall, &contig_n_stats, &scaffold_n_stats, all_gaps.len())
        .expect("Error writing Basic statistics");

    let duration = start_time.elapsed();
    let end_datetime = Local::now();
    println!("Program completed at: {}", end_datetime.format("%Y-%m-%d %H:%M:%S"));
    println!("Total execution time: {:.2?}", duration);
}