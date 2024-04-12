#![allow(unused_imports, dead_code)]

use pbus_config_handler::*;
use pbus_timer::*;
use std::error::Error;
use std::thread;
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let base_mount_point = "../data/";

    let config_status: ConfigStatus = check_config(base_mount_point)?;

    let config = Config::read_config(base_mount_point)?;

    match config_status {
        ConfigStatus::New => {
            config.write_config(base_mount_point)?;
        }
        ConfigStatus::Existing => {
            // Handle existing config file case
            // let mut config = Config::read_config()?;
        }
    }

      ("/backup-system/pbus_software/data/").await;

    // Start the worker manager thread
    // let worker_manager = tokio::spawn(async move {
    //     worker_manager("/backup-system/pbus_software/data/config.json").await;
    // });

    // println!("Started Server Successfully!");

    // loop {
    //     // asks the user for input
    //     let mut input = String::new();
    //     std::io::stdin().read_line(&mut input).unwrap();
    //     let input = input.trim();
    // }

    Ok(())
}
