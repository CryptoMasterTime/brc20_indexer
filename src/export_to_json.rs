use std::fs::File;
use std::io::{self, BufRead};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Transaction {
    id: String,
    txid: String,
    address: String,
}

pub(crate) fn read_export_data(file_path: &str, output_file: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut transactions = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with('#') {
            continue;
        }
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 4 {
            
            let id = parts[0].to_string();
            let txid = parts[2].split(':').next().unwrap_or_default().to_string();
            let address = parts[3].to_string();
            
            let transaction = Transaction { id, txid, address };
            transactions.push(transaction);
        }
    }

    
    write_json(output_file, &transactions)?;

    Ok(())
}

fn write_json(file_path: &str, transactions: &[Transaction]) -> io::Result<()> {
    let json_content = serde_json::to_string(transactions)?;
    std::fs::write(file_path, json_content)?;
    Ok(())
}

