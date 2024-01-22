use std::process::Command;
use std::io::{Error, ErrorKind};
use std::fmt;

pub struct JobStats {
    pub running: u64,
    pub waiting: u64
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

pub fn check_slurm(job_name: &str) -> Result<JobStats, Error> {
    // Run `squeue` command to get job information
    let output = Command::new("squeue")
        .args(&["--name", job_name])
        .output()
        .expect("Failed to execute command");

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
            if line.contains(" R ") {
                running_count += 1;
            } else if line.contains(" PD ") {
                waiting_count += 1;
            }
        }
        return Ok(JobStats{
            running: running_count,
            waiting: waiting_count
        })
    }
    return Err(
        Error::new(ErrorKind::Other, String::from_utf8_lossy(&output.stderr))
    )
}
