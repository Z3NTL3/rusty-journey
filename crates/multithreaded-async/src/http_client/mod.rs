pub use self::errors::EmptyBody;
use std::{error::Error, sync::Arc};
use errors::{IPTrans, NoHostPort};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use tokio_rustls::{rustls::{self, pki_types::ServerName}, TlsConnector};
use url::Url;
use async_std_resolver::AsyncStdResolver;


pub type Response = String;

pub struct HttpClient {
    resolver: AsyncStdResolver,
    config: Arc<rustls::ClientConfig>
}

impl HttpClient {
    pub fn new(resolver: AsyncStdResolver) -> HttpClient{
        let root_store = rustls::RootCertStore {
            roots: webpki_roots::TLS_SERVER_ROOTS.to_vec(),
        };

        HttpClient{
            resolver,
            config: Arc::new(
                rustls::ClientConfig::builder()
                    .with_root_certificates(root_store)
                    .with_no_client_auth()
            )   
        }
    }

    // + sync is needed, otherwise ``?`` error propogation wont work
    // 
    // the reason for that is:
    // the From trait has no auto impl for box<dyn error> + send
    //
    // it has for:
    // for box<dyn error> + send + sync
    pub async fn https_get(&self, url: &str) -> Result<Response, Box<dyn Error + Send + Sync>> {
        let uri = Url::parse(url)?;
        
        let host = uri.host_str().unwrap_or("");
        let port = uri.port().unwrap_or(443);

        if host.is_empty() {
            return Err(Box::new(NoHostPort));
        }
        
        let mut ip = String::default();

        // ip translation
        if let Some(addr) = self.resolver.ipv4_lookup(host).await?.iter().next() {
            ip = addr.to_string();
        }

        if ip.is_empty() {
            return Err(Box::new(IPTrans));
        }
           
        let connector = TlsConnector::from(self.config.clone());
        let dnsname = ServerName::try_from(host.to_owned())?;
        
        let mut tls_conn = connector.connect(
            dnsname, 
            TcpStream::connect((ip, port)).await?
        ).await?;

        let request = format!(
            "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
            uri.path(),
            host
        );

        tls_conn.write(request.as_bytes()).await?;
        
        let mut body: String = String::from("");
        if tls_conn.read_to_string(&mut body).await? == 0 { 
            return Err(Box::new(EmptyBody));
        }
        
        Ok(body)
    }
}


pub mod errors {
    use std::fmt::{self};
    use std::error::Error;

    #[derive(Debug)]
    pub struct EmptyBody;

    impl fmt::Display for EmptyBody {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "got empty body")
        }
    }

    impl Error for EmptyBody {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            None
        }
    }


    #[derive(Debug)]
    pub struct NoHostPort;

    impl fmt::Display for NoHostPort {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "host and/or port cannot be empty")
        }
    }

    impl Error for NoHostPort {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            None
        }
    }

    #[derive(Debug)]
    pub struct IPTrans;

    impl fmt::Display for IPTrans {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "ip translation failed")
        }
    }

    impl Error for IPTrans {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            None
        }
    }

}