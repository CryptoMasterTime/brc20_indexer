use std::fs::File;
use std::io::{self, Read};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction2 {
    pub id: String,
    pub txid: String,
    pub address: String,
}

pub fn read_json(file_path: &str) -> io::Result<Vec<Transaction2>> {
    let mut file = File::open(file_path)?;
    let mut json_content = String::new();
    file.read_to_string(&mut json_content)?;

    let transactions: Vec<Transaction2> = serde_json::from_str(&json_content)?;

    Ok(transactions)
}

pub fn analyze_txid(txid: &str) -> io::Result<serde_json::Value> {
    let ordi_cmd = format!("ord  --bitcoin-rpc-url=http://0.0.0.0:8332 --bitcoin-rpc-username=rooch-main --bitcoin-rpc-password=rooch1202$ decode --txid {} --compact | jq -r '.inscriptions[0].body' | xxd -r -p",txid);

    let output = Command::new("sh")
        .arg("-c")
        .arg(ordi_cmd)
        .output()?;

    if output.status.success() {
        let inscription_body = String::from_utf8_lossy(&output.stdout);
        let json_value: serde_json::Value = serde_json::from_str(&inscription_body)?;
        Ok(json_value)
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        Err(io::Error::new(io::ErrorKind::Other, error_message.to_string()))
    }
}

pub fn write_json2(file_path: &str, data: &serde_json::Value) -> io::Result<()> {
    let json_content = serde_json::to_string_pretty(data)?;

    std::fs::write(file_path, json_content)?;

    Ok(())
}

pub fn process_transactions() -> io::Result<()> {
    let transaction_path = "/root/data/indexer/brc20-indexer/id_txid_addr.json";
    if let Ok(transactions) = read_json(&transaction_path) {
        let mut result: Vec<serde_json::Value> = Vec::new();
        let mut found_target_txid = false;

        for transaction in transactions {
            let txid = &transaction.txid;
            println!("{}", txid);
            if !found_target_txid && txid != "b61b0172d95e266c18aea0c624db987e971a5d6d4ebc2aaed85da4642d635735" {
                println!("continue!");
                continue;
            }
            found_target_txid = true;
            if let Ok(inscription_body) = analyze_txid(txid) {
                let entry = json!({
                    "id": transaction.id,
                    "txid": txid,
                    "inscription_body": inscription_body,
                    "address": transaction.address
                });
                if let Some(p) = inscription_body.get("p").and_then(|v| v.as_str()) {
                    if p != "brc-20" {
                        continue;
                    }
                } else {
                    continue;
                }
                println!("{}", entry);
                result.push(entry);
            }
        }

        let result_json_value: serde_json::Value = serde_json::Value::Array(result);
        let wirite_path = "/home/CytptoMasterTime/brc20-indexer/id_txid_inscription_addr_new.json";
        if let Err(err) = write_json2(&wirite_path, &result_json_value) {
            eprintln!("Error writing JSON file: {}", err);
        }
    } else {
        eprintln!("Error reading id_txid_addr.json file.");
    }

    Ok(())
}

fn main() {
    if let Err(_err) = process_transactions() {
        eprintln!("Error processing transactions.");
    }
}
