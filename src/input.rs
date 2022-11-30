use reqwest;
use reqwest::cookie::Jar;
use reqwest::Url;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

pub fn get(url: &str) -> String {
    let cookie = "session=53616c7465645f5fa22636e4ee445ab2219c5de2df6dcdc1c879aeb9d4d1e770b70f91f285b6283587ec82b23e5c2cb819fcf67e6c06cb48302d4b8566a61755; Expires=null; Domain=adventofcode.com";
    let durl = "https://adventofcode.com".parse::<Url>().unwrap();

    let jar = Jar::default();
    jar.add_cookie_str(cookie, &durl);
    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .cookie_provider(Arc::new(jar))
        .build()
        .unwrap();
    return client.get(url).send().unwrap().text().unwrap();
}

pub fn get_advent_by_day(day: &str) -> String {
    let path = &("./src/input/".to_owned() + day + ".txt");
    if !Path::new(path).exists() {
        let url = "https://adventofcode.com/2021/day/".to_owned() + day + "/input";
        let mut file = File::create(path).unwrap();
        file.write_all(get(url.as_str()).as_bytes()).expect("msg");
    }
    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).expect("msg");
    return contents;
}
