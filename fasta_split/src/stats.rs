use crate::fasta::FastaSequence;

/// 代表序列统计结果的结构体
pub struct SequenceStats {
    pub id: String,         // 序列标识符
    pub total_length: i64,  // 序列总长度（使用int64）
    pub n_count: usize,     // N的数量
}

/// 计算给定序列列表的统计信息
/// 返回一个包含所有序列统计信息的向量
pub fn calculate_stats(sequences: &Vec<FastaSequence>) -> Vec<SequenceStats> {
    sequences.iter().map(|seq| {
        let total_length = seq.sequence.len() as i64;
        let n_count = seq.sequence.chars().filter(|&c| c == 'N' || c == 'n').count();
        SequenceStats {
            id: seq.id.clone(),
            total_length,
            n_count,
        }
    }).collect()
}

//#[cfg(test)]
//mod tests {
//    use super::*;

//    #[test]
//    fn test_calculate_stats() {
//        // 创建测试FASTA序列
//        let sequences = vec![
//            FastaSequence {
//                id: "seq1".to_string(),
//                description: "description1".to_string(),
//                sequence: "ATCGNNNNATCG".to_string(),
//            },
//            FastaSequence {
//                id: "seq2".to_string(),
//                description: "description2".to_string(),
//                sequence: "GCTAGCTA".to_string(),
//            }
//        ];

//        // 计算统计信息
//        let stats = calculate_stats(&sequences);
//        assert_eq!(stats.len(), 2);

//        // 验证第一条序列的统计信息
//        assert_eq!(stats[0].id, "seq1");
//        assert_eq!(stats[0].total_length, 12);
//        assert_eq!(stats[0].n_count, 4);

//        // 验证第二条序列的统计信息
//        assert_eq!(stats[1].id, "seq2");
//        assert_eq!(stats[1].total_length, 8);
//        assert_eq!(stats[1].n_count, 0);
//    }
//}
