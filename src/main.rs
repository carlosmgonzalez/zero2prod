// use secrecy::ExposeSecret;
use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgPool::connect_lazy(&configuration.database.connection_string())
        .expect("Failed to connect to Postgres.");
    // .await

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;

    run(listener, connection)?.await
}

// `init` does call `set_logger`, so this is all we need to do.
// We are falling back to printing all logs at info-level or above
// if the RUST_LOG environment variable has not been set.
// env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
