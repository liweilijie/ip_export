use reqwest::blocking;
use std::io::Read;
use regex::Regex;

use std::fs;
use std::io::{self};
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::time::Duration;
use reqwest::header::USER_AGENT;

const DEFAULT_CONNECT_TIMEOUT_SECS: Duration = Duration::from_secs(25);
const DEFAULT_CLIENT_GET_TIMEOUT_SECS: Duration = Duration::from_secs(25);

pub fn parse_ip(content: &str) -> Option<String> {
    let re: Regex;

    if let Ok(r) = Regex::new(r"(?x)([0,1]?\d{1,2}|2([0-4][0-9]|5[0-5]))(\.([0,1]?\d{1,2}|2([0-4][0-9]|5[0-5]))){2}\.\d{1,3}") {

//    if let Ok(r) = Regex::new(r"(?x)([0,1]?\d{1,2}|2([0-4][0-9]|5[0-5]))(\.([0,1]?\d{1,2}|2([0-4][0-9]|5[0-9]))){3}") {
        re = r;
    } else {
        return None;
    }

    if !re.is_match(content) {
        return None;
    }

    if let Some(c) = re.captures(content) {
        Some(String::from(&c[0]))
    } else {
        None
    }
}

pub async fn async_get_ip(user_agent: &str, url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::builder()
        .timeout(DEFAULT_CLIENT_GET_TIMEOUT_SECS)
        .connect_timeout(DEFAULT_CONNECT_TIMEOUT_SECS)
        .build()?;

    let res = client
        .get(url)
        .header(USER_AGENT, user_agent)
        .send()
        .await?;

    // println!("Status: {}", res.status());
    let body = res.text().await?;
    // println!("Body: \n\n{}", body);

    Ok(String::from(body))
}

#[allow(dead_code)]
pub fn blocking_get_ip(url: &str) -> Option<String> {
    // println!("GET: {}", url);
    let client = match blocking::Client::builder()
        .timeout(DEFAULT_CLIENT_GET_TIMEOUT_SECS) // request timeout 10s
        .build() {
        Ok(c) => c,
        Err(_) => return None,
    };
    // let mut res = match blocking::get(url) {
    let mut res = match client.get(url).send() {
        Ok(r) => r,
        Err(e) => {
            println!("error: {}, {}", url, e);
            return None;
        },
    };

    if !res.status().is_success() {
        return None;
    }

    // println!("Status:{}", res.status());

    let mut content = String::new();

    match res.read_to_string(&mut content) {
        Err(e) => {
            println!("read_to_string error: {}", e);
            return None;
        },
        Ok(_) => {},
    }
    Some(content)
}

pub fn read_contents(f: &str) -> io::Result<String> {
    fs::read_to_string(f)
}

pub fn write_contents(p: &str, c: &str) -> io::Result<()> {
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(p);

    match f {
        Ok(mut stream) => {
            stream.write_all(c.as_bytes())?;
        },
        Err(err) => {
            println!("write {} error:{}", p, err);
        }
    }

    Ok(())
}

