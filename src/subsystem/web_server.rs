use std::net::ToSocketAddrs;

use activitypub_federation::config::{FederationConfig, FederationMiddleware};
use axum::Router;
use tokio_graceful_shutdown::SubsystemHandle;

use crate::{
    AppData,
    AppError,
    routes,
    entities::user::Model as DbUser,
};

pub struct WebServer {
    pub data: AppData,
    pub hatsu_domain: String,
    pub hatsu_listen: String,
    pub test_account: DbUser
}

impl WebServer {
    pub async fn run(self, subsys: SubsystemHandle<AppError>) -> Result<(), AppError> {
        tracing::info!("setup configuration");
        let federation_config = FederationConfig::builder()
            // 实例域名，这里使用 `HATSU_DOMAIN` 环境变量
            // instance domain, `HATSU_DOMAIN` environment is used here.
            .domain(self.hatsu_domain)
            // 使用测试账户作为 Signed fetch actor，以和 GoToSocial 或启用安全模式的 Mastodon 实例交互
            // Use a test account as a Signed fetch actor to interact with GoToSocial or a Mastodon instance with secure mode enabled
            .signed_fetch_actor(&self.test_account)
            // Fediverse 应用数据，目前只有数据库连接
            // Fediverse application data, currently only database connections
            .app_data(self.data)
            // TODO:
            // Disable this configuration when Pleroma supports HTTP Signature draft-11
            // 当 Pleroma 支持 HTTP Signature draft-11 时，禁用此配置
            // https://git.pleroma.social/pleroma/pleroma/-/issues/2939
            .http_signature_compat(true)
            .build()
            .await?;

        // build our application with a route
        tracing::info!("creating app");
        let app = Router::new()
            .merge(routes::init())
            .layer(FederationMiddleware::new(federation_config));

        // axum 0.6
        // run our app with hyper
        let addr = self.hatsu_listen
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
