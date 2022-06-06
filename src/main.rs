mod resource_market;

use crate::resource_market::*;
use clap::{command, Arg, ArgMatches};
use futures::future::join_all;
use futures::stream::{iter, StreamFuture};
use futures::StreamExt;
use octocrab::models::repos::Release;
use octocrab::models::Repository;
use octocrab::{instance, Error};
use std::borrow::Borrow;
use std::future::Future;
use std::io::Read;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command = command!().args([
        Arg::new("spigotmc").takes_value(true).short('c'),
        Arg::new("mcmarket").takes_value(true).short('m'),
        Arg::new("polymart").takes_value(true).short('p'),
        Arg::new("repo").takes_value(true).short('f').required(true),
    ]);
    let mut args = command.get_matches();

    let mut markets: Vec<Box<dyn ResourceMarket>> = Vec::with_capacity(3);

    if let Some(resource) = args.value_of("spigotmc") {
        markets.push(SpigotMC::new(resource));
    }

    if let Some(resource) = args.value_of("mcmarket") {
        markets.push(McMarket::new(resource));
    }

    if let Some(resource) = args.value_of("polymart") {
        markets.push(Polymart::new(resource));
    }

    let client = reqwest::Client::new();

    let failed = markets
        .iter()
        .map(move |market| market.resource_exists(&client).await)
        .collect::<Vec<_>>()
        .iter()
        .any(|result| result == &false);

    if failed {
        eprintln!("Failed to find all resources: {}");
    }

    return Ok(());

    //markets.for_each_concurrent(|m| m.resource_exists()).await;
}

async fn get_repo_and_release(args: &ArgMatches) -> Result<(Repository, Release), Error> {
    let (owner_str, repo_str) = match args.value_of("repo").unwrap().split_once("/") {
        Some(str) => str,
        None => panic!("Invalid repo name"),
    };
    let instance = instance();
    let handler = instance.repos(owner_str, repo_str);
    let releases_handler = handler.releases();
    let repo_future = handler.get();
    let releases_future = releases_handler.get_latest();

    return Ok((repo_future.await?, releases_future.await?));
}

//POST /resources/minix-async-kotlin-performant-coroutines.97423/save-version HTTP/3
//Host: www.spigotmc.org
//User-Agent: Mozilla/5.0 (X11; Linux x86_64; rv:100.0) Gecko/20100101 Firefox/100.0
//Accept: application/json, text/javascript, */*; q=0.01
//Accept-Language: en-GB,en;q=0.5
//Accept-Encoding: gzip, deflate, br
//Content-Type: application/x-www-form-urlencoded; charset=UTF-8
//X-Ajax-Referer: https://www.spigotmc.org/resources/minix-async-kotlin-performant-coroutines.97423/add-version
//X-Requested-With: XMLHttpRequest
//Content-Length: 547
//Origin: https://www.spigotmc.org
//DNT: 1
//Alt-Used: www.spigotmc.org
//Connection: keep-alive
//Referer: https://www.spigotmc.org/resources/minix-async-kotlin-performant-coroutines.97423/add-version
//Cookie: cf_clearance=xDF4.Qn5snon_mTba6md7JJ3tgkysu2wKBNSoWC3F6I-1653051924-0-150; xf_user=748072%2C61e6263f00765884338940d4702f57b7399327c5; xf_session=bd88f2534dcca774444aaa3ddb6f6130; xf_EWRcarta_172=1; xf_EWRcarta_1=1; colorname_c=Wd9dee07811121ac72033e2ea53d339a6S; colorname_e=Wd9dee07811121ac72033e2ea53d339a6S; colorname_p=Wd9dee07811121ac72033e2ea53d339a6S; wmt_secToken=Wd9dee07811121ac72033e2ea53d339a6S
//Sec-Fetch-Dest: empty
//Sec-Fetch-Mode: cors
//Sec-Fetch-Site: same-origin
//TE: trailers
