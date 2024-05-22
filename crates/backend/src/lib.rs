use std::net::SocketAddr;

use activitypub_federation::config::{FederationConfig, FederationMiddleware};
use hatsu_utils::{AppData, AppError};
use tokio_graceful_shutdown::{IntoSubsystem, SubsystemHandle};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

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

#[async_trait::async_trait]
impl IntoSubsystem<AppError, AppError> for Server {
    async fn run(self, subsys: SubsystemHandle<AppError>) -> Result<(), AppError> {
        let data = self.federation_config.to_request_data();

        // build our application with a route
        tracing::info!("creating app");
        let app = routes::routes()
            .layer(FederationMiddleware::new(self.federation_config))
            .layer(CorsLayer::permissive())
            .layer(TraceLayer::new_for_http());

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
            .with_graceful_shutdown(subsys.on_shutdown_requested())
            .await?;

        // axum 0.7
        // run our app with hyper
        // let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        // let listener = tokio::net::TcpListener::bind(hatsu_listen)
        //     .await?;
        // tracing::debug!("listening on http://{}", listener.local_addr()?);
        // axum::serve(listener, app).await?;

        Ok(())
    }
}
