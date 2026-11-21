/// Why a job failed.
#[derive(Debug)]
pub struct JobError(pub String);

/// Runs each job; a job that returns an error is counted as failed.
pub fn run_jobs(jobs: Vec<fn() -> Result<(), JobError>>) -> usize {
    jobs.into_iter().filter(|job| job().is_err()).count()
}
