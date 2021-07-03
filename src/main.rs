use std::path::PathBuf;

use clap::{crate_version, Clap};
use futures::executor::block_on;
use futures::prelude::*;
use libp2p::ping::{Ping, PingConfig};
use libp2p::swarm::Swarm;
use libp2p::{floodsub, identity, Multiaddr, PeerId, Transport};
use log::info;
use pretty_env_logger;
use std::task::Poll;

pub mod messages;
pub mod schema;

type Result<T> =
    std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[derive(Clap)]
#[clap(version = crate_version!())]
pub struct Opts {
    data_filepath: PathBuf,
    remote_peer: Option<String>,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    let local_keys: identity::Keypair = identity::Keypair::generate_ed25519();
    let local_peer_id: PeerId = PeerId::from(local_keys.public());
    let topic: floodsub::Topic = floodsub::Topic::new("recipes".to_string());

    pretty_env_logger::init();

    info!("Peer ID: {}", local_peer_id.clone());

    let transport = block_on(libp2p::development_transport(local_keys))?;

    let behaviour = Ping::new(PingConfig::new().with_keep_alive(true));

    let mut swarm = Swarm::new(transport, behaviour, local_peer_id);

    info!("Listening...");
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    if let Some(addr) = opts.remote_peer {
        let remote: Multiaddr = addr.parse()?;
        info!("Dialling remote peer at {}...", remote.clone());
        swarm.dial_addr(remote)?;
    }

    let mut listening = false;
    block_on(future::poll_fn(move |cx| loop {
        match swarm.poll_next_unpin(cx) {
            Poll::Ready(Some(event)) => info!("{:?}", event),
            Poll::Ready(None) => return Poll::Ready(()),
            Poll::Pending => {
                if !listening {
                    for addr in Swarm::listeners(&swarm) {
                        info!("Listening on {}", addr);
                        listening = true;
                    }
                }
                return Poll::Pending;
            }
        }
    }));

    Ok(())
}
