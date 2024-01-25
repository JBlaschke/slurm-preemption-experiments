use std::process::Command;
use std::io::{Error, ErrorKind};
use std::fmt;


#[derive(Debug, Clone, Copy)]
pub enum JobState {
    Running,
    Pending
}

#[derive(Debug)]
pub struct JobInfo {
    pub id: u64,
    pub state: JobState,
}

pub struct JobStats {
    pub running: u64,
    pub waiting: u64,
    pub jobs: Vec<JobInfo>
}

impl fmt::Display for JobStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "JobStats(running: {}, waiting: {})",
            self.running, self.waiting
        )
    }
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
        .args(&["--name", job_name, "--format=%A %T"])
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
        let mut running_count = 0;
        let mut waiting_count = 0;

        for line in lines.iter().skip(1) { // skip first (header) line of squeue
            let mut iter = line.split_whitespace();

            // Parse Job ID
            let job_id_string = iter.next().unwrap_or_default().to_string();
            let job_id = match job_id_string.parse::<u64>() {
                Ok(job_id) => {job_id}
                Err(_) => {
                    return Err(Error::new(ErrorKind::Other, "Failed to parse"))
                }
            };

            // Parse Job State
            let job_state_string = iter.next().unwrap_or_default().to_string();
            let job_state = if job_state_string == "RUNNING" {
                running_count += 1;
                JobState::Running
            } else if job_state_string == "PENDING" {
                waiting_count += 1;
                JobState::Pending
            } else {
                return Err(Error::new(ErrorKind::Other, "Failed to parse"))
            };

            jobs.push(JobInfo{
                id: job_id,
                state: job_state
            });
        }

        return Ok(JobStats{
            running: running_count,
            waiting: waiting_count,
            jobs: jobs
        })
    }
    return Err(
        Error::new(ErrorKind::Other, String::from_utf8_lossy(&output.stderr))
    )
}
