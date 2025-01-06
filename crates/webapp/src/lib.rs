use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use whoisthere::parse_info;
use std::future::Future;
pub use whoisthere::DomainProps;

#[derive(Clone)]
pub struct Whois;
pub trait WhoisResolver: Sized {
    type Error;
    fn new() -> Whois;
    fn query(&self, whois_server: &str, domain2_lookup: &str) -> impl Future<Output = Result<DomainProps, Self::Error>>;
}

impl WhoisResolver for Whois {
    type Error = Box<dyn std::error::Error>;
    fn new() -> Whois {
        Whois
    }
    
    async fn query(&self, whois_server: &str, domain2_lookup: &str) -> Result<DomainProps, Self::Error> {
        let mut conn = TcpStream::connect(whois_server).await?;
        conn.write(format!("{domain2_lookup}\r\n").as_bytes()).await?;
        
        let mut data: Vec<u8> = vec![];
        loop {
            let mut buff: Vec<u8> = Vec::with_capacity(1042);
            let n = conn.read_exact(&mut buff).await?;

            if n == 0 {
                break;
            }

            data.append(&mut buff);
        }
        
        if data.len() == 0 {
            return Err(Box::new(errors::WhoisError::WhoisServerIO { ctx: "Wrote to WHOIS server, but got no response" }));
        }

        let whois_data = String::from_utf8(data)?;
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

#[tokio::test]
async fn test_client() {
    let client = Whois::new();
    let res = client.query("whois.iana.org:43", "simpaix.net").await.unwrap();
    println!("domain name: {}, exp: {} etc...", res.domain_name, res.expiration_date);
}