mod cli;
mod slurm;

use crate::cli::{init, parse};
use crate::slurm::{check_slurm, JobStats};

use tabled::{Tabled, Table};

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

fn main() {
    let args = init();
    let cli = parse(&args);

    println!("Checking slurm job stats for name={}:", cli.name);

    match check_slurm(&cli.name){
        Ok(job_stats) => {
            let table = vec![
                job_stats.to_table(&cli.name)
            ];
            let table_str = Table::new(table).to_string();
            println!("{}", table_str);
        }
        Err(error) => {
            eprintln!("Error occurred: {}", error);
        }
    }
}

