pub mod whois {
    use std::net::SocketAddr;
    use std::str::FromStr;
    use hickory_client::client::ClientHandle;
    use hickory_client::proto::iocompat::AsyncIoTokioAsStd;
    use hickory_client::{client::AsyncClient, rr::Name, tcp::TcpClientStream};
    use tokio::net::TcpStream;
    use whoisthere::parse_info;
    use std::future::Future;

    pub use whoisthere::DomainProps;

    #[derive(Clone)]
    pub struct Whois{
        client: AsyncClient
    }
    
    pub trait WhoisResolver: Sized {
        type Error;
        fn new(ns: SocketAddr) -> impl Future<Output = Result<Whois, Self::Error>>;
        fn query(&mut self, target: &str) -> impl Future<Output = Result<DomainProps, Self::Error>>;
    }

    impl WhoisResolver for Whois {
        type Error = Box<dyn std::error::Error>;
        async fn new(ns: SocketAddr) -> Result<Whois, Self::Error> {
            let (stream, sender) =
                TcpClientStream::<AsyncIoTokioAsStd<TcpStream>>::new(ns);
            let (client, bg) = AsyncClient::new(stream, sender, None).await?;
            tokio::spawn(bg);
            
            Ok(Whois { client })
        }
        
        async fn query(&mut self, target: &str) -> Result<DomainProps, Self::Error> {
            let name =  Name::from_str(target);
            if let Err(res) = name {
                return Err::<DomainProps, Self::Error>(Box::new(res));
            }

            let response = self.client.query(
                name.unwrap_or_default(), 
                hickory_client::rr::DNSClass::IN, 
                hickory_client::rr::RecordType::A
            ).await?;

            let parsed  = parse_info(target, response.to_string().as_str());
            Ok(parsed)
        }
    }
}