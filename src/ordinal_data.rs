use std::process::Command;
use crate::config::project_path;
pub(crate) fn export_index_data(export_data_dir: &str) {
    
    let ordi1_cmd = format!("{}/ord", project_path::ORDI_PATH);
    let output = Command::new(ordi1_cmd)
        .arg("-r")
        .arg("index")
        .arg("export")
        .arg("--include-addresses")
        .arg("--tsv")
        .arg(export_data_dir) 
        .output()
        .expect("Failed to execute the command");

    
    if output.status.success() {
        // println!("ordinals export chaindata indexer executed successfully!");
    } else {
        eprintln!("Command failed with error code: {:?}", output.status);
    }
}
