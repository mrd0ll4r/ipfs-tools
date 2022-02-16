table! {
    bitswap_messages (id) {
        id -> Int8,
        peer_id -> Int4,
        monitor_id -> Int4,
        timestamp -> Timestamptz,
    }
}

table! {
    bitswap_messages_underlay_addresses (message_id, address_id) {
        message_id -> Int8,
        address_id -> Int8,
    }
}

table! {
    bitswap_wantlist_entries (id) {
        id -> Int8,
        message_id -> Int8,
        cid_id -> Int8,
        entry_type_id -> Int4,
        priority -> Int4,
    }
}

table! {
    bitswap_wantlist_entry_types (id) {
        id -> Int4,
        entry_type -> Text,
    }
}

table! {
    block_stats (block_id) {
        block_id -> Int8,
        block_size -> Int4,
        first_bytes -> Bytea,
    }
}

table! {
    blocks (id) {
        id -> Int8,
        codec_id -> Int4,
        cidv1 -> Bytea,
    }
}

table! {
    codecs (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    errors (id) {
        id -> Int4,
        name -> Text,
    }
}

table! {
    failed_resolves (id) {
        block_id -> Int8,
        error_id -> Int4,
        ts -> Timestamptz,
        id -> Int8,
    }
}

table! {
    monitors (id) {
        id -> Int4,
        name -> Text,
        ipfs_version -> Text,
        comments -> Nullable<Text>,
    }
}

table! {
    peers (id) {
        id -> Int4,
        peer_id -> Text,
    }
}

table! {
    successful_resolves (id) {
        block_id -> Int8,
        ts -> Timestamptz,
        id -> Int8,
    }
}

table! {
    underlay_addresses (id) {
        id -> Int8,
        multiaddress -> Text,
    }
}

table! {
    unixfs_blocks (block_id) {
        block_id -> Int8,
        unixfs_type_id -> Int4,
        size -> Int8,
        cumulative_size -> Int8,
        blocks -> Int4,
        num_links -> Int4,
    }
}

table! {
    unixfs_file_heuristics (block_id) {
        block_id -> Int8,
        tree_mime_mime_type -> Nullable<Text>,
        chardet_encoding -> Nullable<Text>,
        chardet_language -> Nullable<Text>,
        chardet_confidence -> Nullable<Float4>,
        chardetng_encoding -> Nullable<Text>,
        whatlang_language -> Nullable<Text>,
        whatlang_script -> Nullable<Text>,
        whatlang_confidence -> Nullable<Float8>,
    }
}

table! {
    unixfs_links (parent_block_id, name, referenced_cidv1) {
        parent_block_id -> Int8,
        name -> Text,
        size -> Int8,
        referenced_cidv1 -> Bytea,
    }
}

table! {
    unixfs_types (id) {
        id -> Int4,
        name -> Text,
    }
}

joinable!(bitswap_messages -> monitors (monitor_id));
joinable!(bitswap_messages -> peers (peer_id));
joinable!(bitswap_messages_underlay_addresses -> bitswap_messages (message_id));
joinable!(bitswap_messages_underlay_addresses -> underlay_addresses (address_id));
joinable!(bitswap_wantlist_entries -> bitswap_messages (message_id));
joinable!(bitswap_wantlist_entries -> bitswap_wantlist_entry_types (entry_type_id));
joinable!(bitswap_wantlist_entries -> blocks (cid_id));
joinable!(block_stats -> blocks (block_id));
joinable!(blocks -> codecs (codec_id));
joinable!(failed_resolves -> blocks (block_id));
joinable!(failed_resolves -> errors (error_id));
joinable!(successful_resolves -> blocks (block_id));
joinable!(unixfs_blocks -> block_stats (block_id));
joinable!(unixfs_blocks -> unixfs_types (unixfs_type_id));
joinable!(unixfs_file_heuristics -> unixfs_blocks (block_id));
joinable!(unixfs_links -> unixfs_blocks (parent_block_id));

allow_tables_to_appear_in_same_query!(
    bitswap_messages,
    bitswap_messages_underlay_addresses,
    bitswap_wantlist_entries,
    bitswap_wantlist_entry_types,
    block_stats,
    blocks,
    codecs,
    errors,
    failed_resolves,
    monitors,
    peers,
    successful_resolves,
    underlay_addresses,
    unixfs_blocks,
    unixfs_file_heuristics,
    unixfs_links,
    unixfs_types,
);
