// ---------------------------------------------------------------------
// connection manager for the databases supported
// ----------------------------------------------------------------------

/* Author: Yury - 11/01/2025 */

pub mod psql;
pub mod mssql;
pub mod mongo;
pub mod csdr;
pub mod dyndb;

pub mod errors;

use errors::ErrorIn;

// =========================== CONFIG ==================

#[derive(Debug)]
pub struct StrConn {
    pub user: Option<String>,
    pub pwd: Option<String>,
    pub dbn: Option<String>,
    pub host: String,
    pub port: u16,
    pub acs_key: Option<String>,
    pub scr_key: Option<String>,
    pub timeout: u8,
}

impl StrConn {
    pub fn new(
        host: String, 
        port: u16
    ) -> Self {
        StrConn {
            user: None,
            pwd: None,
            dbn: None,
            acs_key: None,
            scr_key: None,
            host,
            port,
            timeout: 5,
        }
    }

    pub fn with_auth(
        mut self,
        user: String, 
        pwd: String,    
    ) -> Self {
        self.user = Some(user.into());
        self.pwd = Some(pwd.into());
        self
    }

    pub fn with_db(mut self, dbn: String,) -> Self {
        self.dbn = Some(dbn.into());
        self
    }

    fn parameters(&self) -> String {
        let mut params = Vec::new();

        if let Some(user) = &self.user {
            params.push(format!("user={}", user));
        }

        if let Some(pwd) = &self.pwd {
            params.push(format!("pwd={}", pwd));
        }

        if let Some(dbn) = &self.dbn {
            params.push(format!("dbname={}", dbn));
        }
        
        params.join(" ")
    }

    fn uri_parameters(&self, protocol: &str) -> String {
        match (&self.user, &self.pwd) {
            (Some(user), Some(pwd)) => format!(
                "{}://{}:{}@{}:{}",
                protocol, user, pwd, self.host, self.port
            ),
            _ => format!(
                "{}://{}:{}",
                protocol, self.host, self.port
            ),
        }
    }

    fn user(&self) -> Option<&str> {
        self.user.as_deref()
    }

    fn pwd(&self) -> Option<&str> {
        self.pwd.as_deref()
    }
    
    fn dbn(&self) -> Option<&str> {
        self.dbn.as_deref()
    }

    fn acs_key(&self) -> Option<&str> {
        self.acs_key.as_deref()
    }

    fn scr_key(&self) -> Option<&str> {
        self.scr_key.as_deref()
    }

   fn host(&self) -> &str {
        &self.host
    }

    fn port(&self) -> u16 {
        self.port
    }
}


// =========================== INTERFACE ==================

pub trait Connect {
    async fn conn_psql(&self) -> Result<tokio_postgres::Client, ErrorIn>;
    
}