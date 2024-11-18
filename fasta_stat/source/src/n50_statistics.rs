//use std::cmp::Ordering;

pub struct NStatistics {
    pub n50: usize,
    pub n60: usize,
    pub n70: usize,
    pub n80: usize,
    pub n90: usize,
}

impl Default for NStatistics {
    fn default() -> Self {
        NStatistics {
            n50: 0,
            n60: 0,
            n70: 0,
            n80: 0,
            n90: 0,
        }
    }
}

impl NStatistics {
    pub fn from_lengths(lengths: &[usize]) -> Result<Self, &'static str> {
        if lengths.is_empty() {
            return Err("Input lengths cannot be empty");
        }

        let mut sorted_lengths = lengths.to_vec();
        sorted_lengths.sort_unstable_by(|a, b| b.cmp(a)); // 按降序排序
        let total_length: usize = sorted_lengths.iter().sum();

        let mut n_statistics = [0; 5];
        let thresholds = [0.50, 0.60, 0.70, 0.80, 0.90];

        for (i, &threshold) in thresholds.iter().enumerate() {
            n_statistics[i] = Self::calculate_n_statistic(&sorted_lengths, total_length, threshold);
        }

        Ok(NStatistics { 
            n50: n_statistics[0], 
            n60: n_statistics[1], 
            n70: n_statistics[2], 
            n80: n_statistics[3], 
            n90: n_statistics[4] 
        })
    }

    /// 计算指定比例的 N 值 (例如 N50, N60 等)
    fn calculate_n_statistic(lengths: &[usize], total_length: usize, threshold: f64) -> usize {
        let threshold_length = (total_length as f64 * threshold).ceil() as usize;
        let mut cumulative_length = 0;

        for &length in lengths {
            cumulative_length += length;
            if cumulative_length >= threshold_length {
                return length;
            }
        }

        0 // 当数据不充分时返回0
    }
}

/// 根据序列计算 N50 等指标
pub fn calculate_n50_for_sequences(sequences: &[(String, String)]) -> (NStatistics, NStatistics) {
    if sequences.is_empty() {
        return (NStatistics::default(), NStatistics::default());
    }

    let mut scaffold_lengths = Vec::new();
    let mut contig_lengths = Vec::new();

    for (_id, sequence) in sequences {
        if sequence.is_empty() {
            continue;
        }

        scaffold_lengths.push(sequence.len());

        let mut start = 0;
        for (i, c) in sequence.chars().enumerate() {
            if c == 'N' {
                if i > start {
                    contig_lengths.push(i - start);
                }
                start = i + 1;
            }
        }
        if start < sequence.len() {
            contig_lengths.push(sequence.len() - start);
        }
    }

    let contig_n_stats = NStatistics::from_lengths(&contig_lengths).unwrap_or_default();
    let scaffold_n_stats = NStatistics::from_lengths(&scaffold_lengths).unwrap_or_default();

    (contig_n_stats, scaffold_n_stats)
}