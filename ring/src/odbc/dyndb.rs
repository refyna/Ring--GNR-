use aws_sdk_dynamodb::{
    Client, 
    config::Builder,
    config::Credentials,
};
use aws_config::{defaults, BehaviorVersion};

use crate::odbc::{Connect, DbInfo, NotifyInfo, NotifyError, StrConn};

pub struct ConnectDyndb {pub cfg: StrConn}

impl Connect for ConnectDyndb {
    async fn conn(&self) -> Result<()> {
        let ac_key = self.cfg.acs_key().expect("dynamodb requires key access");
        let sc_key = self.cfg.scr_key().expect("dynamodb requires secret key access");

        let config = defaults(BehaviorVersion::latest())
            .credentials_provider(Credentials::new(
                    ac_key,
                    sc_key,
                    None,
                    None,
                    "custom",
                )
            )
            .load()
            .await;
        let config_dyndb = Builder::from(&config).build();
        
        let client = Client::from_conf(config_dyndb);
        let list_resp = client.list_tables().send().await;
        match list_resp {
            Ok(resp) => {
                DbInfo::Tables{ 
                        message: resp.table_names().join(" "), 
                        source: None,
                    }
                    .notify("info tables dynamo");
            }
            Err(err) => {
                DbError::Connection { 
                    message: err.to_string(), 
                    source: Some(Box::new(err)) 
                }
                .notify("test tables conn dynamo");
            }
        }

        Ok(())
    }
}