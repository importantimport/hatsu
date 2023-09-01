use tokio_cron_scheduler::{JobScheduler, Job};
use tokio_graceful_shutdown::SubsystemHandle;

use crate::{
    AppData,
    AppError,
    // utilities::fast_update
};

mod update;
use update::fast_update;

pub struct Scheduler {
    pub data: AppData,
}

impl Scheduler {
    pub async fn run(self, _subsys: SubsystemHandle<AppError>) -> Result<(), AppError> {
        tracing::info!("creating scheduler");
        let scheduler: JobScheduler = JobScheduler::new().await?;

        scheduler.add(
            Job::new("0 */5 * * * *", |_, _| {
                tracing::info!("I run every 5 minutes");
            })?
        ).await?;

        scheduler.add(
            Job::new_async("0 */10 * * * *", move |_, _| {
                tracing::info!("I run every 10 minutes");
                let data = self.data.clone();
                Box::pin(async move {
                    match fast_update(&data).await {
                        Ok(_) => tracing::info!("ok"),
                        Err(error) =>  tracing::warn!(%error, "error")
                    }
                })
            })?
        ).await?;

        scheduler.start().await?;

        Ok(())
    }
}
