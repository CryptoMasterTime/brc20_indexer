use std::fs::File;
//use std::io::{self, BufRead};
use serde::{Serialize, Deserialize};
/* 
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
*/
//step one ^|^
//
//


//use std::fs::File;
use std::io::{self, Read};
//use serde::{Deserialize, Serialize};
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
    let ordi_cmd = format!("ord  decode --txid {} --compact | jq -r '.inscriptions[0].body' | xxd -r -p",txid);

    let output = Command::new("sh")
        .arg("-c")
        .arg(ordi_cmd)
        .output()?;

    //println!("Command output: {:?}", output);

    if output.status.success() {
        let inscription_body = String::from_utf8_lossy(&output.stdout);
        let json_value: serde_json::Value = serde_json::from_str(&inscription_body)?;
        //println!("{}",json_value);
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
    let transaction_path = "/home/CytptoMasterTime/brc20-indexer/id_txid_addr.json";
    if let Ok(transactions) = read_json(&transaction_path) {
        let mut result: Vec<serde_json::Value> = Vec::new();
        //let transactions_slice = if transactions.len() > 100 {
        //    &transactions[transactions.len() - 100..]
        //} else {
        //    &transactions
        //};
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
                        continue
                    }
                println!("{}", entry);
                result.push(entry);
                  // eprintln!("txid: {} is not inscription transaction, no need to process!", txid);
            }
        }
        // for transaction in transactions.iter().rev() {
        //     let txid = &transaction.txid;
        //     // println!("{}", txid);
        //     if let Ok(inscription_body) = analyze_txid(txid) {
        //         let entry = json!({
        //             "id": transaction.id,
        //             "txid": txid,
        //             "inscription_body": inscription_body,
        //             "address": transaction.address
        //         });
        //         // println!("id: {},inscription_body: {},address: {}", transaction.id,inscription_body,transaction.address );
        //         result.push(entry);
        //     } else {
        //         // eprintln!("txid: {} is not inscription transaction, no need to process!", txid);
        //     }
        // }



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
    //let export_json = format!("/home/CytptoMasterTime/brc20-indexer/id_txid_addr.json");
    //let export_data_path = "/home/CytptoMasterTime/brc20-indexer/ord.export.tsv";
    //if let Err(_err) = read_export_data(&export_data_path, &export_json) {

    let _ = process_transactions();
    //let export_json = format!("{}/data/id_txid_inscription_addr.json", project_path::PATH);
    //let _ = process_balance::process_transactions(&export_json);
    //println!("Indexer data executed successfully!")
}
