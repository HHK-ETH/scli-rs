use phf::phf_map;

#[derive(Debug)]
pub enum NetworkName {
    Ethereum,
    Polygon,
    Avalanche,
    Arbitrum,
    Bsc,
    Celo,
    Fantom,
    Gnosis,
    Moonbeam,
    Moonriver,
    Harmony,
    ArbitrumNova,
    Boba,
    BobaAvax,
    BobaBnb,
    Bttc,
    Metis,
    Optimism,
    Kava,
}

#[derive(Debug)]
pub struct Network<'a> {
    pub name: NetworkName,
    pub chain_id: u32,
    pub rpc: &'a str,
}

pub static NETWORKS: phf::Map<&'static str, Network> = phf_map! {
    "ethereum" => Network {
        name: NetworkName::Ethereum,
        chain_id: 1,
        rpc: "https://eth.public-rpc.com"
    },
    "arbitrum" => Network {
        name: NetworkName::Arbitrum,
        chain_id: 42161,
        rpc: "https://arb1.arbitrum.io/rpc"
    },
    "polygon" => Network {
        name: NetworkName::Polygon,
        chain_id: 137,
        rpc: "https://polygon-rpc.com"
    },
    "fantom" => Network {
        name: NetworkName::Fantom,
        chain_id: 250,
        rpc: "https://rpc.ftm.tools"
    },
    "gnosis" => Network {
        name: NetworkName::Gnosis,
        chain_id: 100,
        rpc: "https://gnosis.public-rpc.com"
    },
    "boba" => Network {
        name: NetworkName::Boba,
        chain_id: 288,
        rpc: "https://mainnet.boba.network"
    },
    "avalanche" => Network {
        name: NetworkName::Avalanche,
        chain_id: 43114,
        rpc: "https://avalanche.public-rpc.com"
    },
    "celo" => Network {
        name: NetworkName::Celo,
        chain_id: 42220,
        rpc: "https://rpc.ankr.com/celo"
    },
    "bsc" => Network {
        name: NetworkName::Bsc,
        chain_id: 56,
        rpc: "https://bscrpc.com"
    },
    "harmony" => Network {
        name: NetworkName::Harmony,
        chain_id: 1666600000,
        rpc: "https://harmony.public-rpc.com"
    },
    "kava" => Network {
        name: NetworkName::Kava,
        chain_id: 2222,
        rpc: "https://evm2.kava.io"
    },
    "metis" => Network {
        name: NetworkName::Metis,
        chain_id: 1088,
        rpc: "https://andromeda.metis.io/?owner=1088"
    },
    "optimism" => Network {
        name: NetworkName::Optimism,
        chain_id: 10,
        rpc: "https://mainnet.optimism.io"
    },
    "bttc" => Network {
        name: NetworkName::Bttc,
        chain_id: 199,
        rpc: "https://rpc.bittorrentchain.io"
    },
    "arbitrum_nova" => Network {
        name: NetworkName::ArbitrumNova,
        chain_id: 42170,
        rpc: "https://nova.arbitrum.io/rpc"
    },
    "moonriver" => Network {
        name: NetworkName::Moonriver,
        chain_id: 1285,
        rpc: "https://rpc.api.moonriver.moonbeam.network"
    },
    "moonbeam" => Network {
        name: NetworkName::Moonbeam,
        chain_id: 1284,
        rpc: "https://rpc.api.moonbeam.network"
    },
    "boba_avax" => Network {
        name: NetworkName::BobaAvax,
        chain_id: 43288,
        rpc: "https://avax.boba.network"
    },
    "boba_bnb" => Network {
        name: NetworkName::BobaBnb,
        chain_id: 56288,
        rpc: "https://bnb.boba.network"
    },
};

pub static LEGACY_SUBGRAPH: phf::Map<&'static str, &'static str> = phf_map! {
    "ethereum" => "https://api.thegraph.com/subgraphs/name/sushi-v2/sushiswap-ethereum",
    "arbitrum" => "https://api.thegraph.com/subgraphs/name/sushi-v2/sushiswap-arbitrum",
    "polygon" => "https://api.thegraph.com/subgraphs/name/sushi-v2/sushiswap-polygon",
    "fantom" => "https://api.thegraph.com/subgraphs/name/sushi-v2/sushiswap-fantom",
    "gnosis" => "https://api.thegraph.com/subgraphs/name/sushi-v2/sushiswap-gnosis",
    "boba" => "https://api.thegraph.com/subgraphs/name/sushi-v2/sushiswap-boba",
    "avalanche" => "https://api.thegraph.com/subgraphs/name/sushi-v2/sushiswap-avalanche",
    "celo" => "https://api.thegraph.com/subgraphs/name/sushi-v2/sushiswap-celo",
    "bsc" => "https://api.thegraph.com/subgraphs/name/sushi-v2/sushiswap-bsc",
    "harmony" => "https://api.thegraph.com/subgraphs/name/olastenberg/sushiswap-harmony-fix",
    "arbitrum_nova" => "https://subgraphs.sushi.com/subgraphs/name/sushi-0m/sushiswap-arbitrum-nova",
    "moonriver" => "https://api.thegraph.com/subgraphs/name/sushi-v2/sushiswap-moonriver",
    "moonbeam" => "https://api.thegraph.com/subgraphs/name/sushi-v2/sushiswap-moonbeam",
    "boba_avax" => "https://subgraphs.sushi.com/subgraphs/name/sushi-0m/sushiswap-boba-avax",
    "boba_bnb" => "https://subgraphs.sushi.com/subgraphs/name/sushi-0m/sushiswap-boba-bnb"
};

pub static TRIDENT_SUBGRAPH: phf::Map<&'static str, &'static str> = phf_map! {
    "arbitrum" => "https://api.thegraph.com/subgraphs/name/sushi-v2/trident-arbitrum",
    "polygon" => "https://api.thegraph.com/subgraphs/name/sushi-v2/trident-polygon",
    "avalanche" => "https://api.thegraph.com/subgraphs/name/sushi-v2/trident-avalanche",
    "bsc" => "https://api.thegraph.com/subgraphs/name/sushi-v2/trident-bsc",
    "kava" => "https://pvt.graph.kava.io/subgraphs/name/sushi-v2/trident-kava",
    "metis" => "https://andromeda.thegraph.metis.io/subgraphs/name/sushi-v2/trident-metis",
    "optimism" => "https://api.thegraph.com/subgraphs/name/sushi-v2/trident-optimism",
    "bttc" => "https://subgraphs.sushi.com/subgraphs/name/sushi-v2/trident-bttc"
};

pub const MASTERCHEF_SUBGRAPH: &str =
    "https://api.thegraph.com/subgraphs/name/jiro-ono/masterchef-staging";

pub const MASTERCHEFV2_SUBGRAPH: &str =
    "https://api.thegraph.com/subgraphs/name/sushiswap/master-chefv2";
pub const MASTERCHEFV2_TOKEN: &str = "0xa5e3142b7a5d59f778483a7e0fd3fe4e263388e9";
pub const MASTERCHEF_DUMMY_TOKENS: [&str; 11] = [
    "0xfb736dad22b879f055c7aebf3a2e8a197f923ea1", //reduce
    "0x393b6dc9b00e18314888678721ec0e923fc5f49d", //kava
    "0x11b66abb675b955bd6f066fde849442865c60e29", //op
    "0x47a307e3167820daf22a377d777371753758f59c", //polygon
    "0xcb277e48526f30f625e24850cf293d89301ea470", //bttc
    "0x65550c7f7280579f34999358b5234f45cfadf50f", //boba
    "0x69cb9f3d42cf6e3706d62db661c30d048220637a", //arbi nova
    "0x8f7a3ca0c676cee87c34d5d9c9cab5a51e929984", //kava
    "0xe7e656893030187f1073e5b2d768e3c1e8861f26", //bsc
    "0xdf4395818bb8b1a45d4ca6ac9c685646c517f274", //ftm
    "0x2377e68f0f909e208884e707db4a7cd493911280", //metis
];

pub static MINICHEF_SUBGRAPH: phf::Map<&'static str, &'static str> = phf_map! {
    "polygon" => "https://api.thegraph.com/subgraphs/name/jiro-ono/minichef-staging-updates",
    "gnosis" => "https://api.thegraph.com/subgraphs/name/jiro-ono/gnosis-minichef-staging",
    "arbitrum" => "https://api.thegraph.com/subgraphs/name/jiro-ono/arbitrum-minichef-staging",
    "celo" => "https://api.thegraph.com/subgraphs/name/sushiswap/celo-minichef-v2",
    "moonriver" => "https://api.thegraph.com/subgraphs/name/sushiswap/moonriver-minichef",
    "fuse" => "https://api.thegraph.com/subgraphs/name/sushiswap/fuse-minichef",
    "fantom" => "https://api.thegraph.com/subgraphs/name/sushiswap/fantom-minichef",
    "moonbeam" => "https://api.thegraph.com/subgraphs/name/sushiswap/moonbeam-minichef",
    "kava" => "https://pvt.graph.kava.io/subgraphs/name/sushiswap/kava-minichef",
    "metis" => "https://andromeda.thegraph.metis.io/subgraphs/name/sushiswap/metis-minichef",
    "boba" => "https://api.thegraph.com/subgraphs/name/sushiswap/minichef-boba",
    "arbitrum_nova" => "https://subgraphs.sushi.com/subgraphs/name/sushiswap/minichef-arbitrum-nova",
    "bttc" => "https://subgraphs.sushi.com/subgraphs/name/sushiswap/minichef-bttc"
};

pub static BLOCK_SUBGRAPH: phf::Map<&'static str, &'static str> = phf_map! {
    "ethereum" => "https://api.thegraph.com/subgraphs/name/blocklytics/ethereum-blocks",
    "gnosis" => "https://api.thegraph.com/subgraphs/name/matthewlilley/xdai-blocks",
    "polygon" => "https://api.thegraph.com/subgraphs/name/matthewlilley/polygon-blocks",
    "fantom" => "https://api.thegraph.com/subgraphs/name/matthewlilley/fantom-blocks",
    "bsc" => "https://api.thegraph.com/subgraphs/name/matthewlilley/bsc-blocks",
    "harmony" => "https://api.thegraph.com/subgraphs/name/sushiswap/harmony-blocks",
    "avalanche" => "https://api.thegraph.com/subgraphs/name/matthewlilley/avalanche-blocks",
    "celo" => "https://api.thegraph.com/subgraphs/name/ubeswap/celo-blocks",
    "arbitrum" => "https://api.thegraph.com/subgraphs/name/sushiswap/arbitrum-blocks",
    "moonriver" => "https://api.thegraph.com/subgraphs/name/sushiswap/moonriver-blocks",
    "fuse" => "https://api.thegraph.com/subgraphs/name/sushiswap/fuse-blocks",
    "moonbeam" => "https://api.thegraph.com/subgraphs/name/sushiswap/moonbeam-blocks",
    "optimism" => "https://api.thegraph.com/subgraphs/name/kybernetwork/optimism-blocks",
    "kava" => "https://pvt.graph.kava.io/subgraphs/name/sushiswap/blocks-kava",
    "metis" => "https://andromeda.thegraph.metis.io/subgraphs/name/sushiswap/blocks-metis",
    "arbitrum_nova" => "https://subgraphs.sushi.com/subgraphs/name/sushiswap/blocks-arbitrum-nova",
    "boba" => "https://api.thegraph.com/subgraphs/name/sushiswap/blocks-boba",
    "boba_avax" => "https://subgraphs.sushi.com/subgraphs/name/sushiswap/blocks-boba-avax",
    "boba_bnb" => "https://subgraphs.sushi.com/subgraphs/name/sushiswap/blocks-boba-bnb",
    "bttc" => "https://subgraphs.sushi.com/subgraphs/name/sushiswap/blocks-bttc",
};
