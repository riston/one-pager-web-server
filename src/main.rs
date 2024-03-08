use std::{
    io::{BufRead, BufReader, ErrorKind},
    process::{Command, Stdio},
};

use actix_web::{http::StatusCode, web, App, HttpRequest, HttpServer, Responder};

async fn health(_req: HttpRequest) -> impl Responder {
    format!("Server health check")
}

async fn execute_method(req: HttpRequest) -> (impl Responder, StatusCode) {
    let raw_method = req.match_info().get("method");
    let method = match raw_method {
        Some("toggle") | Some("on") | Some("off") => raw_method,
        _ => None,
    };

    match method {
        Some("toggle") | Some("on") | Some("off") => {
            let exec_result = execute(method.unwrap()).await;

            match exec_result {
                Ok(_) => (
                    format!("Display method {}!", method.unwrap()),
                    StatusCode::OK,
                ),
                Err(e) => {
                    return (
                        format!("Error executing command: {}", e),
                        StatusCode::INTERNAL_SERVER_ERROR,
                    )
                }
            }
        }
        _ => (
            format!("Invalid method parameter!"),
            StatusCode::BAD_REQUEST,
        ),
    }
}

// Use lsusb to find the device and the corresponding port
// /home/pi/uhubctl/uhubctl -l 1-1 -p 2 -a toggle
// The uhubctl could be replaced with `sudo echo "1" >> /sys/bus/usb/devices/1-1:1.0/1-1-port2/disable`
async fn execute(method: &str) -> std::io::Result<()> {
    let cmd_path_str: &str = "/home/pi/uhubctl/uhubctl";
    let args = ["-l", "1-1", "-p", "2", "-a", method];

    let stdout = Command::new(cmd_path_str)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| {
            std::io::Error::new(ErrorKind::Other, "Could not capture standard output.")
        })?;

    let reader = BufReader::new(stdout);
    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Page server started");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(health))
            .route("/display/{method}", web::get().to(execute_method))
            .default_service(web::route().to(|| actix_web::HttpResponse::NotFound()))
    })
    .bind("[::]:8080")?
    .run()
    .await
}
