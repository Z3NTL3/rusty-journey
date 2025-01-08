use std::{fmt::Debug, str::FromStr};

pub use chrono::{DateTime, Utc};


#[derive(Debug)]
pub struct WhoisInformation {
    pub domain_name: String,
    pub registry_domain_id: String,
    pub registrar_whois_server: String,
    pub registrar_url: String,
    pub updated_date: DateTime<Utc>, // whois datetimes are expressed in UTC
    pub creation_date: DateTime<Utc>,
    pub registry_expirity_date: DateTime<Utc>,
    pub registrar: String,
    pub registrar_iana_id: String,
    pub registrar_abuse_email_contact: String,
    pub registrar_abuse_phone_contact: String,
    pub domain_status: String,
    pub nameservers: Vec<String>,
    pub dnssec: String,
}

impl Default for WhoisInformation {
    fn default() -> Self {
        let default_time: DateTime<Utc> =  DateTime::default();
        Self { 
            domain_name: "".into(), 
            registry_domain_id: "".into(), 
            registrar_whois_server: "".into(), 
            registrar_url: "".into(), 
            updated_date: default_time, 
            creation_date: default_time, 
            registry_expirity_date: default_time, 
            registrar: "".into(), 
            registrar_iana_id: "".into(),
            registrar_abuse_email_contact: "".into(), 
            registrar_abuse_phone_contact: "".into(), 
            domain_status: "".into(), 
            nameservers: Vec::default(), 
            dnssec: "" .into()
        }
    }
}

pub struct Parser;

impl Parser {
    pub fn new() -> Parser {
        Parser
    }
    pub fn parse(&self, content: String) -> Result<WhoisInformation, Box<dyn std::error::Error>> {
        let body = content.split("\n");
        let mut parsed = WhoisInformation::default();
        
        let body = body.map(|item| {
            let find =  item.trim().to_lowercase();
            let joints = find.split_once(":").unwrap_or_default();

            if find.contains(&joints.0) {
                Some((joints.0.to_owned(), joints.1.to_owned()))
            } else {
                None
            }
        });

        for item in body {
            match item {
                Some((key, value)) => {
                    // println!("key: {key} - value: {value}");

                    match key.as_str() {
                        "domain name" => parsed.domain_name = value.into(),
                        "registry domain id" => parsed.registry_domain_id = value.into(),
                        "registrar whois server" => parsed.registrar_whois_server = value.into(),
                        "registrar url" => parsed.registrar_url = value.into(),
                        "updated date" => parsed.updated_date = match DateTime::<Utc>::from_str(value.as_str()) {
                            Ok(date) => date,
                            Err(err) => return Err(Box::new(err)),
                        },
                        "creation date" => parsed.creation_date = match DateTime::<Utc>::from_str(value.as_str()) {
                            Ok(date) => date,
                            Err(err) => return Err(Box::new(err)),
                        },
                        "registry expiry date" => parsed.registry_expirity_date = match DateTime::<Utc>::from_str(value.as_str()) {
                            Ok(date) => date,
                            Err(err) => return Err(Box::new(err)),
                        },
                        "registrar" => parsed.registrar = value.into(),
                        "registrar iana id" => parsed.registrar_iana_id = value.into(),
                        "registrar abuse contact email" => parsed.registrar_abuse_email_contact = value.into(),
                        "registrar abuse contact phone" => parsed.registrar_abuse_phone_contact = value,
                        "domain status" => parsed.domain_status = value.into(),
                        "name server" => parsed.nameservers.push(value),
                        "dnssec" => parsed.dnssec = value,
                        _ => ()
                    }
                },
                None => (),
            };
        }
        Ok(parsed)
    }
}
