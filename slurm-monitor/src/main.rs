mod cli;
mod slurm;

use crate::cli::{init, parse};
use crate::slurm::{
    check_slurm, start_job, stop_job, JobStats, JobInfo, JobState
};

use tabled::{Tabled, Table};
use tabled::settings::Style;

use std::collections::{HashMap, HashSet};
use toml::Value;
use std::fs;

use std::thread;
use std::time::Duration;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

#[derive(Tabled)]
struct JobStatsRow<'a> {
    name: &'a str,
    nodes: u64,
    running: u64,
    waiting: u64
}

impl JobStats {
    fn to_table<'a>(&'a self, name: &'a str, nodes: &u64) -> JobStatsRow {
        let running = match self.running.get(nodes) {
            Some(value) => {value},
            None => {&0}
        };
        let waiting = match self.waiting.get(nodes) {
            Some(value) => {value},
            None => {&0}
        };

        JobStatsRow{
            name: name,
            nodes: *nodes,
            running: *running,
            waiting: *waiting
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

fn interpret_i64(value: &Value) -> i64 {
    match value {
        Value::Integer(i) => *i,
        _ => 0 // Default to 0 -- TODO: use options here
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

fn generate_random_string(length: usize) -> String {
    let rng = thread_rng();
    let random_string: String = rng.sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    random_string
}

fn union_keys<'a, T: std::cmp::Eq + std::hash::Hash,W>(
        a: &'a HashMap<T,W>, b: &'a HashMap<T,W>
    ) -> HashSet<&'a T> {
    let keys_a: HashSet<_> = a.keys().collect();
    let keys_b: HashSet<_> = b.keys().collect();
    let union_keys: HashSet<_> = keys_a.union(&keys_b).cloned().collect();
    return union_keys;
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
    let targets = interpret_number_vec(&settings["targets"]);

    let account_name = interpret_string(&settings["account_name"]);
    let node_type = interpret_string(&settings["node_type"]);
    let qos_name = interpret_string(&settings["qos_name"]);
    let walltime = interpret_string(&settings["walltime"]);
    let signal = interpret_string(&settings["signal"]);
    let reservation_name = interpret_string(&settings["reservation_name"]);
    let app = interpret_string(&settings["app"]);

    loop {
        println!("Checking slurm job stats for name={}:", name);

        match check_slurm(&name){
            Ok(job_stats) => {
                let mut stats_table: Vec<JobStatsRow> = Vec::new();
                let all_keys = union_keys(&job_stats.running, &job_stats.waiting);
                for &key in all_keys.iter() {
                    stats_table.push(job_stats.to_table(&name, key));
                }

                let stats_str = Table::new(stats_table).with(Style::sharp()).to_string();
                println!("{}", stats_str);

                let mut jobs_table: Vec<JobInfoRow> = Vec::new();
                for j in &job_stats.jobs {
                    jobs_table.push(j.to_table());
                }

                let jobs_str = Table::new(jobs_table).with(Style::sharp()).to_string();
                println!("{}", jobs_str);

                if cli.drain {
                    println!("Draining all jobs!");
                    for job in job_stats.jobs {
                        println!("Cancelling: {}", job.id);
                        stop_job(job.id);
                    }
                    break;
                };

                let combined_iter = nodes
                    .clone()
                    .into_iter()
                    .zip(targets.clone().into_iter());
                for (n, t) in combined_iter {
                    let nnode = n.try_into().unwrap();
                    let target = t.try_into().unwrap();
                    let running: u64 = match job_stats.running.get(&nnode) {
                        Some(v) => *v,
                        None => 0
                    };

                    let waiting: u64 = match job_stats.waiting.get(&nnode) {
                        Some(v) => *v,
                        None => 0
                    };

                    let total = running + waiting;
                    if total < target {
                        let rand_dir = generate_random_string(16);
                        println!("Submitting job in: {}", rand_dir);
                        match fs::create_dir(rand_dir.clone()){
                            Err(_) => eprintln!("Failed to create: {}", rand_dir),
                            _ => {
                                start_job(
                                    nnode, &account_name, &node_type, &qos_name,
                                    &walltime, &signal, &name, &reservation_name,
                                    &rand_dir, &app
                                );
                            }
                        }
                    }
                }

            }
            Err(error) => {
                eprintln!("Error occurred: {}", error);
            }
        }

        thread::sleep(Duration::from_secs(
                interpret_i64(&settings["sleep"]).try_into().unwrap()
        ));
    }
}

