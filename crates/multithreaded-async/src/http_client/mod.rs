pub use self::errors::EmptyBody;
use std::error::Error;
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
pub type Response = String;

pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> HttpClient{
        HttpClient{}
    }

    pub async fn http_get(&self, addr: &str, path: &str, host: &str) -> Result<Response, Box<dyn Error>>{
        let mut conn: TcpStream = TcpStream::connect((addr, 80)).await?;
        
        let request = format!(
            "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
            path,
            host
        );

        conn.write(request.as_bytes()).await?;
        
        let mut body: String = String::from("");
        if conn.read_to_string(&mut body).await? == 0 { 
            return Err(Box::new(EmptyBody));
        }

        Ok(body)
    }
}


pub mod errors {
    use std::fmt::{self, *};
    use std::error::Error;

    #[derive(Debug)]
    pub struct EmptyBody;

    impl Display for EmptyBody {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "got empty body")
        }
    }

    impl Error for EmptyBody {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            None
        }
    }
}