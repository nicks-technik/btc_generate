use serde::Serialize;
use std::error::Error;
use std::fs::OpenOptions;

#[derive(Serialize)]
pub struct LogEntry {
    pub log_id: String,
    pub timestamp: String,
    pub network: String,
    pub address: String,
    pub derivation_path: String,
    pub passphrase: String,
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
    let file_exists = std::path::Path::new(file_path).exists();
    
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)?;
    
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false) // We manage headers manually to handle append correctly
        .from_writer(file);
        
    if !file_exists {
        writer.write_record(&[
            "log_id", "timestamp", "network", "address", "derivation_path", "passphrase",
            "word1", "word2", "word3", "word4", "word5", "word6",
            "word7", "word8", "word9", "word10", "word11", "word12"
        ])?;
    }

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
        if std::path::Path::new(file_path).exists() {
            fs::remove_file(file_path).unwrap();
        }

        let entries = vec![
            LogEntry {
                log_id: "uuid1".to_string(),
                timestamp: "2026-03-10".to_string(),
                network: "bitcoin".to_string(),
                address: "addr1".to_string(),
                derivation_path: "m/84h/0h/0h/0/0".to_string(),
                passphrase: "TREZOR".to_string(),
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
        assert!(content.contains("log_id,timestamp,network,address,derivation_path,passphrase"));
        assert!(content.contains("uuid1,2026-03-10,bitcoin,addr1,m/84h/0h/0h/0/0,TREZOR"));
        
        fs::remove_file(file_path).unwrap();
    }
}
