use std::str::FromStr;

use activitypub_federation::config::FederationConfig;
use apalis::{
    prelude::{Monitor, WorkerBuilder, WorkerFactoryFn},
    utils::TokioExecutor,
};
use apalis_cron::{CronStream, Schedule};
use hatsu_utils::{AppData, AppError};

mod jobs;
mod tasks;

pub async fn run(federation_config: &FederationConfig<AppData>) -> Result<(), AppError> {
    let partial_update_schedule = Schedule::from_str("0 */5 * * * *")?;
    let partial_update_worker = WorkerBuilder::new("hatsu_cron::jobs::PartialUpdate")
        .data(federation_config.clone())
        // .layer(RetryLayer::new(RetryPolicy::retries(5)))
        // .layer(TraceLayer::new().make_span_with(ReminderSpan::new()))
        .backend(CronStream::new(partial_update_schedule))
        .build_fn(jobs::partial_update);

    let full_update_schedule = Schedule::from_str("0 */10 * * * *")?;
    let full_update_worker = WorkerBuilder::new("hatsu_cron::jobs::FullUpdate")
        .data(federation_config.clone())
        .backend(CronStream::new(full_update_schedule))
        .build_fn(jobs::full_update);

    Monitor::<TokioExecutor>::new()
        .register(partial_update_worker)
        .register(full_update_worker)
        .run_with_signal(hatsu_utils::shutdown_signal())
        .await?;

    Ok(())
}
