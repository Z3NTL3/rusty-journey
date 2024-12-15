pub use self::errors::{HostError, AddrEmpty};
use std::{
    error::Error,  
    time::Duration
};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
pub type Body = Vec<u8>;

pub struct HttpClient {
}

impl HttpClient {
    pub fn new() -> HttpClient{
        HttpClient{}
    }

    pub async fn http_get(&self, addr: &str, path: &str, host: &str) -> Result<Body, Box<dyn Error>>{
        let mut conn: TcpStream = TcpStream::connect((addr, 80)).await?;
        let request = format!(
            "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
            path,
            host
        );

        conn.write(request.as_bytes()).await?;
        
        let mut body: Vec<u8> = vec![];

        // read all until EOF
        loop {
            let mut buff: [u8; 1042] = [0; 1042];
            let len = conn.read(&mut buff).await?;    

            if len == 0 {
                break;
            }

            for byte in buff {
                if byte == 0x00 {
                    continue;
                }

                body.push(byte);
            }
        };
        
        Ok(body)
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