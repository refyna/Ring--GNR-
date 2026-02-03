use tokio::spawn;
use tokio_postgres::{NoTls, Client};

use crate::odbc::{StrConn, Connect, ErrorIn};

pub struct PsqlConnector {pub cfg: StrConn}

impl Connect for PsqlConnector {
    async fn conn_psql(&self) -> Result<Client, ErrorIn> {
        let (client, connection) = 
            tokio_postgres::connect(&self.cfg.parameters(), NoTls)
            .await
            .map_err(|e| ErrorIn::new("psql connection", e))?;
        
        spawn(async move {
            if let Err(e) = connection.await{                
                //
            }
        });

        Ok(client)
    }
}