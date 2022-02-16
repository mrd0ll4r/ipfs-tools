-- Your SQL goes here

CREATE TABLE peers (
    id SERIAL NOT NULL PRIMARY KEY,
    peer_id TEXT NOT NULL UNIQUE
);

-- Two things:
-- 1. It would be nicer to have the peer ID be saved as CIDv1 bytes.
--   That would save space, and maybe(?) deduplicate.
-- 2. It would be nicer to use a HASH index, because that, too, would save
--   space.
--   But it doesn't work with postgres 14, see
--   https://www.postgresql.org/message-id/6318fb86-0a64-61e7-e4e2-714db2b3407a%40anastigmatix.net
--   CREATE UNIQUE INDEX peers_peer_id_cidv1_unique ON peers USING hash (peer_id_cidv1);

CREATE TABLE monitors (
    id SERIAL NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    ipfs_version TEXT NOT NULL,
    comments TEXT
);

CREATE TABLE bitswap_wantlist_entry_types (
    id INT NOT NULL PRIMARY KEY,
    entry_type TEXT NOT NULL UNIQUE
);

INSERT INTO bitswap_wantlist_entry_types(id, entry_type) VALUES
 (1,'WANT_BLOCK'),
 (2,'WANT_BLOCK_SEND_DONT_HAVE'),
 (3,'WANT_HAVE'),
 (4,'WANT_HAVE_SEND_DONT_HAVE'),
 (5,'CANCEL');

CREATE TABLE bitswap_messages (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    peer_id INT NOT NULL REFERENCES peers(id),
    monitor_id INT NOT NULL REFERENCES monitors(id),
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE bitswap_wantlist_entries (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    message_id BIGINT NOT NULL REFERENCES bitswap_messages(id),
    cid_id BIGINT NOT NULL REFERENCES blocks(id),
    entry_type_id INT NOT NULL REFERENCES bitswap_wantlist_entry_types(id),
    priority INT NOT NULL
);

-- Underlay addresses
CREATE TABLE underlay_addresses (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    multiaddress TEXT NOT NULL UNIQUE
);

-- n:n mapping of bitswap messages to underlay addresses
CREATE TABLE bitswap_messages_underlay_addresses (
    message_id BIGINT NOT NULL REFERENCES bitswap_messages(id),
    address_id BIGINT NOT NULL REFERENCES underlay_addresses(id),
    PRIMARY KEY (message_id,address_id)
);
-- We create non-unique indices on both columns, in case we search by them.
CREATE INDEX bitswap_messages_underlay_addresses_message_id ON bitswap_messages_underlay_addresses(message_id);
CREATE INDEX bitswap_messages_underlay_addresses_address_id ON bitswap_messages_underlay_addresses(address_id);

-- Some cruft: make blocks.id into BIGINT, along with other things that might need it.
ALTER TABLE blocks ALTER id TYPE BIGINT;
ALTER TABLE failed_resolves ALTER id TYPE BIGINT;
ALTER TABLE failed_resolves ALTER block_id TYPE BIGINT;
ALTER TABLE successful_resolves ALTER id TYPE BIGINT;
ALTER TABLE successful_resolves ALTER block_id TYPE BIGINT;
ALTER TABLE block_stats ALTER block_id TYPE BIGINT;
ALTER TABLE unixfs_blocks ALTER block_id TYPE BIGINT;
ALTER TABLE unixfs_file_heuristics ALTER block_id TYPE BIGINT;
ALTER TABLE unixfs_links ALTER parent_block_id TYPE BIGINT;

ALTER SEQUENCE blocks_id_seq AS BIGINT;
ALTER SEQUENCE failed_resolves_id_seq AS BIGINT;
ALTER SEQUENCE successful_resolves_id_seq AS BIGINT;

-- More cruft: Add timezones to all timestamps
ALTER TABLE successful_resolves ALTER ts TYPE TIMESTAMP WITH TIME ZONE;
ALTER TABLE failed_resolves ALTER ts TYPE TIMESTAMP WITH TIME ZONE;

-- Add more codecs
INSERT INTO codecs(id, name) VALUES
-- MerkleDAG cbor
(x'71'::INT,'dag-cbor'),
-- Raw Git object
(x'78'::INT,'git-raw'),
-- Ethereum Block (RLP)
(x'90'::INT,'eth-block'),
-- Ethereum Block List (RLP)
(x'91'::INT,'eth-block-list'),
-- Ethereum Transaction Trie (Eth-Trie)
(x'92'::INT,'eth-tx-trie'),
-- Ethereum Transaction (RLP)
(x'93'::INT,'eth-tx'),
-- Ethereum Transaction Receipt Trie (Eth-Trie)
(x'94'::INT,'eth-tx-receipt-trie'),
-- Ethereum Transaction Receipt (RLP)
(x'95'::INT,'eth-tx-receipt'),
-- Ethereum State Trie (Eth-Secure-Trie)
(x'96'::INT,'eth-state-trie'),
-- Ethereum Account Snapshot (RLP)
(x'97'::INT,'eth-account-snapshot'),
-- Ethereum Contract Storage Trie (Eth-Secure-Trie)
(x'98'::INT,'eth-storage-trie'),
-- Bitcoin Block
(x'b0'::INT,'bitcoin-block'),
-- Bitcoin Transaction
(x'b1'::INT,'bitcoin-tx'),
-- Zcash Block
(x'c0'::INT,'zcash-block'),
-- Zcash Transaction
(x'c1'::INT,'zcash-tx'),
-- MerkleDAG json
(x'0129'::INT,'dag-json');















































