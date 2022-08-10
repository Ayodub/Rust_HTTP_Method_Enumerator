extern crate clap;

#[macro_use]
extern crate colour;

use clap::{Arg, App};
use std::fs::File;
use std::io::{ prelude::*, BufReader};
use reqwest::{Client, Method, header};
use tokio;
use regex::Regex;

#[derive(Clone)]
struct UrlStruct {
    url: String,
    body: Option<String>,
    cookie: Option<String>
}
struct Result {
    method: Method,
    status: u16,
    response: String
}
struct ResultsStruct {
    url: String,
    results: Vec<Result>
}

fn remove_new_line(st:String ) -> String{
    let re = Regex::new(r"\n").unwrap();
    let result = re.replace_all(&st, "");
    result.to_string()
}
fn print_output(results: Vec<ResultsStruct>) {
    for result in results {
        dark_cyan_ln!("URL - {}", result.url);
        dark_magenta_ln!(
            "{0: <10} | {1: <10} | {2: <10}",
            "Method", "Status Code", "Response"
        );
        for r in result.results {
            if r.status < 300 {
                green_ln!("{0: <10} | {1: <10} | {2: <10}", r.method, r.status, r.response);
            } else {
                red_ln!("{0: <10} | {1: <10} | {2: <10}", r.method, r.status, r.response);
            }
        }
    }

}

async fn http_req(client: Client, url_item: UrlStruct, method: Method) -> Result {
    let response;
    if  (url_item.cookie.is_some() && url_item.body.is_some()) && (method == Method::POST || method == Method::PUT || method == Method::PATCH){
        response = client
            .request(method.clone(), &url_item.url)
            .header(header::COOKIE, url_item.cookie.unwrap())
            .body(url_item.body.unwrap())
            .send()
            .await
            .unwrap();
    }
    else if url_item.cookie.is_some() {
        response = client
            .request(method.clone(), &url_item.url)
            .header(header::COOKIE, url_item.cookie.unwrap())
            .send()
            .await
            .unwrap();

    }
    else if url_item.body.is_some() && (method == Method::POST || method == Method::PUT || method == Method::PATCH) {
        response = client
            .request(method.clone(), &url_item.url)
            .body(url_item.body.unwrap())
            .send()
            .await
            .unwrap();
    } else  {
        response = client
            .request(method.clone(), &url_item.url)
            .send()
            .await
            .unwrap();
    }
    let result = Result {
        method,
        status: response.status().as_u16(),
        response: remove_new_line(response.text().await.unwrap())
    };
    result
}


#[tokio::main]
async fn main() {
    let app = App::new("http-status")
        .version("0.0.1");

    let url_option = Arg::with_name("url")
        .long("url")
        .takes_value(true);

    let body_option = Arg::with_name("body")
        .long("body")
        .takes_value(true);
    let cookie_option = Arg::with_name("cookie")
        .long("cookie")
        .takes_value(true);

    let wordlist_option = Arg::with_name("wordlist")
        .long("wordlist")
        .takes_value(true);


    let app = app.arg(url_option)
        .arg(body_option)
        .arg(cookie_option)
        .arg(wordlist_option);
    let matches = app.get_matches();
    let url = matches.value_of("url");
    let body = matches.value_of("body");
    let cookie = matches.value_of("cookie");
    let wordlist = matches.value_of("wordlist");



    let mut url_list = Vec::<UrlStruct>::new();
    if url.is_some() {
        let mut url_struct = UrlStruct {
            url: url.unwrap().to_string(),
            body: None,
            cookie: None
        };
        if body.is_some() {
            url_struct.body = Some(body.unwrap().to_string());
        }
        if cookie.is_some() {
            url_struct.cookie = Some(cookie.unwrap().to_string());
        }
        url_list.push(url_struct);
    }

    if wordlist.is_some() {
        let wordlist_path = wordlist.unwrap().to_string();
        let file = File::open(wordlist_path).unwrap();
        let reader = BufReader::new(file);
        let mut url_struct:UrlStruct = UrlStruct {
            url: "".to_string(),
            body: None,
            cookie: None
        };
        for line_r in reader.lines() {
            let line = line_r.unwrap();
            if line.starts_with("http") {
                if !url_struct.url.is_empty() {
                    url_list.push(url_struct.clone());
                    url_struct = UrlStruct {
                        url: "".to_string(),
                        body: None,
                        cookie: None
                    };
                };
                url_struct.url = line.to_string();
            };
            if line.starts_with("--body") && !url_struct.url.is_empty() {
                url_struct.body = Some(line.replace("--body ", ""));
            }
            if line.starts_with("--cookie") && !url_struct.url.is_empty() {
                url_struct.cookie = Some(line.replace("--cookie ", ""));
            }
        }
        if !url_struct.url.is_empty() {
            url_list.push(url_struct);
        }
    }

    let allow_methods: Vec<Method> = vec![
        Method::OPTIONS,
        Method::GET,
        Method::POST,
        Method::PUT,
        Method::PATCH,
        Method::DELETE,
        Method::HEAD,
        Method::CONNECT,
        Method::TRACE
    ];

    let mut results_list = Vec::<ResultsStruct>::new();
    let client = reqwest::Client::new();
    for url_item in url_list {
        let mut results = Vec::<Result>::new();
        for allow_method in &allow_methods {
            let client = client.clone();
            results.push(http_req(client, url_item.clone(), allow_method.clone()).await);
        }
        results_list.push(ResultsStruct{ url: url_item.url, results });

    }


    print_output(results_list);
}
