use activitypub_federation::config::FederationConfig;
use hatsu_utils::{AppData, AppError};
use tokio_cron_scheduler::{Job, JobScheduler};
use tokio_graceful_shutdown::{IntoSubsystem, SubsystemHandle};

mod update;

pub struct Scheduler {
    pub federation_config: FederationConfig<AppData>,
}

impl Scheduler {
    #[must_use]
    pub fn new(federation_config: &FederationConfig<AppData>) -> Self {
        Self {
            federation_config: federation_config.clone(),
        }
    }
}

#[async_trait::async_trait]
impl IntoSubsystem<AppError, AppError> for Scheduler {
    async fn run(self, _subsys: SubsystemHandle<AppError>) -> Result<(), AppError> {
        tracing::info!("creating scheduler");
        let scheduler: JobScheduler = JobScheduler::new().await?;

        let config = self.federation_config.clone();
        scheduler
            .add(Job::new_async("0 */10 * * * *", move |_, _| {
                let data = config.to_request_data();
                Box::pin(async move {
                    match update::fast_update(&data).await {
                        Ok(()) => tracing::info!("ok"),
                        Err(error) => tracing::warn!(%error, "error"),
                    }
                })
            })?)
            .await?;

        let config = self.federation_config.clone();
        scheduler
            .add(Job::new_async("0 */30 * * * *", move |_, _| {
                let data = config.to_request_data();
                Box::pin(async move {
                    match update::full_update(&data).await {
                        Ok(()) => tracing::info!("ok"),
                        Err(error) => tracing::warn!(%error, "error"),
                    }
                })
            })?)
            .await?;

        scheduler.start().await?;

        Ok(())
    }
}
