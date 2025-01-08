#![crate_type = "lib"]

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use std::future::Future;

#[cfg(feature = "parser")]
pub mod parser;

#[derive(Clone)]
// Configuration for your WHOIS instance
pub struct WhoisOpt {
    whois_server: &'static str,
    domain2lookup: &'static str
}

#[derive(Clone)]
// Whois instance, used for querying a domain to a specific WHOIS server for WHOIS data.
//
// # Examples
// ```
//
// #[tokio::test]
// async fn test_client() {
//     let client = Whois::new(WhoisOpt{
//         whois_server: "whois.iana.org:43", 
//         domain2lookup: "simpaix.net"
//     });
//     let res = client.query().await.unwrap();
//     let parser = parser::Parser::new();
//     let info = parser.parse(res).unwrap();
//
//     println!("{:?}", info); // info.registry_domain_id , etc etc
// 
// }
// ```
pub struct Whois{
    target: WhoisOpt
}

pub trait WhoisResolver: Sized {
    type Error;

    // creates a new whois instance and configures the target
    fn new(opt: WhoisOpt) -> Whois;
    // Queries the WHOIS server and retrieves domain information.
    // Returns WHOIS information as a string.
    //
    // So that you can use any arbitrary parser.
    fn query(&self) -> impl Future<Output = Result<String, Self::Error>>;
}

impl WhoisResolver for Whois {
    type Error = Box<dyn std::error::Error>;
    //cool
    fn new(opt: WhoisOpt) -> Whois {
        Whois{target: opt}
    }
    
    async fn query(&self) -> Result<String, Self::Error> {
        let q1 = Whois::lookup(self.target.whois_server, self.target.domain2lookup).await?;
        let main_server = 
        if let Some((_, b)) = q1.split_once("whois:") {
            b.trim().split_once("\n").ok_or_else(|| errors::WhoisError::MissingNewline)?.0
        } else { return Err(Box::new(errors::WhoisError::GeneralErr { ctx: "could not find whois server to lookup" }));};

        let port: &str = self.target.whois_server.split_once(":").ok_or_else(|| {
            Box::new(errors::WhoisError::GeneralErr{ ctx: "whois server should be in host:port format" })
        })?.1;
        Ok(Whois::lookup(&format!("{main_server}:{port}"), self.target.domain2lookup).await?)
    }
}

impl Whois {
    // private
    // Sends a query request to the WHOIS server and returns a String that holds WHOIS information
    async fn lookup(whois_server: &str, domain2_lookup: &str) -> Result<String, Box<dyn std::error::Error>> {
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

// Errors that may occur for parent module
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
