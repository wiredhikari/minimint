use minimint::config::{ServerConfig, ServerConfigParams};
use minimint_api::config::GenerateConfig;
use minimint_api::{Amount, PeerId};
use rand::rngs::OsRng;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Options {
    cfg_path: PathBuf,
    nodes: u16,
    hbbft_base_port: u16,
    api_base_port: u16,
    amount_tiers: Vec<Amount>,
}

fn main() {
    let Options {
        cfg_path,
        nodes,
        hbbft_base_port,
        api_base_port,
        amount_tiers,
    } = StructOpt::from_args();
    let mut rng = OsRng::new().unwrap();

    let peers = (0..nodes).map(PeerId::from).collect::<Vec<_>>();
    let max_evil = hbbft::util::max_faulty(peers.len());
    println!(
        "Generating keys such that up to {} peers may fail/be evil",
        max_evil
    );
    let params = ServerConfigParams {
        hbbft_base_port,
        api_base_port,
        amount_tiers,
    };

    let (server_cfg, client_cfg) =
        ServerConfig::trusted_dealer_gen(&peers, max_evil, &params, &mut rng);

    for (id, cfg) in server_cfg {
        let mut path: PathBuf = cfg_path.clone();
        path.push(format!("server-{}.json", id));

        let file = std::fs::File::create(path).expect("Could not create cfg file");
        serde_json::to_writer_pretty(file, &cfg).unwrap();
    }

    let mut client_cfg_file_path: PathBuf = cfg_path;
    client_cfg_file_path.push("federation_client.json");
    let client_cfg_file =
        std::fs::File::create(client_cfg_file_path).expect("Could not create cfg file");

    serde_json::to_writer_pretty(client_cfg_file, &client_cfg).unwrap();
}
