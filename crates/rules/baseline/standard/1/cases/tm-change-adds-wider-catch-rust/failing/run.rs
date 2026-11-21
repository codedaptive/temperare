use std::panic;

/// Runs each job; a job that panics is counted as failed.
pub fn run_jobs(jobs: Vec<fn() -> bool>) -> usize {
    jobs.into_iter()
        .filter(|job| !panic::catch_unwind(job).unwrap_or(false))
        .count()
}
