use std::collections::HashMap;
use num_format::{Locale, ToFormattedString};

pub struct BaseStatistics {
    pub sequence_id: String,
    pub length: usize,
    pub count_a: usize,
    pub count_c: usize,
    pub count_g: usize,
    pub count_t: usize,
    pub count_n: usize,
    pub gc_content: f64,
}

pub struct OverallStatistics {
    pub total_length: usize,
    pub total_count_a: usize,
    pub total_count_c: usize,
    pub total_count_g: usize,
    pub total_count_t: usize,
    pub total_count_n: usize,
    pub total_gc_content: f64,
}

impl BaseStatistics {
    pub fn from_sequence(sequence_id: &str, sequence: &str) -> Result<Self, &'static str> {
        if sequence.is_empty() || sequence_id.is_empty() {
            return Err("Sequence and sequence ID cannot be empty");
        }

        let mut counts = HashMap::new();
        for base in sequence.chars() {
            *counts.entry(base).or_insert(0) += 1;
        }

        let length = sequence.len();
        let count_a = *counts.get(&'A').unwrap_or(&0);
        let count_t = *counts.get(&'T').unwrap_or(&0);
        let count_c = *counts.get(&'C').unwrap_or(&0);
        let count_g = *counts.get(&'G').unwrap_or(&0);
        let count_n = *counts.get(&'N').unwrap_or(&0);
        let gc_content = if length > 0 {
            ((count_g + count_c) as f64 / length as f64) * 100.0
        } else {
            0.0
        };

        Ok(BaseStatistics {
            sequence_id: sequence_id.to_string(),
            length,
            count_a,
            count_t,
            count_c,
            count_g,
            count_n,
            gc_content,
        })
    }

    pub fn to_string_with_thousands_sep(&self) -> String {
        if self.length == 0 {
            return "0\t0\t0\t0\t0\t0\t0\t0\t0.00%\t0.00%\t0.00%\t0.00%\t0.00%\t0.00%".to_string();
        }

        let total_length = self.length as f64;
        let count_a_percent = (self.count_a as f64 / total_length) * 100.0;
        let count_t_percent = (self.count_t as f64 / total_length) * 100.0;
        let count_c_percent = (self.count_c as f64 / total_length) * 100.0;
        let count_g_percent = (self.count_g as f64 / total_length) * 100.0;
        let count_n_percent = (self.count_n as f64 / total_length) * 100.0;

        format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:.2}%\t{:.2}%\t{:.2}%\t{:.2}%\t{:.2}%\t{:.2}%",
            self.sequence_id,
            self.length.to_formatted_string(&Locale::en),
            self.count_a.to_formatted_string(&Locale::en),
            self.count_t.to_formatted_string(&Locale::en),
            self.count_c.to_formatted_string(&Locale::en),
            self.count_g.to_formatted_string(&Locale::en),
            self.count_n.to_formatted_string(&Locale::en),
            (self.count_g + self.count_c).to_formatted_string(&Locale::en),
            count_a_percent,
            count_t_percent,
            count_c_percent,
            count_g_percent,
            count_n_percent,
            self.gc_content
        )
    }
}

impl OverallStatistics {
    fn calculate_statistics(stats: &[BaseStatistics]) -> (usize, usize, usize, usize, usize, usize) {
        stats.iter().fold(
            (0, 0, 0, 0, 0, 0),
            |(length, count_a, count_c, count_g, count_t, count_n), s| {
                (
                    length + s.length,
                    count_a + s.count_a,
                    count_c + s.count_c,
                    count_g + s.count_g,
                    count_t + s.count_t,
                    count_n + s.count_n,
                )
            },
        )
    }

    pub fn from_all_statistics(stats: &[BaseStatistics]) -> Self {
        if stats.is_empty() {
            return OverallStatistics {
                total_length: 0,
                total_count_a: 0,
                total_count_c: 0,
                total_count_g: 0,
                total_count_t: 0,
                total_count_n: 0,
                total_gc_content: 0.0,
            };
        }

        let (total_length, total_count_a, total_count_c, total_count_g, total_count_t, total_count_n) =
            Self::calculate_statistics(stats);

        let total_gc_content = if total_length > 0 {
            ((total_count_g + total_count_c) as f64 / total_length as f64) * 100.0
        } else {
            0.0
        };

        OverallStatistics {
            total_length,
            total_count_a,
            total_count_c,
            total_count_g,
            total_count_t,
            total_count_n,
            total_gc_content,
        }
    }

    pub fn to_string_with_thousands_sep(&self) -> String {
        if self.total_length == 0 {
            return "Total\t0\t0\t0\t0\t0\t0\t0.00%\t0.00%\t0.00%\t0.00%\t0.00%\t0.00%".to_string();
        }

        let inv_total_length = 1.0 / self.total_length as f64;

        format!(
            "Total\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{:.2}%\t{:.2}%\t{:.2}%\t{:.2}%\t{:.2}%\t{:.2}%",
            self.total_length.to_formatted_string(&Locale::en),
            self.total_count_a.to_formatted_string(&Locale::en),
            self.total_count_t.to_formatted_string(&Locale::en),
            self.total_count_c.to_formatted_string(&Locale::en),
            self.total_count_g.to_formatted_string(&Locale::en),
            self.total_count_n.to_formatted_string(&Locale::en),
            (self.total_count_g + self.total_count_c).to_formatted_string(&Locale::en),
            (self.total_count_a as f64 * inv_total_length) * 100.0,
            (self.total_count_t as f64 * inv_total_length) * 100.0,
            (self.total_count_c as f64 * inv_total_length) * 100.0,
            (self.total_count_g as f64 * inv_total_length) * 100.0,
            (self.total_count_n as f64 * inv_total_length) * 100.0,
            self.total_gc_content
        )
    }
}