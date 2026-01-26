use mongodb::{
    Client, 
    bson::doc, 
    options::ClientOptions
};

use crate::odbc::{StrConn, Connect};

pub struct MongoConnector {pub cfg: StrConn}

impl Connect for MongoConnector {
    async fn conn(&self) -> Result<(), > {
        let uri = &self.cfg.uri_parameters("mongodb");
        let mut client_options = ClientOptions::parse(uri).await?;

        let client = Client::with_options(client_options)?;
        client.database("admin").run_command(doc! {"ping":1}).await?;

        Ok(())

    }
}