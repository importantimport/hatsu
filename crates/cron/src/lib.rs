use std::str::FromStr;

use activitypub_federation::config::FederationConfig;
use apalis::{
    prelude::{Monitor, WorkerBuilder, WorkerFactoryFn},
    utils::TokioExecutor,
};
use apalis_cron::{CronStream, Schedule};
use hatsu_utils::{AppData, AppError};
use tokio_graceful_shutdown::{IntoSubsystem, SubsystemHandle};

mod jobs;
mod tasks;

pub struct Cron {
    pub federation_config: FederationConfig<AppData>,
}

impl Cron {
    #[must_use]
    pub fn new(federation_config: &FederationConfig<AppData>) -> Self {
        Self {
            federation_config: federation_config.clone(),
        }
    }
}

#[async_trait::async_trait]
impl IntoSubsystem<AppError, AppError> for Cron {
    async fn run(self, _subsys: SubsystemHandle<AppError>) -> Result<(), AppError> {
        let partial_update_schedule = Schedule::from_str("0 */5 * * * *")?;
        let partial_update_worker = WorkerBuilder::new("hatsu_cron::jobs::PartialUpdate")
            .data(self.federation_config.clone())
            // .layer(RetryLayer::new(RetryPolicy::retries(5)))
            // .layer(TraceLayer::new().make_span_with(ReminderSpan::new()))
            .backend(CronStream::new(partial_update_schedule))
            .build_fn(jobs::partial_update);

        let full_update_schedule = Schedule::from_str("0 */10 * * * *")?;
        let full_update_worker = WorkerBuilder::new("hatsu_cron::jobs::FullUpdate")
            .data(self.federation_config)
            .backend(CronStream::new(full_update_schedule))
            .build_fn(jobs::full_update);

        Monitor::<TokioExecutor>::new()
            .register(partial_update_worker)
            .register(full_update_worker)
            .run()
            .await?;

        Ok(())
    }
}
