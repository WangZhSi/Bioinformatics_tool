Split FASTA file, by Ns or input bed;

Usage: fa_split [COMMAND]

Commands:
```bash
  splitN    Splits sequences based on N positions
  splitBed  Splits sequences based on BED file
  help      Print this message or the help of the given subcommand(s)
```

Options:
```bash
  -h, --help     Print help
  -V, --version  Print version
```

# splitN

Splits sequences based on N positions

Usage: `fa_split splitN --fasta <FILE> --output-stats <FILE> --output-positions <FILE> --output-seqs <FILE>`

Options:
```bash
  -f, --fasta <FILE>             Path to the input FASTA file
  -s, --output-stats <FILE>      Path to the output statistics file
  -p, --output-positions <FILE>  Path to the output split positions log
  -q, --output-seqs <FILE>       Path to the output sequences file
  -h, --help                     Print help
```

# splitBed

Splits sequences based on BED file

Usage: `fa_split splitBed --fasta <FILE> --bed <FILE> --output-seqs <FILE>`

Options:
```bash
  -f, --fasta <FILE>        Path to the input FASTA file
  -b, --bed <FILE>          Path to the BED file(0 base, [close, open) ); default for 3 column; 4th column will used as seq id;
  -q, --output-seqs <FILE>  Path to the output sequences file
  -h, --help                Print help
```