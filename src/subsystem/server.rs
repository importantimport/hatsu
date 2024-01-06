use std::net::ToSocketAddrs;

use activitypub_federation::config::{FederationConfig, FederationMiddleware};
use axum::Router;
use hatsu_utils::{AppData, AppEnv, AppError};
use tokio_graceful_shutdown::SubsystemHandle;

use crate::routes;

pub struct Server {
    pub federation_config: FederationConfig<AppData>,
    pub env: AppEnv,
}

impl Server {
    pub async fn run(self, subsys: SubsystemHandle<AppError>) -> Result<(), AppError> {
        // build our application with a route
        tracing::info!("creating app");
        let app = Router::new()
            .merge(routes::handler())
            .layer(FederationMiddleware::new(self.federation_config));

        // axum 0.6
        // run our app with hyper
        let addr = format!("{}:{}", self.env.hatsu_listen_host, self.env.hatsu_listen_port)
            .to_socket_addrs()?
            .next()
            .expect("Failed to lookup domain name");

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
