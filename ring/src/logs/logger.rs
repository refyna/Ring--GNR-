use tracing_appender::rolling::{
    RollingFileAppender,
    Rotation
};

pub fn init_logger() {
    let file_appender = RollingFileAppender::new(
        Rotation::DAILY,
        "rg_logs",
        "app.lpg"
    );

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_max_level(tracing::Level::INFO)
        .with_ansi(false)
        .init();

    std::mem::forget(_guard);
}