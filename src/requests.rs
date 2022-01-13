use std::fmt::{Display, Formatter, Error};
use reqwest;

use crate::chain;

use chain::Chain;

#[derive(Debug)]
pub enum RequestError {
    RequestError(String),
    ParseError(String),
}

impl Display for RequestError {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match self {
            RequestError::RequestError(url) => write!(fmt, "Can\'t get response from: {}", url),
            RequestError::ParseError(url) => write!(fmt, "Can\'t parse response from: {}", url),
        }
    }
}

pub async fn load_chain(net_location: &str) -> Result<Chain, RequestError> {
    let url = format!("http://{}/chain", net_location);
    let url_for_error = url.clone();
    let body = reqwest::get(url).await;
    
    match body {
        Ok(body) => {
            let chain = body.json::<Chain>().await;
            
            match chain {
                Ok(chain) => Ok(chain),
                Err(_) => {
                    return Err(RequestError::ParseError(url_for_error));
                },
            }
        },
        Err(_) => {
            return Err(RequestError::RequestError(url_for_error));
        },
    }
}
