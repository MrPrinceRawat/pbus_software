use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub struct HitTargets {
    name: String,
    database_name: String,
    last_hit: SystemTime,
    interval: Duration,
    next_hit: SystemTime,
}

impl HitTargets {
    pub fn new(name: String, db_name: String, interval: Duration) -> HitTargets {
        let last_hit = SystemTime::now();
        let next_hit = last_hit + interval;
        HitTargets {
            name,
            database_name: db_name,
            last_hit,
            interval,
            next_hit,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_last_hit(&self) -> SystemTime {
        self.last_hit
    }

    pub fn get_interval(&self) -> Duration {
        self.interval
    }

    pub fn get_next_hit(&self) -> SystemTime {
        self.next_hit
    }

    pub fn set_last_hit(&mut self, last_hit: SystemTime) {
        self.last_hit = last_hit;
    }

    pub fn set_next_hit(&mut self, next_hit: SystemTime) {
        self.next_hit = next_hit;
    }

    pub fn copy(&self) -> HitTargets {
        HitTargets {
            name: self.name.clone(),
            database_name: self.database_name.clone(),
            last_hit: self.last_hit,
            interval: self.interval,
            next_hit: self.next_hit,
        }
    }

    pub fn get_database_name(&self) -> String {
        self.database_name.clone()
    }
}
