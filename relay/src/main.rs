use libremexre::errors::{log_err, Result};
use stahlnet::Server;
use std::{
    net::{IpAddr, SocketAddr, ToSocketAddrs},
    sync::{Arc, Mutex},
};
use structopt::StructOpt;
use tokio::prelude::*;
use tokio_signal::ctrl_c;

fn main() {
    let opts = Options::from_args();
    libremexre::init_logger(opts.verbosity, opts.quiet);
    if let Err(err) = run(opts) {
        log_err(&*err);
        std::process::exit(1)
    }
}

fn run(opts: Options) -> Result<()> {
    let mut server = Server::new(
        opts.addr(),
        ctrl_c()
            .and_then(|stream| stream.into_future().map_err(|(e, _)| e))
            .then(|_| Ok(())),
    )?;

    opts.peers
        .into_iter()
        .flat_map(|mut addr| match addr.to_socket_addrs() {
            Ok(addrs) => addrs,
            Err(err) => {
                addr.push_str(":51441");
                match addr.to_socket_addrs() {
                    Ok(addrs) => addrs,
                    Err(_) => {
                        log_err(&err);
                        Vec::new().into_iter()
                    }
                }
            }
        })
        .for_each(|peer| server.add_peer(peer));

    let err = Arc::new(Mutex::new(None));
    let err_clone = err.clone();
    tokio::run(server.map_err(move |err| {
        *err_clone.lock().unwrap() = Some(err);
    }));
    let err = err.lock().unwrap().take();
    match err {
        Some(err) => Err(err.into()),
        None => Ok(()),
    }
}

#[derive(Debug, StructOpt)]
struct Options {
    /// The address to serve on.
    #[structopt(short = "a", long = "ip-addr", default_value = "::")]
    ip: IpAddr,

    /// The port to serve on.
    #[structopt(short = "p", long = "port", default_value = "51441")]
    port: u16,

    /// The addresses of known peers.
    #[structopt(long = "peer")]
    peers: Vec<String>,

    /// Disables log output.
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,

    /// Increases the verbosity.
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbosity: usize,
}

impl Options {
    fn addr(&self) -> SocketAddr {
        (self.ip, self.port).into()
    }
}
