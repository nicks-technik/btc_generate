use serde::Serialize;
use std::error::Error;
use std::fs::OpenOptions;

#[derive(Serialize)]
pub struct LogEntry {
    pub address: String,
    pub word1: String,
    pub word2: String,
    pub word3: String,
    pub word4: String,
    pub word5: String,
    pub word6: String,
    pub word7: String,
    pub word8: String,
    pub word9: String,
    pub word10: String,
    pub word11: String,
    pub word12: String,
}

pub fn write_to_csv(file_path: &str, entries: &[LogEntry]) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)?;
    
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);
        
    for entry in entries {
        writer.serialize(entry)?;
    }
    
    writer.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_write_to_csv() {
        let file_path = "test_log.csv";
        let entries = vec![
            LogEntry {
                address: "addr1".to_string(),
                word1: "w1".to_string(),
                word2: "w2".to_string(),
                word3: "w3".to_string(),
                word4: "w4".to_string(),
                word5: "w5".to_string(),
                word6: "w6".to_string(),
                word7: "w7".to_string(),
                word8: "w8".to_string(),
                word9: "w9".to_string(),
                word10: "w10".to_string(),
                word11: "w11".to_string(),
                word12: "w12".to_string(),
            }
        ];

        write_to_csv(file_path, &entries).unwrap();

        let content = fs::read_to_string(file_path).unwrap();
        assert!(content.contains("addr1,w1,w2,w3,w4,w5,w6,w7,w8,w9,w10,w11,w12"));
        
        fs::remove_file(file_path).unwrap();
    }
}
