use std::future::{Future};
use std::process::Output;
use reqwest::{get, Client, Method, Request, Url, Response, Error};
use async_trait::async_trait;

const USER_AGENT: &str = "Mozilla/5.0 (Linux x86_64; rv:100.0) Gecko/20100101 Firefox/100.0";

#[async_trait]
pub trait ResourceMarket {
    fn new(resource: &str) -> Box<Self> where Self: Sized;
    async fn resource_exists(&self, client: &Client) -> Result<bool, Error> {
        Ok(false)
    }
}

pub struct SpigotMC {
    pub resource: String,
}

pub struct McMarket {
    pub resource: String,
}

pub struct Polymart {
    pub resource: String,
}

#[async_trait]
impl ResourceMarket for SpigotMC {
    fn new(resource: &str) -> Box<SpigotMC> {
        Box::from(SpigotMC {
            resource: resource.to_string(),
        })
    }

    async fn resource_exists(&self, client: &Client) -> Result<bool, Error> {
        let url = Url::parse(&format!(
            "https://api.spiget.org/v2/resources/{}",
            self.resource
        )).expect("Failed to parse URL");

        match client.get(url).header("User-Agent", USER_AGENT).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(e) => Err(e),
        }
    }
}

#[async_trait]
impl ResourceMarket for McMarket {
    fn new(resource: &str) -> Box<McMarket> {
        Box::from(McMarket {
            resource: resource.to_string(),
        })
    }
}

#[async_trait]
impl ResourceMarket for Polymart {
    fn new(resource: &str) -> Box<Polymart> {
        Box::from(Polymart {
            resource: resource.to_string(),
        })
    }
}

//pub trait SpigotMC: ResourceMarketStruct where Self: ResourceMarketTrait {
//    type ResourceMarketTrait;
//}
//
//impl dyn SpigotMC {
//    pub fn new(resource: String) -> Self::ResourceMarketTrait {
//        ResourceMarketStruct { resource }
//    }
//
//    pub fn resource_exists(&self) -> bool {

//}
