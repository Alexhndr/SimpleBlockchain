use url::{Url, ParseError};

pub fn net_location_by_url(url: &str) -> Result<String, ParseError> {
    let parsed_url = Url::parse(url)?;
    let net_location = format!("{}:{}", parsed_url.host_str().unwrap_or_default(),
        parsed_url.port().unwrap_or_default());
    Ok(net_location)
}
