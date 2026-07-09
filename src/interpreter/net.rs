use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, CONTENT_TYPE};
use crate::engine::value::Value;
use std::thread;
use std::time::Duration;

const MAX_RETRIES: u32 = 3;

fn get_client() -> Client {
    let mut headers = HeaderMap::new();
    // JSONPlaceholder accetta bene questo User-Agent
    headers.insert(USER_AGENT, HeaderValue::from_static("NodeStract-Agent/1.0"));
    
    Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(10))
        // Rimosso .cookie_store(true) per evitare problemi di build, non serve per questo test
        .build()
        .unwrap_or_else(|_| Client::new())
}

fn perform_request<F>(method_name: &str, url: &str, request_fn: F) -> Result<Value, String> 
where F: Fn(&Client) -> Result<reqwest::blocking::Response, reqwest::Error> 
{
    let client = get_client();
    let mut attempt = 0;

    loop {
        attempt += 1;
        match request_fn(&client) {
            Ok(resp) => {
                let status = resp.status();
                
                if status.is_success() {
                    return match resp.text() {
                        Ok(text) => Ok(Value::String(text)),
                        Err(e) => {
                            Err(format!("NET ERROR [{}]: Body read failed. {}", method_name, e))
                        }
                    };
                } 
                
                // Retry su errori server (5xx)
                if (status.is_server_error() || status.as_u16() == 429) && attempt <= MAX_RETRIES {
                    let wait = Duration::from_millis(500 * 2_u64.pow(attempt - 1));
                    thread::sleep(wait);
                    continue;
                }

                return Err(format!("NET ERROR [{}]: HTTP {} on '{}'", method_name, status, url));
            },
            Err(e) => {
                if attempt <= MAX_RETRIES {
                    let wait = Duration::from_millis(500 * 2_u64.pow(attempt - 1));
                    thread::sleep(wait);
                    continue;
                }
                return Err(format!("NET ERROR: {} {} failed after {} attempts. {}", method_name, url, MAX_RETRIES, e));
            }
        }
    }
}

pub fn get(url: &str) -> Result<Value, String> {
    perform_request("GET", url, |client| client.get(url).send())
}

pub fn post(url: &str, body: &str) -> Result<Value, String> {
    let body_string = body.to_string(); 
    perform_request("POST", url, move |client| {
        client.post(url)
            .header(CONTENT_TYPE, "application/json; charset=utf-8") 
            .body(body_string.clone())
            .send()
    })
}