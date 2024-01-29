use std::fs::File;
use std::io::{self, Read};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::process::Command;
use crate::config::project_path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub txid: String,
    pub address: String,
}

pub fn read_json(file_path: &str) -> io::Result<Vec<Transaction>> {
    let mut file = File::open(file_path)?;
    let mut json_content = String::new();
    file.read_to_string(&mut json_content)?;

    let transactions: Vec<Transaction> = serde_json::from_str(&json_content)?;

    Ok(transactions)
}

pub fn analyze_txid(txid: &str) -> io::Result<serde_json::Value> {
    let ordi_cmd = format!("{}/ord -r decode --txid {} --compact | jq -r '.inscriptions[0].body' | xxd -r -p", project_path::ORDI_PATH,txid);

    let output = Command::new("sh")
        .arg("-c")
        .arg(ordi_cmd)
        .output()?;

    // println!("Command output: {:?}", output);

    if output.status.success() {
        let inscription_body = String::from_utf8_lossy(&output.stdout);
        let json_value: serde_json::Value = serde_json::from_str(&inscription_body)?;

        Ok(json_value)
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        Err(io::Error::new(io::ErrorKind::Other, error_message.to_string()))
    }
}

pub fn write_json(file_path: &str, data: &serde_json::Value) -> io::Result<()> {
    let json_content = serde_json::to_string_pretty(data)?;

    std::fs::write(file_path, json_content)?;

    Ok(())
}

pub fn process_transactions() -> io::Result<()> {
    let transaction_path = format!("{}/data/id_txid_addr.json", project_path::PATH);
    if let Ok(transactions) = read_json(&transaction_path) {
        let mut result: Vec<serde_json::Value> = Vec::new();

        for transaction in transactions {
            let txid = &transaction.txid;
            if let Ok(inscription_body) = analyze_txid(txid) {
                let entry = json!({
                    "id": transaction.id,
                    "txid": txid,
                    "inscription_body": inscription_body,
                    "address": transaction.address
                });
                result.push(entry);
            } else {
                // eprintln!("txid: {} is not inscription transaction, no need to process!", txid);
            }
        }

        
        let result_json_value: serde_json::Value = serde_json::Value::Array(result);
        let wirite_path = format!("{}/data/id_txid_inscription_addr.json", project_path::PATH);
        if let Err(err) = write_json(&wirite_path, &result_json_value) {
            eprintln!("Error writing JSON file: {}", err);
        }
    } else {
        eprintln!("Error reading id_txid_addr.json file.");
    }

    Ok(())
}
