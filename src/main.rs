use actix_web::{
    dev,
    http::header::ContentType,
    middleware::{self, Logger},
    web, App, Error, HttpResponse, HttpServer, ResponseError,
};
use rustls::{pki_types::PrivateKeyDer, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::{fmt::Display, fs::File, io::BufReader, net::Ipv4Addr, path::PathBuf};

mod cli;

#[derive(Debug)]
struct AppError {
    message: String,
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl ResponseError for AppError {}

impl AppError {
    pub fn new(message: String) -> Self {
        AppError { message }
    }
}

async fn respond(req: dev::ServiceRequest) -> Result<dev::ServiceResponse, Error> {
    log::info!("{req:#?}");

    let base_dir = req.app_data::<web::Data<PathBuf>>().unwrap();
    let new_path = base_dir.join(&req.path()[1..]);

    if !new_path.exists() {
        return Err(AppError::new(format!("Path {} does not exists", req.path())).into());
    }

    if new_path.is_dir() {
        let entry = std::fs::read_dir(&new_path)
            .into_iter()
            .flat_map(|mut entry| entry.next())
            .flatten()
            .next();
        match entry {
            Some(entry) => {
                let content = std::fs::read(entry.path())?;
                if let Ok(body) = String::from_utf8(content) {
                    Ok(req.into_response(
                        HttpResponse::Ok()
                            .content_type(ContentType::json())
                            .body(body),
                    ))
                } else {
                    Err(AppError::new("Invalid content...".to_string()).into())
                }
            }
            _ => Err(AppError::new(format!("Invalid path {}", req.path())).into()),
        }
    } else {
        Err(AppError::new("Not a directory...".to_string()).into())
    }
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let matches = cli::build_cli();

    let resources_dir = matches.get_one::<PathBuf>("resources").unwrap().clone();
    let path = resources_dir.as_path().display().to_string();

    let tls: &bool = matches.get_one("tls").unwrap();
    let host: &Ipv4Addr = matches.get_one("host").unwrap();
    let http_port: &u16 = matches.get_one("port").unwrap();
    let tls_port: &u16 = matches.get_one("tls-port").unwrap();

    if !resources_dir.exists() {
        panic!(
            "Invalid configuration! Resource path does not exists: {}",
            &path
        );
    }

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .wrap(Logger::default())
            .app_data(web::Data::new(resources_dir.clone()))
            .service(web::service("/{list:.*}").finish(respond))
    });

    let binded = if *tls {
        let addr = format!("{}:{}", host, tls_port);
        log::info!(
            "Https server running on {} with resources from {}",
            addr,
            &path
        );
        server.bind_rustls_0_23(addr, load_rustls_config())
    } else {
        Ok(server)
    };

    log::info!(
        "Http server running on {}:{} with resources from {}",
        host,
        http_port,
        &path
    );

    binded?.bind((*host, *http_port))?.run().await
}

// заимствовано из примера/copied from
// https://github.com/actix/examples/blob/master/https-tls/rustls/src/main.rs
fn load_rustls_config() -> rustls::ServerConfig {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

    // init server config builder with safe defaults
    let config = ServerConfig::builder().with_no_client_auth();

    // load TLS key/cert files
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());

    // convert files to key/cert objects
    let cert_chain = certs(cert_file).collect::<Result<Vec<_>, _>>().unwrap();
    let mut keys = pkcs8_private_keys(key_file)
        .map(|key| key.map(PrivateKeyDer::Pkcs8))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    // exit if no keys could be parsed
    if keys.is_empty() {
        eprintln!("Could not locate PKCS 8 private keys.");
        std::process::exit(1);
    }

    config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}
