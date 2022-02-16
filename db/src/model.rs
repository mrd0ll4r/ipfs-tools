use crate::schema::*;

#[derive(Identifiable, Queryable, PartialEq, Debug, Clone)]
#[table_name = "codecs"]
#[primary_key(id)]
pub struct Codec {
    pub id: i32,
    pub name: String,
}

lazy_static! {
    pub static ref CODEC_DAG_PB: Codec = Codec {
        id: 112,
        name: "dag-pb".to_string()
    };
    pub static ref CODEC_RAW: Codec = Codec {
        id: 85,
        name: "raw".to_string()
    };

/// MerkleDAG cbor
pub static ref CODEC_DAG_CBOR : Codec = Codec {
    id: 0x71,
    name:"dag-cbor".to_string()
    };
/// Raw Git object
pub static ref CODEC_GIT_RAW : Codec = Codec {
    id: 0x78,
    name:"git-raw".to_string()
};
/// Ethereum Block (RLP)
pub static ref CODEC_ETH_BLOCK : Codec = Codec {
    id: 0x90,
    name:"eth-block".to_string()
};
/// Ethereum Block List (RLP)
pub static ref CODEC_ETH_BLOCK_LIST : Codec = Codec {
    id: 0x91,
    name:"eth-block-list".to_string()
};
/// Ethereum Transaction Trie (Eth-Trie)
pub static ref CODEC_ETH_TX_TRIE : Codec = Codec {
    id: 0x92,
    name:"eth-tx-trie".to_string()
};
/// Ethereum Transaction (RLP)
pub static ref CODEC_ETH_TX : Codec = Codec {
    id: 0x93,
    name:"eth-tx".to_string()
};
/// Ethereum Transaction Receipt Trie (Eth-Trie)
pub static ref CODEC_ETH_TX_RECEIPT_TRIE : Codec = Codec {
    id: 0x94,
    name:"eth-tx-receipt-trie".to_string()
};
/// Ethereum Transaction Receipt (RLP)
pub static ref CODEC_ETH_TX_RECEIPT : Codec = Codec {
    id: 0x95,
    name:"eth-tx-receipt".to_string()
};
/// Ethereum State Trie (Eth-Secure-Trie)
pub static ref CODEC_ETH_STATE_TRIE : Codec = Codec {
    id: 0x96,
    name:"eth-state-trie".to_string()
};
/// Ethereum Account Snapshot (RLP)
pub static ref CODEC_ETH_ACCOUNT_SNAPSHOT : Codec = Codec {
    id: 0x97,
    name:"eth-account-snapshot".to_string()
};
/// Ethereum Contract Storage Trie (Eth-Secure-Trie)
pub static ref CODEC_ETH_STORAGE_TRIE : Codec = Codec {
    id: 0x98,
    name:"eth-storage-trie".to_string()
};
/// Bitcoin Block
pub static ref CODEC_BITCOIN_BLOCK : Codec = Codec {
    id: 0xb0,
    name:"bitcoin-block".to_string()
};
/// Bitcoin Transaction
pub static ref CODEC_BITCOIN_TX : Codec = Codec {
    id: 0xb1,
    name:"bitcoin-tx".to_string()
};
/// Zcash Block
pub static ref CODEC_ZCASH_BLOCK : Codec = Codec {
    id: 0xc0,
    name:"zcash-block".to_string()
};
/// Zcash Transaction
pub static ref CODEC_ZCASH_TX : Codec = Codec {
    id: 0xc1,
    name:"zcash-tx".to_string()
};
/// MerkleDAG json
pub static ref CODEC_DAG_JSON : Codec = Codec {
    id: 0x0129,
    name:"dag-json".to_string()
};

}

#[derive(Insertable)]
#[table_name = "codecs"]
pub struct NewCodec<'a> {
    pub id: &'a i32,
    pub name: &'a str,
}

#[derive(Identifiable, Queryable, PartialEq, Debug, Clone)]
#[table_name = "unixfs_types"]
#[primary_key(id)]
pub struct UnixFSType {
    pub id: i32,
    pub name: String,
}

lazy_static! {
    pub static ref UNIXFS_TYPE_RAW: UnixFSType = UnixFSType {
        id: 0,
        name: "raw".to_string()
    };
    pub static ref UNIXFS_TYPE_DIRECTORY: UnixFSType = UnixFSType {
        id: 1,
        name: "directory".to_string()
    };
    pub static ref UNIXFS_TYPE_FILE: UnixFSType = UnixFSType {
        id: 2,
        name: "file".to_string()
    };
    pub static ref UNIXFS_TYPE_METADATA: UnixFSType = UnixFSType {
        id: 3,
        name: "metadata".to_string()
    };
    pub static ref UNIXFS_TYPE_SYMLINK: UnixFSType = UnixFSType {
        id: 4,
        name: "symlink".to_string()
    };
    pub static ref UNIXFS_TYPE_HAMT_SHARD: UnixFSType = UnixFSType {
        id: 5,
        name: "HAMTShard".to_string()
    };
}

#[derive(Identifiable, Queryable, PartialEq, Debug, Clone)]
#[table_name = "errors"]
#[primary_key(id)]
pub struct BlockError {
    pub id: i32,
    pub name: String,
}

lazy_static! {
    pub static ref BLOCK_ERROR_FAILED_TO_GET_BLOCK_DEADLINE_EXCEEDED: BlockError = BlockError {
        id: 1,
        name: "failed to get block: deadline exceeded".to_string()
    };
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Clone)]
#[table_name = "blocks"]
#[belongs_to(Codec)]
#[primary_key(id)]
pub struct Block {
    pub id: i64,
    pub codec_id: i32,
    pub cidv1: Vec<u8>,
}

#[derive(Insertable)]
#[table_name = "blocks"]
pub struct NewBlock<'a> {
    pub cidv1: &'a Vec<u8>,
    pub codec_id: &'a i32,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Clone)]
#[table_name = "block_stats"]
#[belongs_to(Block, foreign_key = "block_id")]
#[primary_key(block_id)]
pub struct BlockStat {
    pub block_id: i64,
    pub block_size: i32,
    pub first_bytes: Vec<u8>,
}

#[derive(Insertable)]
#[table_name = "block_stats"]
pub struct NewBlockStat<'a> {
    pub block_id: &'a i64,
    pub block_size: &'a i32,
    pub first_bytes: &'a Vec<u8>,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Clone)]
#[table_name = "failed_resolves"]
#[belongs_to(Block, foreign_key = "block_id")]
#[belongs_to(BlockError, foreign_key = "error_id")]
#[primary_key(id)]
pub struct FailedResolve {
    pub block_id: i64,
    pub error_id: i32,
    pub ts: chrono::DateTime<chrono::Utc>,
    pub id: i64,
}

#[derive(Insertable)]
#[table_name = "failed_resolves"]
pub struct NewFailedResolve<'a> {
    pub block_id: &'a i64,
    pub error_id: &'a i32,
    pub ts: &'a chrono::DateTime<chrono::Utc>,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Clone)]
#[table_name = "successful_resolves"]
#[belongs_to(Block, foreign_key = "block_id")]
#[primary_key(id)]
pub struct SuccessfulResolve {
    pub block_id: i64,
    pub ts: chrono::DateTime<chrono::Utc>,
    pub id: i64,
}

#[derive(Insertable)]
#[table_name = "successful_resolves"]
pub struct NewSuccessfulResolve<'a> {
    pub block_id: &'a i64,
    pub ts: &'a chrono::DateTime<chrono::Utc>,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Clone)]
#[table_name = "unixfs_blocks"]
#[belongs_to(BlockStat, foreign_key = "block_id")]
#[belongs_to(UnixFSType, foreign_key = "unixfs_type_id")]
#[primary_key(block_id)]
pub struct UnixFSBlock {
    pub block_id: i64,
    pub unixfs_type_id: i32,
    pub size: i64,
    pub cumulative_size: i64,
    pub blocks: i32,
    pub num_links: i32,
}

#[derive(Insertable)]
#[table_name = "unixfs_blocks"]
pub struct NewUnixFSBlock<'a> {
    pub block_id: &'a i64,
    pub unixfs_type_id: &'a i32,
    pub size: &'a i64,
    pub cumulative_size: &'a i64,
    pub blocks: &'a i32,
    pub num_links: &'a i32,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Clone)]
#[table_name = "unixfs_links"]
#[belongs_to(UnixFSBlock, foreign_key = "parent_block_id")]
#[primary_key(parent_block_id, referenced_cidv1, name)]
pub struct UnixFSLink {
    pub parent_block_id: i64,
    pub name: String,
    pub size: i64,
    pub referenced_cidv1: Vec<u8>,
}

#[derive(Insertable)]
#[table_name = "unixfs_links"]
pub struct NewUnixFSLink<'a> {
    pub parent_block_id: &'a i64,
    pub referenced_cidv1: &'a Vec<u8>,
    pub name: &'a str,
    pub size: &'a i64,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Clone)]
#[table_name = "unixfs_file_heuristics"]
#[belongs_to(UnixFSBlock, foreign_key = "block_id")]
#[primary_key(block_id)]
pub struct UnixFSFileHeuristics {
    pub block_id: i64,
    pub tree_mime_mime_type: Option<String>,
    pub chardet_encoding: Option<String>,
    pub chardet_language: Option<String>,
    pub chardet_confidence: Option<f32>,
    pub chardetng_encoding: Option<String>,
    pub whatlang_language: Option<String>,
    pub whatlang_script: Option<String>,
    pub whatlang_confidence: Option<f64>,
}

#[derive(Insertable)]
#[table_name = "unixfs_file_heuristics"]
pub struct NewUnixFSFileHeuristics<'a> {
    pub block_id: &'a i64,
    pub tree_mime_mime_type: Option<&'a str>,
    pub chardet_encoding: Option<&'a str>,
    pub chardet_language: Option<&'a str>,
    pub chardet_confidence: Option<&'a f32>,
    pub chardetng_encoding: Option<&'a str>,
    pub whatlang_language: Option<&'a str>,
    pub whatlang_script: Option<&'a str>,
    pub whatlang_confidence: Option<&'a f64>,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Clone)]
#[table_name = "peers"]
#[primary_key(id)]
pub struct Peer {
    pub id: i32,
    pub peer_id: String,
}

#[derive(Insertable)]
#[table_name = "peers"]
pub struct NewPeer<'a> {
    pub peer_id: &'a str,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Clone)]
#[table_name = "monitors"]
#[primary_key(id)]
pub struct Monitor {
    pub id: i32,
    pub name: String,
    pub ipfs_version: String,
    pub comments: Option<String>,
}

#[derive(Insertable)]
#[table_name = "monitors"]
pub struct NewMonitor<'a> {
    pub id: i32,
    pub name: &'a str,
    pub ipfs_version: &'a str,
    pub comments: Option<&'a str>,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Clone)]
#[table_name = "bitswap_wantlist_entry_types"]
#[primary_key(id)]
pub struct BitswapWantlistEntryType {
    pub id: i32,
    pub entry_type: String,
}

lazy_static! {
    pub static ref BITSWAP_WANTLIST_ENTRY_TYPE_WANT_BLOCK: BitswapWantlistEntryType =
        BitswapWantlistEntryType {
            id: 1,
            entry_type: "WANT_BLOCK".to_string()
        };
    pub static ref BITSWAP_WANTLIST_ENTRY_TYPE_WANT_BLOCK_SEND_DONT_HAVE: BitswapWantlistEntryType =
        BitswapWantlistEntryType {
            id: 2,
            entry_type: "WANT_BLOCK_SEND_DONT_HAVE".to_string()
        };
    pub static ref BITSWAP_WANTLIST_ENTRY_TYPE_WANT_HAVE: BitswapWantlistEntryType =
        BitswapWantlistEntryType {
            id: 3,
            entry_type: "WANT_HAVE".to_string()
        };
    pub static ref BITSWAP_WANTLIST_ENTRY_TYPE_WANT_HAVE_SEND_DONT_HAVE: BitswapWantlistEntryType =
        BitswapWantlistEntryType {
            id: 4,
            entry_type: "WANT_HAVE_SEND_DONT_HAVE".to_string()
        };
    pub static ref BITSWAP_WANTLIST_ENTRY_TYPE_CANCEL: BitswapWantlistEntryType =
        BitswapWantlistEntryType {
            id: 5,
            entry_type: "CANCEL".to_string()
        };
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Clone)]
#[table_name = "bitswap_messages"]
#[belongs_to(Peer, foreign_key = "peer_id")]
#[belongs_to(Monitor, foreign_key = "monitor_id")]
#[primary_key(id)]
pub struct BitswapMessage {
    pub id: i64,
    pub peer_id: i32,
    pub monitor_id: i32,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Insertable)]
#[table_name = "bitswap_messages"]
pub struct NewBitswapMessage<'a> {
    pub peer_id: &'a i32,
    pub monitor_id: &'a i32,
    pub timestamp: &'a chrono::DateTime<chrono::Utc>,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Clone)]
#[table_name = "bitswap_wantlist_entries"]
#[belongs_to(BitswapMessage, foreign_key = "message_id")]
#[belongs_to(Block, foreign_key = "cid_id")]
#[belongs_to(BitswapWantlistEntryType, foreign_key = "entry_type_id")]
#[primary_key(id)]
pub struct BitswapWantlistEntry {
    pub id: i64,
    pub message_id: i64,
    pub cid_id: i64,
    pub entry_type_id: i32,
    pub priority: i32,
}

#[derive(Insertable)]
#[table_name = "bitswap_wantlist_entries"]
pub struct NewBitswapWantlistEntry<'a> {
    pub message_id: &'a i64,
    pub cid_id: &'a i64,
    pub entry_type_id: &'a i32,
    pub priority: &'a i32,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Clone)]
#[table_name = "underlay_addresses"]
#[primary_key(id)]
pub struct UnderlayAddress {
    pub id: i64,
    pub multiaddress: String,
}

#[derive(Insertable)]
#[table_name = "underlay_addresses"]
pub struct NewUnderlayAddress<'a> {
    pub multiaddress: &'a str,
}

#[derive(Identifiable, Queryable, Associations, PartialEq, Debug, Clone)]
#[table_name = "bitswap_messages_underlay_addresses"]
#[belongs_to(BitswapMessage, foreign_key = "message_id")]
#[belongs_to(UnderlayAddress, foreign_key = "address_id")]
#[primary_key(message_id, address_id)]
pub struct BitswapMessageUnderlayAddress {
    pub message_id: i64,
    pub address_id: i64,
}

#[derive(Insertable)]
#[table_name = "bitswap_messages_underlay_addresses"]
pub struct NewBitswapMessageUnderlayAddress<'a> {
    pub message_id: &'a i64,
    pub address_id: &'a i64,
}
