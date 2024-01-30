use std::process::{Command, ExitStatus};
use std::io::{Write, Error, ErrorKind};
use std::fs::File;
use std::{fmt, env};

use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum JobState {
    Running,
    Pending
}

#[derive(Debug)]
pub struct JobInfo {
    pub id: u64,
    pub state: JobState,
    pub n: u64,
    pub nids: String
}

pub struct JobStats {
    pub running: HashMap<u64, u64>,
    pub waiting: HashMap<u64, u64>,
    pub jobs: Vec<JobInfo>
}

impl fmt::Display for JobState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JobState::Running => write!(f, "Running"),
            JobState::Pending => write!(f, "Pending")
        }
    }
}

pub fn check_slurm(job_name: &str) -> Result<JobStats, Error> {
    // Run `squeue` command to get job information
    let output = Command::new("squeue")
        .args(&["--name", job_name, "--format=%A %T %D %N"])
        .output()
        .expect("Failed to execute command");

    let mut jobs: Vec<JobInfo> = Vec::new();

    // Check if the command was successful
    if output.status.success() {
        // Convert the command output to a string
        let result = String::from_utf8_lossy(&output.stdout);

        // Split the output by lines
        let lines: Vec<&str> = result.lines().collect();

        // Count running and waiting jobs
        let mut running_map: HashMap<u64, u64> = HashMap::new();
        let mut waiting_map: HashMap<u64, u64> = HashMap::new();

        for line in lines.iter().skip(1) { // skip first (header) line of squeue
            let mut iter = line.split_whitespace();

            // Parse Job ID
            let job_id_string = iter.next().unwrap_or_default().to_string();
            let job_id = match job_id_string.parse::<u64>() {
                Ok(job_id) => {job_id}
                Err(_) => {
                    // TODO: better errors
                    return Err(Error::new(ErrorKind::Other, "Failed to parse"))
                }
            };

            // Parse Job State
            let job_state_string = iter.next().unwrap_or_default().to_string();

            let node_number_string = iter.next().unwrap_or_default().to_string();
            let node_number:u64 = match node_number_string.parse() {
                Ok(number) => {number},
                Err(_) => {0}
            };

            let node_nids = iter.next().unwrap_or_default().to_string();

            let job_state = if job_state_string == "RUNNING" {
                let running_count = running_map.entry(node_number).or_insert(0);
                *running_count += 1;
                JobState::Running
            } else if job_state_string == "PENDING" {
                let waiting_count = waiting_map.entry(node_number).or_insert(0);
                *waiting_count += 1;
                JobState::Pending
            } else {
                // TODO: better errors
                return Err(Error::new(ErrorKind::Other, "Failed to parse"))
            };

            jobs.push(JobInfo {
                id: job_id,
                state: job_state,
                n: node_number,
                nids: node_nids.clone()
            });
        }

        return Ok(JobStats{
            running: running_map,
            waiting: waiting_map,
            jobs: jobs
        })
    }
    return Err(
        Error::new(ErrorKind::Other, String::from_utf8_lossy(&output.stderr))
    )
}

pub fn stop_job(id: u64) -> ExitStatus {
    let output = Command::new("scancel")
        .args(&[id.to_string()])
        .output()
        .expect("Failed to execute command");
    return output.status;
}

pub fn start_job(
        nodes: u64, account_name: &str, node_type: &str, qos_name: &str,
        walltime: &str, signal: &str, job_name: &str, reservation_name: &str,
        path: &str, app: &str
    ) -> ExitStatus {

    let current_dir = env::current_dir()
        .expect("Failed to get current working directory");

    env::set_current_dir(path)
        .expect(&format!("Failed to set working directory to {}", path));

    let script_file_name = "preemptible.sh";

    // Create the job script file
    let mut script_file = File::create(script_file_name)
        .expect("Failed to create job script file");

    // Write the job script content to the file with variable substitution
    write!(
        script_file,
        "#!/bin/bash\n\
        \n\
        #SBATCH -N {}\n\
        #SBATCH -A {}\n\
        #SBATCH -C {}\n\
        #SBATCH -q {}\n\
        #SBATCH -t {}\n\
        #SBATCH --signal={}\n\
        #SBATCH --job-name={}\n\
        #SBATCH --reservation={}\n\
        \n\
        srun -n 1 {}\n",
        nodes, account_name, node_type, qos_name, walltime, signal, job_name,
        reservation_name, app
    )
    .expect("Failed to write to job script file");

    // Close the file after writing
    script_file.flush().unwrap();

    // Submit the job using sbatch command
    let output = Command::new("sbatch")
        .arg(script_file_name)
        .output()
        .expect("Failed to execute command");

    env::set_current_dir(current_dir.clone())
        .expect(
            &format!(
                "Failed to reset working directory to {}",
                current_dir.display()
            )
        );

    return output.status;
}
