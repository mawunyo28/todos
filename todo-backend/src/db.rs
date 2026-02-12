use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;
use tokio_postgres::Client;

pub async fn create_client(database_url: &str) -> Client {
    let connector = TlsConnector::builder()
        .danger_accept_invalid_certs(true) // ← Add this line
        .build()
        .expect("Failed to build TLS connector");
    let connector = MakeTlsConnector::new(connector);

    let (client, connection) = tokio_postgres::connect(database_url, connector)
        .await
        .expect("Failed to connect to Postgres");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client
}

