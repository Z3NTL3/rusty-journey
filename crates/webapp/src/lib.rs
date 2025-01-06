use errors::WhoisError;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::SocketAddr;
use hickory_client::proto::iocompat::AsyncIoTokioAsStd;
use hickory_client::{client::AsyncClient, tcp::TcpClientStream};
use tokio::net::TcpStream;
use whoisthere::parse_info;
use std::future::Future;
pub use whoisthere::DomainProps;

#[derive(Clone)]
pub struct Whois;
pub trait WhoisResolver: Sized {
    type Error;
    fn new() -> impl Future<Output = Whois>;
    fn query(&mut self, whois_server: &str, domain2_lookup: &str) -> impl Future<Output = Result<DomainProps, Self::Error>>;
}

impl WhoisResolver for Whois {
    type Error = Box<dyn std::error::Error>;
    async fn new() -> Whois {
        Whois
    }
    
    async fn query(&mut self, whois_server: &str, domain2_lookup: &str) -> Result<DomainProps, Self::Error> {
        let mut conn = TcpStream::connect(whois_server).await?;
        conn.write(format!("{whois_server}").as_bytes()).await?;

        let mut buff: Vec<u8> = vec![];
        let n = conn.read(&mut buff).await?;

        if n == 0 {
            return Err(Box::new(WhoisError::WhoisServerIO { ctx: "Wrote to WHOIS server, but got no resposne" }));
        }

        let whois_data = String::from_utf8(buff)?;
        Ok(parse_info(domain2_lookup, &whois_data))
    }
}

pub mod errors {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum WhoisError {
        #[error("Error caused by I/O on the WHOIS server: {ctx}")]
        WhoisServerIO{ctx: &'static str},
    }
}