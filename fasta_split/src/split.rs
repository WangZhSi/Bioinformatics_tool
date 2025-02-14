use crate::fasta::FastaSequence;

/// 代表切割位置的结构体
pub struct SplitPosition {
    pub id: String,       // 原序列标识符
    pub start: usize,     // 切割起始位置
    pub end: usize,       // 切割终止位置
    pub new_id: String,   // 新序列标识符
}

/// 根据N的位置切割序列，并生成新的序列和切割位置记录
/// 返回一个元组，包含新的序列列表和切割位置列表
pub fn split_sequences(sequences: &Vec<FastaSequence>) -> (Vec<FastaSequence>, Vec<SplitPosition>) {
    let mut new_sequences = Vec::new();
    let mut split_positions = Vec::new();

    for seq in sequences {
        let mut start = 0;
        let mut segment_count = 1;

        for (i, c) in seq.sequence.chars().enumerate() {
            if c == 'N' || c == 'n' {
                if start < i {
                    let new_id = format!("{}_{}", seq.id, segment_count);
                    let new_sequence = FastaSequence {
                        id: new_id.clone(),
                        description: seq.description.clone(),
                        sequence: seq.sequence[start..i].to_string(),
                    };
                    new_sequences.push(new_sequence);

                    let split_position = SplitPosition {
                        id: seq.id.clone(),
                        start: start + 1,
                        end: i,
                        new_id: new_id.clone(),
                    };
                    split_positions.push(split_position);

                    segment_count += 1;
                }
                start = i + 1;
            }
        }
        if start < seq.sequence.len() {
            let new_id = format!("{}_{}", seq.id, segment_count);
            let new_sequence = FastaSequence {
                id: new_id.clone(),
                description: seq.description.clone(),
                sequence: seq.sequence[start..].to_string(),
            };
            new_sequences.push(new_sequence);

            let split_position = SplitPosition {
                id: seq.id.clone(),
                start: start + 1,
                end: seq.sequence.len(),
                new_id: new_id.clone(),
            };
            split_positions.push(split_position);
        }
    }

    (new_sequences, split_positions)
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    use crate::fasta::FastaSequence;

//    #[test]
//    fn test_split_sequences() {
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

//        let (new_sequences, split_positions) = split_sequences(&sequences);
        
//        // 检查新序列的数量
//        assert_eq!(new_sequences.len(), 3);

//        // 验证第一条序列的切割结果
//        assert_eq!(new_sequences[0].id, "seq1_1");
//        assert_eq!(new_sequences[0].sequence, "ATCG");

//        assert_eq!(new_sequences[1].id, "seq1_2");
//        assert_eq!(new_sequences[1].sequence, "ATCG");

//        // 验证第二条序列（没有N）的切割结果
//        assert_eq!(new_sequences[2].id, "seq2_1");
//        assert_eq!(new_sequences[2].sequence, "GCTA");

//        // 检查切割位置记录的数量
//        assert_eq!(split_positions.len(), 2);

//        // 验证第一条序列的切割位置
//        assert_eq!(split_positions[0].id, "seq1");
//        assert_eq!(split_positions[0].start, 1);
//        assert_eq!(split_positions[0].end, 4);
//        assert_eq!(split_positions[0].new_id, "seq1_1");

//        assert_eq!(split_positions[1].id, "seq1");
//        assert_eq!(split_positions[1].start, 9);
//        assert_eq!(split_positions[1].end, 12);
//        assert_eq!(split_positions[1].new_id, "seq1_2");
//    }
//}
