use std::fs::File;
use std::io::{self, BufRead, BufReader};

/// 代表一条FASTA序列的结构体
pub struct FastaSequence {
    pub id: String,          // 序列标识符
    pub description: String, // 序列描述信息
    pub sequence: String,    // 序列本身
}

/// 从指定文件路径读取并解析FASTA文件
/// 返回一个包含所有FASTA序列的向量
pub fn read_fasta(file_path: &str) -> Result<Vec<FastaSequence>, io::Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file); // 创建缓冲读取器

    let mut sequences = Vec::new(); // 存储解析后的序列
    let mut current_id = String::new();
    let mut current_description = String::new();
    let mut current_sequence = String::new();

    for line in reader.lines() {
        let line = line?; // 逐行读取
        if line.starts_with('>') {
            // 遇到新的序列头，保存之前的序列
            if !current_id.is_empty() {
                sequences.push(FastaSequence {
                    id: current_id.clone(),
                    description: current_description.clone(),
                    sequence: current_sequence.clone(),
                });
                current_sequence.clear(); // 清空当前序列内容
            }
            // 解析序列头
            let parts: Vec<&str> = line[1..].splitn(2, ' ').collect();
            current_id = parts[0].to_string();
            current_description = if parts.len() > 1 {
                parts[1].to_string()
            } else {
                String::new()
            };
        } else {
            // 累加序列内容
            current_sequence.push_str(&line);
        }
    }
    // 保存最后一条序列
    if !current_id.is_empty() {
        sequences.push(FastaSequence {
            id: current_id,
            description: current_description,
            sequence: current_sequence,
        });
    }

    Ok(sequences) // 返回解析后的序列向量
}

//#[cfg(test)]
//mod tests {
//    use super::*;

//    #[test]
//    fn test_read_fasta() {
//        // 测试解析FASTA文件的功能
//        let fasta_content = ">seq1 description
//        ATCG
//        >seq2 another description
//        GCTA";
//        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
//        use std::io::Write;
//        write!(temp_file, "{}", fasta_content).unwrap();

//        let sequences = read_fasta(temp_file.path().to_str().unwrap()).unwrap();
//        assert_eq!(sequences.len(), 2);
//        assert_eq!(sequences[0].id, "seq1");
//        assert_eq!(sequences[0].description, "description");
//        assert_eq!(sequences[0].sequence, "ATCG");
//        assert_eq!(sequences[1].id, "seq2");
//        assert_eq!(sequences[1].description, "another description");
//        assert_eq!(sequences[1].sequence, "GCTA");
//    }
//}
