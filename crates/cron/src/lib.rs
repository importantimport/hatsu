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
        let data: AppData = self.federation_config.to_request_data().app_data().clone();

        let schedule = Schedule::from_str("1/1 * * * * *")?;
        let worker = WorkerBuilder::new("hatsu_cron::jobs::PartialUpdate")
            .data(data)
            // .layer(RetryLayer::new(RetryPolicy::retries(5)))
            // .layer(TraceLayer::new().make_span_with(ReminderSpan::new()))
            .backend(CronStream::new(schedule))
            .build_fn(jobs::partial_update);

        Monitor::<TokioExecutor>::new()
            .register(worker)
            .run()
            .await?;

        Ok(())
    }
}
