
**Genome Statistics Tool**

A command-line tool for comprehensive genome sequence analysis and statistics. This program processes FASTA format genome files and generates detailed statistics, including base composition, N50 statistics, and gap information. Designed for bioinformatics workflows, the tool is efficient and customizable.

**Features**

• **Base Composition Analysis**: Calculates sequence length, counts of A, T, C, G, and N bases, and their proportions.

• **N50 Statistics**:

• Calculates N50, N60, N70, N80, and N90 for both _contigs_ and _scaffolds_.

• Considers scaffolds as full sequences and contigs as segments separated by N.

• **Gap Analysis**:

• Identifies gaps (continuous N regions) in sequences.

• Outputs gap information in BED format, including start and end positions and gap lengths.

• **Summary Statistics**:

• Provides an overview of genome sequences, including total length, GC content, sequence counts, and N50 metrics.

• **Custom Prefix Filtering**: Optionally filter sequences by a specific prefix (e.g., chr).

**Installation**

Download the precompiled binary or the source code.

**Option 1: Using Precompiled Binary**

1. Download the binary suitable for your platform (e.g., Linux, macOS, Windows).

2. Place the binary in a directory included in your PATH, or run it directly from its location.

**Option 2: Building from Source**

1. Download the source code.

2. Compile the program (rust environment required):

```bash
cd source && cargo build --release
```

**Arguments**

• -f, --fasta _(required)_: Path to the input FASTA file.

• -o, --outdir _(optional)_: Directory for output files. Default: current directory (./).

• --chr_prefix _(optional)_: Filter sequences by prefix (e.g., chr).

**Example**

```bash
fasta_stat -f example.fasta -o ./output --chr_prefix chr
```


**Outputs**

The tool generates the following files in the specified output directory:

1. statistics_BASE.txt: Detailed base composition statistics.

2. statistics_N50.txt: N50 statistics for contigs and scaffolds.

3. statistics_GAP.txt: Gap information in BED format.

4. statistics_Basic.txt: A summary of genome-wide statistics.

  

**Example Output**


_statistics_Basic.txt:_

Total Sequences:       1,234

Total Length:          12,345,678

Longest Sequence:      123,456

GC Content (%):        45.67

Non-N Sequences:       1,000

Total Gaps:            234

Contig N50:            5,678

Scaffold N50:          8,910


_statistics_GAP.txt:_

SequenceID   Start   End     Length    GapID

chr1         1000    1100    100       chr1-gap-1

chr2         2000    2100    100       chr2-gap-1


**Feedback During Runtime**


The program provides progress updates, including:

• Start and end time of the run.

• Total runtime at the end.

**License**

This project is licensed under the MIT License. See the LICENSE file for details.
