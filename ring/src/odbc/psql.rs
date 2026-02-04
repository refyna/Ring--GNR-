use tokio_postgres::{NoTls, Client};

use crate::odbc::{StrConn, Connect, ErrorIn, info, error};

pub struct PsqlConnector {pub cfg: StrConn}

impl Connect for PsqlConnector {
    async fn conn_psql(&self) -> Result<Client, ErrorIn> {
        let target = "Conn Psql";

        let (client, connection) = tokio_postgres::connect(&self.cfg.parameters(), NoTls)
            .await
            .map_err(|e| ErrorIn::new("psql connection", e))?;
        
        tokio::spawn(async move {
            if let Err(e) = connection.await{                
                error!(target=target, error= %e);
            }
        });
        
        info!(targer=target, "Connected successfully");
        Ok(client)
    }
}