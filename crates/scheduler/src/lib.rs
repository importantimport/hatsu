use activitypub_federation::config::FederationConfig;
use hatsu_utils::{AppData, AppError};
use tokio_cron_scheduler::{Job, JobScheduler};
use tokio_graceful_shutdown::SubsystemHandle;

mod update;

pub struct Scheduler {
    pub config: FederationConfig<AppData>,
}

impl Scheduler {
    pub async fn run(self, _subsys: SubsystemHandle<AppError>) -> Result<(), AppError> {
        tracing::info!("creating scheduler");
        let scheduler: JobScheduler = JobScheduler::new().await?;
        // let fast_config = self.config.clone();
        let full_config = self.config.clone();

        scheduler
            .add(Job::new_async("0 */10 * * * *", move |_, _| {
                // tracing::info!("I run every 10 minutes");
                // let data = fast_config.to_request_data();
                let data = self.config.to_request_data();
                Box::pin(async move {
                    match update::fast_update(&data).await {
                        Ok(_) => tracing::info!("ok"),
                        Err(error) => tracing::warn!(%error, "error"),
                    }
                })
            })?)
            .await?;

        scheduler
            .add(Job::new_async("0 */30 * * * *", move |_, _| {
                // tracing::info!("I run every 30 minutes");
                let data = full_config.to_request_data();
                Box::pin(async move {
                    match update::full_update(&data).await {
                        Ok(_) => tracing::info!("ok"),
                        Err(error) => tracing::warn!(%error, "error"),
                    }
                })
            })?)
            .await?;

        scheduler.start().await?;

        Ok(())
    }
}
