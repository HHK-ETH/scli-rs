use phf::phf_map;

#[derive(Debug)]
pub enum NetworkName {
    ETHEREUM,
    POLYGON,
    AVALANCHE,
    ARBITRUM,
    BSC,
    CELO,
    FANTOM,
    FUSE,
    GNOSIS,
    MOONBEAM,
    MOONRIVER,
    HARMONY,
    ARBITRUM_NOVA,
    BOBA,
    BOBA_AVAX,
    BOBA_BNB,
    BTTC,
    METIS,
    OPTIMISM,
    KAVA,
}

#[derive(Debug)]
pub struct Network<'a> {
    pub name: NetworkName,
    pub chain_id: u32,
    pub rpc: &'a str,
}

pub static NETWORKS: phf::Map<&'static str, Network> = phf_map! {
    "ethereum" => Network {
        name: NetworkName::ETHEREUM,
        chain_id: 1,
        rpc: "https://eth.public-rpc.com"
    },
    "arbitrum" => Network {
        name: NetworkName::ARBITRUM,
        chain_id: 42161,
        rpc: "https://arb1.arbitrum.io/rpc"
    },
    "polygon" => Network {
        name: NetworkName::POLYGON,
        chain_id: 137,
        rpc: "https://polygon-rpc.com"
    },
    "fantom" => Network {
        name: NetworkName::FANTOM,
        chain_id: 250,
        rpc: "https://rpc.ftm.tools"
    },
    "gnosis" => Network {
        name: NetworkName::GNOSIS,
        chain_id: 100,
        rpc: "https://gnosis.public-rpc.com"
    },
    "boba" => Network {
        name: NetworkName::BOBA,
        chain_id: 288,
        rpc: "https://mainnet.boba.network"
    },
    "avalanche" => Network {
        name: NetworkName::AVALANCHE,
        chain_id: 43114,
        rpc: "https://avalanche.public-rpc.com"
    },
    "celo" => Network {
        name: NetworkName::CELO,
        chain_id: 42220,
        rpc: "https://rpc.ankr.com/celo"
    },
    "bsc" => Network {
        name: NetworkName::BSC,
        chain_id: 56,
        rpc: "https://bscrpc.com"
    },
    "harmony" => Network {
        name: NetworkName::HARMONY,
        chain_id: 1666600000,
        rpc: "https://harmony.public-rpc.com"
    },
    "kava" => Network {
        name: NetworkName::KAVA,
        chain_id: 2222,
        rpc: "https://evm2.kava.io"
    },
    "metis" => Network {
        name: NetworkName::METIS,
        chain_id: 1088,
        rpc: "https://andromeda.metis.io/?owner=1088"
    },
    "optimism" => Network {
        name: NetworkName::OPTIMISM,
        chain_id: 10,
        rpc: "https://mainnet.optimism.io"
    },
    "bttc" => Network {
        name: NetworkName::BTTC,
        chain_id: 199,
        rpc: "https://rpc.bittorrentchain.io"
    },
    "arbitrum_nova" => Network {
        name: NetworkName::ARBITRUM_NOVA,
        chain_id: 42170,
        rpc: "https://nova.arbitrum.io/rpc"
    },
    "moonriver" => Network {
        name: NetworkName::MOONRIVER,
        chain_id: 1285,
        rpc: "https://rpc.api.moonriver.moonbeam.network"
    },
    "moonbeam" => Network {
        name: NetworkName::MOONBEAM,
        chain_id: 1284,
        rpc: "https://rpc.api.moonbeam.network"
    },
    "boba_avax" => Network {
        name: NetworkName::BOBA_AVAX,
        chain_id: 43288,
        rpc: "https://avax.boba.network"
    },
    "boba_bnb" => Network {
        name: NetworkName::BOBA_BNB,
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

pub static MINICHEF_SUBGRAPH: phf::Map<&'static str, &'static str> = phf_map! {
    "ethereum" => "https://api.thegraph.com/subgraphs/name/sushiswap/master-chefv2",
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
