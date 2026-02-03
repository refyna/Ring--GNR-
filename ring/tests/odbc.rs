use croma::odbc::{
    Connect, 
    StrConn, 
    psql::PsqlConnector,
    mssql::MssqlConnector,
    mongo::MongoConnector,
    csdr::CSDRConnector,
};

// cargo execute test in parallel and env_logger wait one call
fn init_logger() {
    let _ = env_logger::builder()
        .is_test(true)
        .try_init();
}

#[tokio::test]
async fn conn_psql() {
    init_logger();

    let user = "postgres".to_string();
    let pwd = "postgresrfy123$".to_string();
    let dbn = "postgres".to_string();
    let host = "localhost".to_string();
    let port = 5432;

    let cfg = StrConn::
        new(host, port)
        .with_auth(user, pwd)
        .with_db(dbn);

    let connector = PsqlConnector {cfg};
    let result = connector.conn().await;

    // match connector.conn().await {
    //     Ok(_) => println!("Conectado"),
    //     Err(e) => drop(e)    
    // }

    assert!(
        result.is_ok(),
        "Failed to connect to PostgreSQL: {:?}",
        result.err()
    );

}

#[tokio::test]
async fn conn_mssql() {
    init_logger();

    let user = "sa".to_string();
    let pwd = "Sqlrfy123$".to_string();
    let dbn = "databases_default".to_string();
    let host = "localhost".to_string();
    let port = 1433;

    let cfg = StrConn::
        new(host, port)
        .with_auth(user, pwd)
        .with_db(dbn);

    let connector = MssqlConnector {cfg};
    let result = connector.conn().await;

    assert!(
        result.is_ok(),
        "Failed to connect to SqlServer: {:?}",
        result.err()
    );


}

#[tokio::test]
async fn conn_mongo() {
    init_logger();

    let user = "admin".to_string();
    let pwd = "rfy123$".to_string();
    let dbn = "admin".to_string();
    let host = "localhost".to_string();
    let port = 27017;

    let cfg = StrConn::
    new(host, port)
    .with_auth(user, pwd)
    .with_db(dbn);

    let connector = MongoConnector {cfg};
    let result = connector.conn().await;

    assert!(
        result.is_ok(),
        "Failed to connect to Mongodb: {:?}",
        result.err()
    );

}


#[tokio::test]
async fn conn_cassandra() {
    init_logger();


    let host = "127.0.0.1".to_string();
    let port = 9042;

    let cfg = StrConn::new(host, port);

    let connector = CSDRConnector {cfg};
    let result = connector.conn().await;

    assert!(
        result.is_ok(),
        "Failed to connect to Cluster assandra: {:?}",
        result.err()
    );

}