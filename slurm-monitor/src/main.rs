mod cli;
mod slurm;

use crate::cli::{init, parse};
use crate::slurm::{check_slurm, JobStats, JobInfo, JobState};

use tabled::{Tabled, Table};
use tabled::settings::Style;

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

fn main() {
    let args = init();
    let cli = parse(&args);

    println!("Checking slurm job stats for name={}:", cli.name);

    match check_slurm(&cli.name){
        Ok(job_stats) => {
            let stats_table = vec![
                job_stats.to_table(&cli.name)
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

