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
