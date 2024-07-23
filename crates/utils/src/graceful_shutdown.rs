/// from <https://github.com/tokio-rs/axum/blob/37e4574012e4692931b53e22d44ce4a3a760002f/examples/graceful-shutdown/src/main.rs#L51-L73>
pub async fn shutdown_signal() -> std::io::Result<()> {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        c = ctrl_c => Ok(c),
        t = terminate => Ok(t),
    }
}
