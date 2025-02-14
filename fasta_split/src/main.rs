mod fasta;
mod stats;
mod split;
mod split_bed;
mod output;
mod bed;

use clap::{Arg, Command};
use std::process;

fn main() {
    let matches = Command::new("FASTA Split Processor")
        .version("1.0.0")
        .author("WangZhSi")
        .about("\nSplit FASTA file, by Ns or input bed;")
        .subcommand(
            Command::new("splitN")
                .about("Splits sequences based on N positions")
                .arg(
                    Arg::new("fasta")
                        .short('f')
                        .long("fasta")
                        .value_name("FILE")
                        .help("Path to the input FASTA file")
                        .required(true),
                )
                .arg(
                    Arg::new("output_stats")
                        .short('s')
                        .long("output-stats")
                        .value_name("FILE")
                        .help("Path to the output statistics file")
                        .required(true),
                )
                .arg(
                    Arg::new("output_positions")
                        .short('p')
                        .long("output-positions")
                        .value_name("FILE")
                        .help("Path to the output split positions log")
                        .required(true),
                )
                .arg(
                    Arg::new("output_seqs")
                        .short('q')
                        .long("output-seqs")
                        .value_name("FILE")
                        .help("Path to the output sequences file")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("splitBed")
                .about("Splits sequences based on BED file")
                .arg(
                    Arg::new("fasta")
                        .short('f')
                        .long("fasta")
                        .value_name("FILE")
                        .help("Path to the input FASTA file")
                        .required(true),
                )
                .arg(
                    Arg::new("bed")
                        .short('b')
                        .long("bed")
                        .value_name("FILE")
                        .help("Path to the BED file(0 base, [close, open) ); default for 3 column; 4th column will used as seq id;")
                        .required(true),
                )
                .arg(
                    Arg::new("output_seqs")
                        .short('q')
                        .long("output-seqs")
                        .value_name("FILE")
                        .help("Path to the output sequences file")
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("splitN", sub_m)) => {
            let input_fasta = sub_m.get_one::<String>("fasta").expect("required").clone();
            let output_stats = sub_m.get_one::<String>("output_stats").expect("required").clone();
            let output_positions = sub_m.get_one::<String>("output_positions").expect("required").clone();
            let output_seqs = sub_m.get_one::<String>("output_seqs").expect("required").clone();

            // 调用基于N的切割功能
            run_split_n(&input_fasta, &output_stats, &output_positions, &output_seqs);
        },
        Some(("splitBed", sub_m)) => {
            let input_fasta = sub_m.get_one::<String>("fasta").expect("required").clone();
            let bed_file = sub_m.get_one::<String>("bed").expect("required").clone();
            let output_seqs = sub_m.get_one::<String>("output_seqs").expect("required").clone();

            // 调用基于BED文件的切割功能
            run_split_bed(&input_fasta, &bed_file, &output_seqs);
        },
        _ => {
            eprintln!("Invalid subcommand. Use 'splitN' or 'splitBed'.");
            process::exit(1);
        }
    }
}

fn run_split_n(input_fasta: &str, output_stats: &str, output_positions: &str, output_seqs: &str) {
    // 读取FASTA文件
    let sequences = match fasta::read_fasta(input_fasta) {
        Ok(seq) => seq,
        Err(e) => {
            eprintln!("Error reading FASTA file: {}", e);
            process::exit(1);
        }
    };

    // 统计序列信息
    let stats = stats::calculate_stats(&sequences);

    // 切割序列
    let (new_sequences, split_positions) = split::split_sequences(&sequences);

    // 输出统计结果
    if let Err(e) = output::write_stats(&stats, output_stats) {
        eprintln!("Error writing stats file: {}", e);
        process::exit(1);
    }

    // 输出切割位置记录
    if let Err(e) = output::write_split_positions(&split_positions, output_positions) {
        eprintln!("Error writing split positions file: {}", e);
        process::exit(1);
    }

    // 输出新的FASTA序列
    if let Err(e) = output::write_new_sequences(&new_sequences, output_seqs) {
        eprintln!("Error writing new sequences file: {}", e);
        process::exit(1);
    }

    println!("Processing based on N positions completed successfully.");
}

fn run_split_bed(input_fasta: &str, bed_file: &str, output_seqs: &str) {
    use crate::bed::read_bed;
    use crate::split_bed::{split_by_bed};
    use crate::output::write_new_sequences_bed;

    // 读取FASTA文件
    let sequences = match fasta::read_fasta(input_fasta) {
        Ok(seq) => seq,
        Err(e) => {
            eprintln!("Error reading FASTA file: {}", e);
            process::exit(1);
        }
    };

    // 读取BED文件
    let bed_records = match read_bed(bed_file) {
        Ok(records) => records,
        Err(e) => {
            eprintln!("Error reading BED file: {}", e);
            process::exit(1);
        }
    };

    // 根据BED文件切割序列
    let (split_results, errors) = split_by_bed(&sequences, &bed_records);

    // 输出错误信息
    for error in errors {
        eprintln!("{}", error);
    }

    // 输出新的FASTA序列
    if let Err(e) = write_new_sequences_bed(&split_results, output_seqs) {
        eprintln!("Error writing new sequences file: {}", e);
        process::exit(1);
    }

    println!("Processing based on BED file completed successfully.");
}
