use tokio_cron_scheduler::{JobScheduler, Job};
use tokio_graceful_shutdown::SubsystemHandle;

use crate::AppError;

pub async fn init(
    _subsys: SubsystemHandle<AppError>
) -> Result<(), AppError> {
    tracing::info!("creating scheduler");
    let scheduler: JobScheduler = JobScheduler::new().await?;

    scheduler.add(
        Job::new("0 */5 * * * *", |_, _| {
            tracing::info!("I run every 5 minutes");
        })?
    ).await?;

    scheduler.start().await?;

    Ok(())
}
