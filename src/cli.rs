use clap::{arg, value_parser, Arg, ArgMatches, Command};

use std::{net::Ipv4Addr, path::PathBuf};

pub fn build_cli() -> ArgMatches {
    Command::new("SimpleRestServer")
        .version("0.2.0")
        .about("Simple REST server. Accept any requests and return resource content, if any")
        .arg(
            arg!(-r --resources <DIR> "Sets a custom resource directory")
                .required(false)
                .value_parser(value_parser!(PathBuf))
                .default_value("./resources"),
        )
        .arg(
            arg!(--host <HOST> "Bind to host")
                .required(false)
                .value_parser(value_parser!(Ipv4Addr))
                .default_value("0.0.0.0"),
        )
        .arg(
            arg!(--port <PORT> "Http port")
                .required(false)
                .value_parser(value_parser!(u16))
                .default_value("8080"),
        )
        .arg(arg!(-s --tls "Enable https support").required(false))
        .arg(
            Arg::new("tls-port")
                .value_name("TLS_PORT")
                .short('t')
                .long("tls-port")
                .required(false)
                .default_value("8443")
                .value_parser(value_parser!(u16))
                .help("Https port (only if https server enabled)"),
        )
        .get_matches()
}
