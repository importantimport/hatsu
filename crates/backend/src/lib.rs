use activitypub_federation::config::{FederationConfig, FederationMiddleware};
use hatsu_utils::{AppData, AppError};
use tokio::net::TcpListener;
use tower_http::{
    cors::CorsLayer,
    trace::{self, TraceLayer},
};
use tracing::Level;

mod favicon;
mod openapi;
mod routes;

pub struct Server {
    pub federation_config: FederationConfig<AppData>,
}

impl Server {
    #[must_use]
    pub fn new(federation_config: &FederationConfig<AppData>) -> Self {
        Self {
            federation_config: federation_config.clone(),
        }
    }
}

pub async fn run(federation_config: FederationConfig<AppData>) -> Result<(), AppError> {
    let data = federation_config.to_request_data();

    // build our application with a route
    tracing::info!("creating app");
    let app = routes::routes()
        .layer(FederationMiddleware::new(federation_config.clone()))
        .layer(CorsLayer::permissive())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let http = async {
        // TODO: env HATSU_LISTEN (localhost:3939)
        let listener = TcpListener::bind(format!(
            "{}:{}",
            data.env.hatsu_listen_host, data.env.hatsu_listen_port
        ))
        .await?;
        tracing::debug!("listening on http://{}", listener.local_addr()?);
        axum::serve(listener, app)
            .with_graceful_shutdown(async {
                hatsu_utils::shutdown_signal()
                    .await
                    .expect("failed to install graceful shutdown handler")
            })
            .await?;

        Ok::<(), AppError>(())
    };

    let cron = hatsu_cron::run(&federation_config);

    let _res = tokio::join!(http, cron);

    Ok(())
}
