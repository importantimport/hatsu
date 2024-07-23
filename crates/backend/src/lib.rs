use std::net::SocketAddr;

use activitypub_federation::config::{FederationConfig, FederationMiddleware};
use hatsu_utils::{AppData, AppError};
use tower_http::{
    cors::CorsLayer,
    trace::{self, TraceLayer},
};
use tracing::Level;

mod favicon;
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
        // axum 0.6
        // run our app with hyper
        let addr: SocketAddr = format!(
            "{}:{}",
            data.env.hatsu_listen_host, data.env.hatsu_listen_port
        )
        .parse()?;

        tracing::debug!("listening on http://{}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .with_graceful_shutdown(async {
                hatsu_utils::shutdown_signal()
                    .await
                    .expect("failed to install graceful shutdown handler")
            })
            .await?;

        // axum 0.7
        // run our app with hyper
        // let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        // let listener = tokio::net::TcpListener::bind(hatsu_listen)
        //     .await?;
        // tracing::debug!("listening on http://{}", listener.local_addr()?);
        // axum::serve(listener, app).await?;
        Ok::<(), AppError>(())
    };

    let cron = hatsu_cron::run(&federation_config);

    let _res = tokio::join!(http, cron);

    Ok(())
}
