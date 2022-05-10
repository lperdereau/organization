use serde::Deserialize;
use reqwest::Url;

#[derive(Deserialize, Debug)]
pub struct Response {
    pub results: Vec<Organiaztion>,
    pub total_pages: i64,
}

#[derive(Deserialize, Debug)]
pub struct Organiaztion {
    pub id: uuid::Uuid,
    pub name: String,
}

pub fn list(endpoint_url: Url) -> Result<Response, Box<dyn std::error::Error>>{
    debug!("Fetch {endpoint_url}");
    let body: Response = reqwest::blocking::get(endpoint_url)?
        .json::<Response>()?;
    Ok(body)
}

pub fn get(endpoint_url: Url, id : uuid::Uuid) -> Result<Organiaztion, Box<dyn std::error::Error>>{
    let endpoint = endpoint_url.join(&id.to_string())?;
    debug!("Fetch {endpoint}");
    let body: Organiaztion = reqwest::blocking::get(endpoint)?
        .json::<Organiaztion>()?;
    Ok(body)
}
