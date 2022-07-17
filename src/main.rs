use ipfs::{Ipfs, IpfsOptions, IpfsPath, TestTypes, UninitializedIpfs/* , PeerId,MultiaddrWithoutPeerId, MultiaddrWithPeerId, Multiaddr */};
use std::{process::exit};
use tokio::io::AsyncWriteExt;
use tokio_stream::StreamExt;

/* fn bootstrap_nodes( peer_id: PeerId, multiaddr: MultiaddrWithoutPeerId) -> MultiaddrWithPeerId {
    MultiaddrWithPeerId {
        peer_id,
        multiaddr,
    }
}

fn withoutpeer( multiaddr: Multiaddr) -> MultiaddrWithoutPeerId {
    MultiaddrWithoutPeerId::try_from(multiaddr).unwrap()
} */

#[tokio::main]
async fn main() {
    // Initialize an in-memory repo and start a daemon.
    let opts = IpfsOptions::inmemory_with_generated_keys();
    let (ipfs, fut): (Ipfs<TestTypes>, _) = UninitializedIpfs::new(opts).start().await.unwrap();

    // Spawn the background task
    tokio::task::spawn(fut);

    // Restore the default bootstrappers to enable content discovery
    ipfs.restore_bootstrappers().await.unwrap();

    /* // Get the PeerId
    let peer_id : PeerId = "12D3KooWHKsRW6YnASgdBombiF2CyJggAJ1R4vzQwPM3JYhFRk3p".parse().unwrap();

    // Get the multiaddress
    let multiaddr: Multiaddr = "/ip4/127.0.0.1/tcp/4001/p2p/".parse().unwrap();

    // Add Boostrapers
    let address_bootstrapper = bootstrap_nodes(peer_id, withoutpeer(multiaddr));
    ipfs.add_bootstrapper(address_bootstrapper.clone()).await.unwrap(); */

    // Get the IPFS FILE
    let path = "/ipfs/QmdeawxhTJkSY8xKte2fzmhn3thHhfZErDjV9TWWf8rAZG"
        .parse::<IpfsPath>()
        .unwrap();

    let stream = ipfs.cat_unixfs(path, None).await.unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        exit(1);
    });

    tokio::pin!(stream);

    let mut stdout = tokio::io::stdout();

    loop {
        match stream.next().await {
            Some(Ok(bytes)) => {
                stdout.write_all(&bytes).await.unwrap();
            }
            Some(Err(e)) => {
                eprintln!("Error: {}", e);
                exit(1);
            }
            None => break,
        }
    }
}