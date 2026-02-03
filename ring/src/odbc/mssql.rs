use tiberius::{Client, Config, AuthMethod};
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncWriteCompatExt;

use crate::odbc::{DbError, StrConn, Connect};

pub struct MssqlConnector {pub cfg: StrConn}

impl Connect for MssqlConnector {
    async fn conn(&self) -> Result<(), DbError> {
        let mut config = Config::new();
        let user = self.cfg.user()
            .expect("SQL Server requires user");

        let pwd = self.cfg.pwd()
            .expect("SQL Server requires password");

        config.host(&self.cfg.host());
        config.port(self.cfg.port());
        config.authentication(AuthMethod::sql_server(
            user, pwd,
            )
        );
        config.trust_cert(); // Remove in production, because it is necessary authenticate the server wich you are connecting
        
        let tcp = TcpStream::connect(config.get_addr()).await?;
        tcp.set_nodelay(true)?;

        let _ = Client::connect(config, tcp.compat_write()).await?;

        Ok(())
    }
}