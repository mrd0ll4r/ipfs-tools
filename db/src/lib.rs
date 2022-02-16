#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;

use cid::{Cid, Codec};
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use diesel::{Connection, PgConnection};
use failure::ResultExt;
use ipfs_resolver_common::Result;
use std::convert::TryFrom;
use std::env;

pub mod db;
pub mod model;
pub mod schema;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

fn init_pool(database_url: &str) -> std::result::Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .min_idle(Some(10))
        .max_size(64)
        .build(manager)
}

pub fn create_pool() -> Result<PgPool> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").context("DATABASE_URL must be set")?;

    let pool = init_pool(&database_url)?;

    Ok(pool)
}

pub fn establish_connection() -> Result<PgConnection> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
    let conn = PgConnection::establish(&database_url)
        .context(format!("error connecting to {}", database_url))?;

    Ok(conn)
}

pub fn canonicalize_cid_from_str_to_cidv1(cid: &str) -> Result<Cid> {
    let provided_cid = Cid::try_from(cid).context("invalid CID")?;
    Ok(canonicalize_cid_to_cidv1(&provided_cid))
}

pub fn canonicalize_cid_to_cidv1(c: &Cid) -> Cid {
    Cid::new_v1(c.codec(), c.hash().to_owned())
}

pub fn canonicalize_cid_from_str_to_base32_cidv1(cid: &str) -> Result<String> {
    let provided_cid = Cid::try_from(cid).context("invalid CID")?;
    Ok(canonicalize_cid_to_base32_cidv1(&provided_cid))
}

pub fn canonicalize_cid_to_base32_cidv1(c: &Cid) -> String {
    let v1_cid = canonicalize_cid_to_cidv1(c);
    multibase::encode(multibase::Base::Base32Lower, v1_cid.to_bytes())
}

pub fn canonicalize_cid_from_str_to_cidv1_and_codec(cid: &str) -> Result<(Cid,Codec)> {
    let provided_cid = Cid::try_from(cid).context("invalid CID")?;
    Ok((canonicalize_cid_to_cidv1(&provided_cid),provided_cid.codec()))
}