-- This file should undo anything in `up.sql`
DROP TABLE bitswap_messages_underlay_addresses;

DROP TABLE underlay_addresses;

DROP TABLE bitswap_wantlist_entries;

DROP TABLE bitswap_messages;

DROP TABLE bitswap_wantlist_entry_types;

DROP TABLE monitors;

DROP TABLE peers;

ALTER TABLE blocks
    ALTER id TYPE INT;
ALTER TABLE failed_resolves
    ALTER id TYPE INT;
ALTER TABLE failed_resolves
    ALTER block_id TYPE INT;
ALTER TABLE successful_resolves
    ALTER id TYPE INT;
ALTER TABLE successful_resolves
    ALTER block_id TYPE INT;
ALTER TABLE block_stats
    ALTER block_id TYPE INT;
ALTER TABLE unixfs_blocks
    ALTER block_id TYPE INT;
ALTER TABLE unixfs_file_heuristics
    ALTER block_id TYPE INT;
ALTER TABLE unixfs_links
    ALTER parent_block_id TYPE INT;

ALTER SEQUENCE blocks_id_seq AS INT;
ALTER SEQUENCE failed_resolves_id_seq AS INT;
ALTER SEQUENCE successful_resolves_id_seq AS INT;

ALTER TABLE successful_resolves
    ALTER ts TYPE TIMESTAMP WITHOUT TIME ZONE;
ALTER TABLE failed_resolves
    ALTER ts TYPE TIMESTAMP WITHOUT TIME ZONE;

DELETE
FROM codecs
WHERE id IN (
-- MerkleDAG cbor
             x'71'::INT,
-- Raw Git object
             x'78'::INT,
-- Ethereum Block (RLP)
             x'90'::INT,
-- Ethereum Block List (RLP)
             x'91'::INT,
-- Ethereum Transaction Trie (Eth-Trie)
             x'92'::INT,
-- Ethereum Transaction (RLP)
             x'93'::INT,
-- Ethereum Transaction Receipt Trie (Eth-Trie)
             x'94'::INT,
-- Ethereum Transaction Receipt (RLP)
             x'95'::INT,
-- Ethereum State Trie (Eth-Secure-Trie)
             x'96'::INT,
-- Ethereum Account Snapshot (RLP)
             x'97'::INT,
-- Ethereum Contract Storage Trie (Eth-Secure-Trie)
             x'98'::INT,
-- Bitcoin Block
             x'b0'::INT,
-- Bitcoin Transaction
             x'b1'::INT,
-- Zcash Block
             x'c0'::INT,
-- Zcash Transaction
             x'c1'::INT,
-- MerkleDAG json
             x'0129'::INT);

