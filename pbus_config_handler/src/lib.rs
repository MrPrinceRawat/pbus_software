use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::{collections::HashMap, error::Error, time::SystemTime};

use utility::Target;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    databases: Vec<Database>,
    update: i32,
    base_path: String,
}

impl Config {
    pub fn new(databases: Vec<Database>, basepath: &str) -> Config {
        Config {
            databases,
            update: 0,
            base_path: basepath.to_string(),
        }
    }

    pub fn add_database(&mut self, database: Database) {
        self.databases.push(database);
    }

    pub fn get_database(&mut self, database_name: &String) -> Option<&mut Database> {
        for database in &mut self.databases {
            if &database.database_name == database_name {
                return Some(database);
            }
        }
        None
    }

    pub fn get_databases(&self) -> &Vec<Database> {
        &self.databases
    }

    pub fn set_databases(&mut self, databases: Vec<Database>) {
        self.databases = databases;
    }

    pub fn replace_database(&mut self, database: Database) {
        for i in 0..self.databases.len() {
            if self.databases[i].database_name == database.database_name {
                self.databases[i] = database.clone();
            }
        }
    }

    fn get_database_names(&self) -> Vec<String> {
        let mut database_names = Vec::new();
        for database in &self.databases {
            database_names.push(database.database_name.clone());
        }
        database_names
    }

    pub fn get_targets(&self) -> Vec<Target> {
        let mut targets = Vec::new();
        for database in &self.databases {
            for target in &database.targets {
                targets.push(target.clone());
            }
        }
        targets
    }

    fn get_database_targets(&self, database_name: String) -> Option<&Vec<Target>> {
        for database in &self.databases {
            if database.database_name == database_name {
                return Some(&database.targets);
            }
        }
        None
    }

    pub fn write_config(&self, base_mount_point: &str) -> Result<(), Box<dyn Error>> {
        let json_str = serde_json::to_string_pretty(&self)?;

        fs::write(format!("{}config.json", base_mount_point), json_str)?;

        Ok(())
    }

    pub fn read_config(base_mount_point: &str) -> Result<Config, Box<dyn Error>> {
        let json_str = fs::read_to_string(format!("{}config.json", base_mount_point))?;

        let config: Config = serde_json::from_str(&json_str)?;

        Ok(config)
    }

    pub fn get_update(&self) -> i32 {
        self.update
    }

    pub fn set_update(&mut self, update: i32) {
        self.update = update;
    }

    pub fn get_base_path(&self) -> &String {
        &self.base_path
    }

    pub fn set_base_path(&mut self, base_path: String) {
        self.base_path = base_path;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Database {
    // Database config
    pub database_host: String,
    pub server_port: u16,
    pub database_user: String,
    pub database_name: String,
    pub database_password: String,
    pub targets: Vec<Target>,
    pub update_interval: u64,
    pub last_updated: SystemTime,
}

impl Database {
    pub fn new(
        database_host: String,
        server_port: u16,
        database_user: String,
        database_name: String,
        database_password: String,
        targets: Vec<Target>,
        update_interval: u64,
        last_updated: SystemTime,
    ) -> Database {
        Database {
            database_host,
            server_port,
            database_user,
            database_name,
            database_password,
            targets,
            update_interval,
            last_updated,
        }
    }

    fn add_target(&mut self, target: Target) {
        self.targets.push(target);
    }

    pub fn get_targets(&self) -> &Vec<Target> {
        &self.targets
    }

    pub fn get_target(&mut self, target_name: String) -> Option<&mut Target> {
        for target in &mut self.targets {
            if target.get_name().to_string() == target_name.to_string() {
                return Some(target);
            }
        }
        None
    }

    fn get_target_fields(&self, target_name: String) -> Option<&HashMap<String, String>> {
        for target in &self.targets {
            if target.get_name().to_string() == target_name {
                return Some(target.get_fields());
            }
        }
        None
    }

    fn get_target_field(&self, target_name: String, field_name: String) -> Option<&String> {
        for target in &self.targets {
            if target.get_name().to_string() == target_name {
                return target.get_field(field_name);
            }
        }
        None
    }

    pub fn get_target_last_id(&self, target_name: String) -> Option<i32> {
        for target in &self.targets {
            if target.get_name().to_string() == target_name {
                return Some(target.get_last_id());
            }
        }
        None
    }

    pub fn set_target_last_id(&mut self, target_name: String, last_id: &i32) {
        for target in &mut self.targets {
            if target.get_name().to_string() == target_name {
                target.set_last_id(*last_id);
            }
        }
    }

    pub fn get_target_last_updated(&self, target_name: String) -> Option<&SystemTime> {
        for target in &self.targets {
            if target.get_name().to_string() == target_name {
                return Some(target.get_last_updated());
            }
        }
        None
    }

    pub fn set_target_last_updated(&mut self, target_name: String, last_updated: SystemTime) {
        for target in &mut self.targets {
            if target.get_name().to_string() == target_name {
                target.set_last_updated(last_updated);
            }
        }
    }

    pub fn get_target_last_checked(&self, target_name: String) -> Option<&SystemTime> {
        for target in &self.targets {
            if target.get_name().to_string() == target_name {
                return Some(target.get_last_checked());
            }
        }
        None
    }

    pub fn set_target_last_checked(&mut self, target_name: String, last_checked: SystemTime) {
        for target in &mut self.targets {
            if target.get_name().to_string() == target_name {
                target.set_last_checked(last_checked);
            }
        }
    }
    pub fn set_target_last_checked_time(&mut self, target_name: String, last_checked: u64) {
        for target in &mut self.targets {
            if target.get_name().to_string() == target_name {
                target.set_last_checked(
                    SystemTime::UNIX_EPOCH
                        .checked_add(std::time::Duration::from_secs(last_checked))
                        .unwrap(),
                );
            }
        }
    }

    pub fn get_target_update_interval(&self, target_name: String) -> Option<u64> {
        for target in &self.targets {
            if target.get_name().to_string() == target_name {
                return Some(self.update_interval);
            }
        }
        None
    }

    pub fn get_target_last_updated_time(&self, target_name: String) -> Option<u64> {
        for target in &self.targets {
            if target.get_name().to_string() == target_name {
                return Some(
                    self.last_updated
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                );
            }
        }
        None
    }

    pub fn set_target_last_updated_time(&mut self, target_name: String, last_updated: u64) {
        for target in &mut self.targets {
            if target.get_name().to_string() == target_name {
                self.last_updated = SystemTime::UNIX_EPOCH
                    .checked_add(std::time::Duration::from_secs(last_updated))
                    .unwrap();
            }
        }
    }

    pub fn get_target_enabled(&self, target_name: String) -> Option<bool> {
        for target in &self.targets {
            if target.get_name().to_string() == target_name {
                return Some(target.get_enabled());
            }
        }
        None
    }

    pub fn set_target_enabled(&mut self, target_name: String, enabled: bool) {
        for target in &mut self.targets {
            if target.get_name().to_string() == target_name {
                target.set_enabled(enabled);
            }
        }
    }

    pub fn get_update_interval(&self) -> u64 {
        self.update_interval
    }
}

#[derive(Debug)]
pub enum ConfigStatus {
    New,
    Existing,
}

pub fn check_config(base_mount_point: &str) -> Result<ConfigStatus, Box<dyn Error>> {
    if fs::metadata(format!("{}/config.json", base_mount_point)).is_err() {
        println!("Config file does not exist, creating new config file");

        let config: Config = Config::new(Vec::new(), base_mount_point);

        config.write_config(base_mount_point)?;

        Ok(ConfigStatus::New)
    } else {
        println!("Config file exists, reading config file");
        let config = Config::read_config(base_mount_point)?;
        Ok(ConfigStatus::Existing)
    }
}

pub fn check_config_update(base_mount_point: &str) -> Result<bool, Box<dyn Error>> {
    let config = Config::read_config(base_mount_point)?;

    let mut update = false;

    if config.update == 1 {
        update = true;
    }

    Ok(update)
}
