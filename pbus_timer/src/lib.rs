use pbus_config_handler::*;
use std::thread;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use utility::*;

/// Main thread function for the timer
///
/// # Arguments
/// * `times` - A vector of time_handler::HitTargets
/// * `base_mount_point` - The base mount point for the config file
///

pub async fn worker_manager(base_mount_point: &str) {
    loop {
        let mut times: Vec<time_handler::HitTargets> = Vec::new();
        let config = Config::read_config(base_mount_point).unwrap();
        for Database in config.get_databases() {
            for target in Database.get_targets() {
                if &target.get_enabled() == &false {
                    continue;
                }
                times.push(time_handler::HitTargets::new(
                    target.get_name().to_string(),
                    Database.database_name.to_string(),
                    Duration::from_secs(Database.get_update_interval() * 1),
                ));
            }
        }
        worker(base_mount_point, &mut times).await;
    }
}

pub async fn worker(base_mount_point: &str, times: &mut Vec<time_handler::HitTargets>) {
    println!("Starting Worker!");
    let mut config = Config::read_config(base_mount_point).unwrap();
    loop {
        if check_config_update(base_mount_point).unwrap() {
            println!("Config updated, restarting");
            config.set_update(0);
            config.write_config(base_mount_point).unwrap();
            break;
        }
        for time in times.iter_mut() {
            if time.get_next_hit() < SystemTime::now() {
                println!("Hitting target: {}", time.get_name());
                println!("Next hit: {:?}", time.get_next_hit());
                // update config

                config
                    .get_database(&time.get_database_name())
                    .unwrap()
                    .set_target_last_checked(time.get_name().to_string(), SystemTime::now());

                config.write_config(base_mount_point).unwrap();
            }
        }
        thread::sleep(Duration::from_secs(10));
    }
}
