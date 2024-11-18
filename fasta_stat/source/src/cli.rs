use clap::{Arg, Command};
use std::path::PathBuf;

#[derive(Debug)]
pub struct CliArgs {
    pub fasta_file: PathBuf,
    pub outdir: PathBuf,
    pub chr_prefix: Option<String>,
    pub prefix: String,
}

pub fn parse_args() -> CliArgs {
    let matches = Command::new("Genome stats")
        .version("1.0.0")
        .author("WangZhSi <wang_zsi@outlook.com>")
        .about("A tool for genome statistics")
        .arg(
            Arg::new("fasta")
                .short('f')
                .long("fasta")
                .required(true)
                .value_name("FILE")
                .num_args(1)
                .help("Input fasta file"),
        )
        .arg(
            Arg::new("outdir")
                .short('o')
                .long("outdir")
                .default_value("./")
                .value_name("DIR")
                .num_args(1)
                .help("Output directory, default: current directory"),
        )
        .arg(
            Arg::new("chr_prefix")
            .long("chr_prefix")
            .value_name("PREFIX")
            .num_args(1)
            .help("Prefix of chromosome sequence to include (e.g., chr)"),
        )
        .arg(
            Arg::new("prefix")
            .long("prefix")
            .value_name("PREFIX")
            .default_value("statistics")
            .num_args(1)
            .help("Prefix of output files, default: statistics"),
        )
        .get_matches();

    let fasta_file = PathBuf::from(matches.get_one::<String>("fasta").expect("FASTA file is required"));
    let outdir = PathBuf::from(matches.get_one::<String>("outdir").unwrap_or(&"./".to_string()));
    let chr_prefix = matches.get_one::<String>("chr_prefix").cloned();
    let prefix = matches.get_one::<String>("prefix").cloned().unwrap_or_else(|| "statistics".to_string());

    CliArgs {
        fasta_file,
        outdir,
        chr_prefix,
        prefix,
    }
}
