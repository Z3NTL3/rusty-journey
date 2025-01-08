use std::{fmt::Debug, str::FromStr};

pub use chrono::{DateTime, Utc};


#[derive(Debug, Default)]
pub struct WhoisInformation {
    pub domain_name: Option<String>,
    pub registry_domain_id: Option<String>,
    pub registrar_whois_server: Option<String>,
    pub registrar_url: Option<String>,
    pub updated_date: Option<DateTime<Utc>>, // whois datetimes are expressed in UTC
    pub creation_date: Option<DateTime<Utc>>,
    pub registry_expirity_date: Option<DateTime<Utc>>,
    pub registrar: Option<String>,
    pub registrar_iana_id: Option<String>,
    pub registrar_abuse_email_contact: Option<String>,
    pub registrar_abuse_phone_contact: Option<String>,
    pub domain_status: Option<String>,
    pub name_servers: Option<Vec<String>>,
    pub dnssec: Option<String>,
}

pub struct Parser;

impl Parser {
    // Creates a new parser
    pub fn new() -> Parser {
        Parser
    }
    
    // Parses a WHOIS information from a String into a WhoisInformation struct
    pub fn parse(&self, content: String) -> Result<WhoisInformation, Box<dyn std::error::Error>> {
        let lines = content.split("\n").flat_map(|line| line.split_once(":"));
        let mut whois_information = WhoisInformation::default();

        for (key, value) in lines {
            let key = key.trim();
            let value = value.trim();
            
            match key.to_lowercase().as_str() {
                "domain name" => whois_information.domain_name = Some(value.to_owned()),
                "registry domain id" => whois_information.registry_domain_id = Some(value.to_owned()),
                "registrar whois server" => {
                    whois_information.registrar_whois_server = Some(value.to_owned())
                }
                "registrar url" => whois_information.registrar_url = Some(value.to_owned()),
                "updated date" => whois_information.updated_date = Some(DateTime::<Utc>::from_str(value)?),
            
                "creation date" => whois_information.creation_date = Some(DateTime::<Utc>::from_str(value)?),
                "registry expiry date" => whois_information.registry_expirity_date = Some(DateTime::<Utc>::from_str(value)?),
                "registrar" => whois_information.registrar = Some(value.to_owned()),
                "registrar iana id" => whois_information.registrar_iana_id = Some(value.to_owned()),
                "registrar abuse contact email" => whois_information.registrar_abuse_email_contact = Some(value.to_owned()),
                "registrar abuse contact phone" => whois_information.registrar_abuse_phone_contact = Some(value.to_owned()),
                "domain status" => whois_information.domain_status = Some(value.to_owned()),
                "name server" => match whois_information.name_servers.as_mut() {
                    Some(name_servers) => name_servers.push(value.to_owned()),
                    None => whois_information.name_servers = Some(Vec::new()),
                },
                "dnssec" => whois_information.dnssec = Some(value.to_owned()),
                _ => {}
            }
        }
        Ok(whois_information)
    }
}
