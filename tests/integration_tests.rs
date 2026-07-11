use std::process::Command;
use std::sync::atomic::{AtomicU32, Ordering};
use std::net::TcpListener;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;
use std::sync::Once;

static START_SERVER: Once = Once::new();
static RETRY_COUNT: AtomicU32 = AtomicU32::new(0);

fn start_mock_server() {
    START_SERVER.call_once(|| {
        thread::spawn(|| {
            let listener = match TcpListener::bind("127.0.0.1:12345") {
                Ok(l) => l,
                Err(e) => {
                    eprintln!("[Mock Server] Failed to bind: {}", e);
                    return;
                }
            };
            
            for stream in listener.incoming() {
                if let Ok(mut stream) = stream {
                    let mut buffer = [0; 1024];
                    if stream.read(&mut buffer).is_ok() {
                        let req = String::from_utf8_lossy(&buffer);
                        let response;
                        
                        if req.starts_with("GET /success") {
                            let body = "Hello NodeStract";
                            response = format!(
                                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
                                body.len(),
                                body
                            );
                        } else if req.starts_with("GET /notfound") {
                            response = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n".to_string();
                        } else if req.starts_with("POST /post") {
                            let body = "POST received";
                            response = format!(
                                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
                                body.len(),
                                body
                            );
                        } else if req.starts_with("GET /retry") {
                            let attempts = RETRY_COUNT.fetch_add(1, Ordering::SeqCst);
                            if attempts == 0 {
                                response = "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\n\r\n".to_string();
                            } else {
                                let body = "Retry success";
                                response = format!(
                                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
                                    body.len(),
                                    body
                                );
                            }
                        } else {
                            response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n".to_string();
                        }
                        
                        let _ = stream.write_all(response.as_bytes());
                        let _ = stream.flush();
                    }
                }
            }
        });
        thread::sleep(Duration::from_millis(100));
    });
}

fn run_one_test(path: &str) {
    if path.contains("/net/") {
        start_mock_server();
    }

    let output = Command::new("cargo")
        .args(&["run", "--quiet", "--", "build", path])
        .output()
        .expect("Failed to execute cargo run");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let full_output = format!("{}\n{}", stdout, stderr);

    if !output.status.success() {
        panic!(
            "Process exited with non-zero status.\n--- OUTPUT ---\n{}\n--------------",
            full_output
        );
    }

    if full_output.contains("FAIL") {
        panic!(
            "Test output contains 'FAIL'.\n--- OUTPUT ---\n{}\n--------------",
            full_output
        );
    }
}

// Include all dynamically generated test functions
include!(concat!(env!("OUT_DIR"), "/generated_tests.rs"));
