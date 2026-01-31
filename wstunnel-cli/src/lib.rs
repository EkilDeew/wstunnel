use core::time;
use std::net::UdpSocket;
use std::thread::sleep;

use clap::{Error, Parser};
use tokio::runtime;
use tracing_subscriber::EnvFilter;
use wstunnel::config::{Client, Server};
use wstunnel::executor::DefaultTokioExecutor;
use wstunnel::run_client;

pub struct WSTunnel {}

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type WSTunnel;

        #[swift_bridge(init)]
        fn new() -> WSTunnel;

        fn connect_client(&self, endpoint: &str);
    }
}

#[derive(clap::Parser, Debug)]
#[command(author, version, about, verbatim_doc_comment, long_about = None)]
pub struct WSTunnelCommands {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(clap::Subcommand, Debug)]
pub enum Commands {
    Client(Box<Client>),
    Server(Box<Server>),
}

impl WSTunnel {
    fn new() -> Self {
        WSTunnel {}
    }

    fn connect_client(&self, endpoint: &str) {
        let url = format!("wss://{}", endpoint);

        // TODO: Either run this code in it's own thread
        //       or warn the user that this will lock the calling thread.

        let env_filter = EnvFilter::builder().parse("info").expect("");

        let logger = tracing_subscriber::fmt().with_env_filter(env_filter);
        logger.init();

        let client_opts = WSTunnelCommands::parse_from([
            ".",
            "client",
            "--http-upgrade-path-prefix",
            "wg-wstunnel",
            "--websocket-ping-frequency",
            "15",
            "-L",
            "udp://51820:localhost:51820",
            "--tls-ech-enable",
            url.as_str(),
        ]);
        match client_opts.commands {
            Commands::Client(args) => {
                // Since we are not launching from an async function we cannot use #[tokio::main] to get a
                // tokio runtime. So we'll need to create a new one
                if let Ok(threaded_rt) = runtime::Runtime::new() {
                    let thread = threaded_rt.spawn(async { run_client(*args, DefaultTokioExecutor::default()).await });

                    // We need to send any data to the client so that it opens the tunnels before wireguard
                    // can create the interface.
                    // This is needed because else we'll have a DNS loop.
                    // ie. The wg tunnel will open and send DNS request to resolve the wstunnel endpoint.
                    // If the DNS server is only reachable via wireguard this will create a deadlock...
                    let socket = UdpSocket::bind("127.0.0.1:0").expect("Could not bind");
                    sleep(time::Duration::from_secs(1));
                    socket.send_to(b"", "127.0.0.1:51820").expect("Could not send packet");

                    // Seems like calling the run_client directly in block_on doesn't work for some reason?
                    // Tokio complains that we are not in a tokio runtime...
                    _ = threaded_rt.block_on(thread);
                }
            }
            Commands::Server(_) => {}
        }
    }
}
