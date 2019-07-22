use chrono::prelude::*;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::{thread, time};

pub struct Config {
    pub duration_in_millis: u32,
    pub log_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough argument");
        }

        // TODO: magic number constant
        let duration_in_millis: u32 = match args[1].trim().parse() {
            Ok(d) => d,
            Err(_) => return Err("invalid param: duration"),
        };

        let log_path = args[2].clone();
        Ok(Config { duration_in_millis, log_path })
    }
}

#[repr(u128)]
pub fn run(conf: Config) {
    let mut file = match OpenOptions::new().append(true).create(true).open(conf.log_path) {
        Err(why) => panic!("couldn't open file, reason: {}", why.description()),
        Ok(file) => file,
    };

    let sleep_duration = time::Duration::from_millis(conf.duration_in_millis as u64);
    let delta = time::Duration::from_millis(10);

    loop {
        let now = time::Instant::now();
        thread::sleep(sleep_duration);
        let elapsed = now.elapsed();

        if elapsed >= sleep_duration + delta {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            let log = format!("{} detected hang: {}ms\n", timestamp, elapsed.as_secs());

            println!("{}", log);
            match file.write_all(log.as_bytes()) {
                Err(why) => panic!("couldn't write: {}", why.description()),
                Ok(_) => println!("successfully wrote"),
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        println!("dummy test!");
    }

    #[test]
    fn test_config_new() {
        let cmd = String::from("hang-monitor ...");
        let test_duration = 100;
        let test_file_path = "/test/file_path/file.log";

        let test_args: Vec<String> = vec![cmd, String::from(test_duration.to_string()), String::from(test_file_path)];
        let conf = Config::new(&test_args).unwrap();

        assert_eq!(test_duration, conf.duration_in_millis);
        assert_eq!(test_file_path, conf.log_path);
    }
}