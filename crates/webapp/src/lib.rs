use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use std::future::Future;

type WhoisICANN = String;

#[derive(Clone)]
pub struct WhoisOpt {
    whois_server: &'static str,
    domain2lookup: &'static str
}

#[derive(Clone)]
pub struct Whois{
    target: WhoisOpt
}
pub trait WhoisResolver: Sized {
    type Error;
    fn new(opt: WhoisOpt) -> Whois;
    fn query(&self) -> impl Future<Output = Result<WhoisICANN, Self::Error>>;
    fn lookup(&self, whois_server: &str, domain2_lookup: &str) -> impl Future<Output = Result<String, Self::Error>>;
}

impl WhoisResolver for Whois {
    type Error = Box<dyn std::error::Error>;
    fn new(opt: WhoisOpt) -> Whois {
        Whois{target: opt}
    }
    
    async fn query(&self) -> Result<WhoisICANN, Self::Error> {
        let q1 = self.lookup(self.target.whois_server, self.target.domain2lookup).await?;
        let main_server = 
        if let Some((_, b)) = q1.split_once("whois:") {
            b.trim().split_once("\n").ok_or_else(|| errors::WhoisError::MissingNewline)?.0
        } else { return Err(Box::new(errors::WhoisError::GeneralErr { ctx: "could not find whois server to lookup" }));};

        let port: &str = self.target.whois_server.split_once(":").ok_or_else(|| {
            Box::new(errors::WhoisError::GeneralErr{ ctx: "whois server parameter should be a host:port" })
        })?.1;
        Ok(self.lookup(&format!("{main_server}:{port}"), self.target.domain2lookup).await?)
    }

    async fn lookup(&self, whois_server: &str, domain2_lookup: &str) -> Result<String, Self::Error> {
        let mut conn = TcpStream::connect(whois_server).await?;
        conn.write(format!("{domain2_lookup}\r\n").as_bytes()).await?;
        
        let mut data: Vec<u8> = vec![];
        conn.read_to_end(&mut data).await?;
        
        if data.len() == 0 {
            return Err(Box::new(errors::WhoisError::WhoisServerIO { ctx: "Wrote to WHOIS server, but got no response" }));
        }
        Ok(String::from_utf8(data)?)
    }
}

pub mod errors {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum WhoisError {
        #[error("Error caused by I/O on the WHOIS server: {ctx}")]
        WhoisServerIO{ctx: &'static str},
        
        #[error("error: {ctx}")]
        GeneralErr{ctx: &'static str},

        #[error("couldn't find newline seperator")]
        MissingNewline
    }
}

#[tokio::test]
async fn test_client() {
    let client = Whois::new(WhoisOpt{
        whois_server: "whois.iana.org:43", 
        domain2lookup: "simpaix.net"
    });
    let res = client.query().await.unwrap();
    println!("icann info: {}", res);
}