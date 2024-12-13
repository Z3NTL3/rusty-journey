pub use self::errors::{HostError, AddrEmpty};
use std::{
    error::Error,  
    net::TcpStream, 
    time::Duration
};
use url::Url;

use trust_dns_resolver::Resolver;
pub type Body<'a> = &'a str;

pub struct HttpClient {
    timeout: Duration, 
    resolver: Resolver
}

impl HttpClient {
    fn new(timeout: Duration, resolver: Resolver) -> HttpClient{
        HttpClient{
            timeout,
            resolver
        }
    }

    fn http_get(&self, url: &str) -> Result<Body<'_>, Box<dyn Error>>{
        let uri = Url::parse(url)?;
        let conn: TcpStream;
        

        match uri.host_str() {
            Some(host) => {
                let mut addr: String = String::from("");
                if let Some(ip) = self.resolver.lookup_ip(host)?.iter().next() {
                    addr = ip.to_string();
                }

                if addr.is_empty() {
                    return Err(Box::new(AddrEmpty));
                }

                conn = TcpStream::connect((addr, 80))?;
            }
            None => {
                return Err(Box::new(HostError));
            }
        }

        Ok("")
    }
}


pub mod errors {
    use std::fmt::{self, *};
    use std::error::Error;

    #[derive(Debug)]
    pub struct HostError;

    impl Display for HostError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "host string invalid")
        }
    }

    impl Error for HostError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            None
        }
    }

    #[derive(Debug)]
    pub struct AddrEmpty;

    impl Display for AddrEmpty {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "host string invalid")
        }
    }

    impl Error for AddrEmpty {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            None
        }
    }
}