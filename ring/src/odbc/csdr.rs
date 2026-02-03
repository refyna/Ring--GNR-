
use cdrs_tokio::{cluster::{NodeTcpConfigBuilder, session::{SessionBuilder, TcpSessionBuilder}}, load_balancing::RoundRobinLoadBalancingStrategy};

use crate::odbc::{StrConn, Connect};


pub struct CSDRConnector {pub cfg: StrConn}

impl Connect for CSDRConnector {
     async fn conn(&self) -> Result<()> {
        let point = format!("{}:{}", &self.cfg.host(), &self.cfg.port());    
        
        let cluster_config = NodeTcpConfigBuilder::new()
            .with_contact_point(point.into())
            .build()
            .await
            .unwrap();

        let session = TcpSessionBuilder::new(RoundRobinLoadBalancingStrategy::new(), cluster_config)
            .build()
            .await
            .unwrap();

        Ok(())

    }
}