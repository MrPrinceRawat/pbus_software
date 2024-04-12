use std::{collections::HashMap, time::SystemTime};

use serde::{Deserialize, Serialize};

pub struct TableField {
    pub name: String,
    pub data_type: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Target {
    name: String,
    // create a hashmap of fields that maps string to type T
    fields: HashMap<String, String>,
    last_id: i32,
    last_updated: SystemTime,
    last_checked: SystemTime,
    enabled: bool,
}

impl Target {
    pub fn new(name: String) -> Target {
        Target {
            name,
            fields: HashMap::new(),
            last_id: 0,
            last_updated: SystemTime::now(),
            last_checked: SystemTime::now(),
            enabled: true,
        }
    }

    pub fn construct(
        name: String,
        fields: HashMap<String, String>,
        last_id: i32,
        last_updated: String,
        last_checked: String,
        enabled: bool,
    ) -> Target {
        let last_updated = SystemTime::UNIX_EPOCH
            .checked_add(std::time::Duration::from_secs(
                last_updated.parse().unwrap(),
            ))
            .unwrap();
        let last_checked = SystemTime::UNIX_EPOCH
            .checked_add(std::time::Duration::from_secs(
                last_checked.parse().unwrap(),
            ))
            .unwrap();

        Target {
            name,
            fields,
            last_id: last_id,
            last_updated: last_updated,
            last_checked: last_checked,
            enabled,
        }
    }

    pub fn add_field(&mut self, name: String, value: String) {
        self.fields.insert(name, value);
    }

    pub fn set_fields(&mut self, fields: Vec<TableField>) {
        for field in fields {
            self.fields.insert(field.name, field.data_type);
        }
    }

    pub fn get_field(&self, name: String) -> Option<&String> {
        self.fields.get(&name)
    }

    pub fn get_fields(&self) -> &HashMap<String, String> {
        &self.fields
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_last_id(&self) -> i32 {
        self.last_id
    }

    pub fn set_last_id(&mut self, last_id: i32) {
        self.last_id = last_id;
    }

    pub fn get_last_updated(&self) -> &SystemTime {
        &self.last_updated
    }

    pub fn set_last_updated(&mut self, last_updated: SystemTime) {
        self.last_updated = last_updated;
    }

    pub fn get_last_checked(&self) -> &SystemTime {
        &self.last_checked
    }

    pub fn set_last_checked(&mut self, last_checked: SystemTime) {
        self.last_checked = last_checked;
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}
