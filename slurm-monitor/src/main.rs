mod cli;
mod slurm;

use crate::cli::{init, parse};
use crate::slurm::check_slurm;

fn main() {
    let args = init();
    let cli = parse(&args);

    println!("Checking slurm job stats for name={}:", cli.name);

    match check_slurm(&cli.name){
        Ok(job_stats) => {
            println!("{}", job_stats);
        }
        Err(error) => {
            eprintln!("Error occurred: {}", error);
        }
    }
}

