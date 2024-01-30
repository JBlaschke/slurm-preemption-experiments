mod cli;
mod slurm;

use crate::cli::{init, parse};
use crate::slurm::{check_slurm, JobStats, JobInfo, JobState};

use tabled::{Tabled, Table};
use tabled::settings::Style;

use std::collections::HashMap;
use toml::Value;
use std::fs;

#[derive(Tabled)]
struct JobStatsRow<'a> {
    name: &'a str,
    running: u64,
    waiting: u64
}

impl JobStats {
    fn to_table<'a>(&'a self, name: &'a str) -> JobStatsRow {
        JobStatsRow{
            name: name,
            running: self.running,
            waiting: self.waiting
        }
    }
}

#[derive(Tabled)]
struct JobInfoRow<'a> {
    id: u64,
    state: JobState,
    n:u64,
    nids: &'a str
}

impl JobInfo {
    fn to_table<'a>(&'a self) -> JobInfoRow {
        JobInfoRow {
            id: self.id,
            state: self.state,
            n: self.n,
            nids: &self.nids
        }
    }
}

fn read_settings(file_path: &str) ->HashMap<String, Value> {
    // When you've got some time -- these should return an option, not crash the
    // program
    let toml_str = fs::read_to_string(file_path).unwrap();
    let hashmap: HashMap<String, Value> = toml::from_str(&toml_str).unwrap();

    hashmap
}

fn interpret_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(), // For strings, just clone the value
        _ => value.to_string(), // For other types, use the default to_string method
    }
}

fn interpret_number_vec(value: &Value) -> Vec<i64> {

    let mut numbers:Vec<i64> = Vec::new();

    if let Some(numbers_array) = value.as_array() {
        for number in numbers_array {
            if let Some(integer_value) = number.as_integer() {
                numbers.push(integer_value);
            } else {
                // TODO: return error
            }
        }
    } else {
        // TODO: return error
    }

    numbers
}

fn main() {
    let args = init();
    let cli = parse(&args);

    println!("Using settings file: {}", cli.settings);
    let settings = read_settings(&cli.settings);
    for (key, value) in settings.clone().into_iter() {
        println!("Key: {}, Value: {}", key, value);
    }

    let name = interpret_string(&settings["name"]);
    let nodes = interpret_number_vec(&settings["nodes"]);

    loop {
        println!("Checking slurm job stats for name={}:", name);

        match check_slurm(&name){
            Ok(job_stats) => {
                let stats_table = vec![
                    job_stats.to_table(&name)
                ];
                let stats_str = Table::new(stats_table).with(Style::sharp()).to_string();
                println!("{}", stats_str);

                let mut jobs_table: Vec<JobInfoRow> = Vec::new();
                for j in &job_stats.jobs {
                    jobs_table.push(j.to_table());
                }

                let jobs_str = Table::new(jobs_table).with(Style::sharp()).to_string();
                println!("{}", jobs_str);
            }
            Err(error) => {
                eprintln!("Error occurred: {}", error);
            }
        }
    }
}

