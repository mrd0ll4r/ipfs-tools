#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;

use failure::{err_msg, Error, ResultExt};
use flate2::read::GzDecoder;
use ipfs_resolver_common::logging;
use ipfs_resolver_common::wantlist::{JSONMessage, JSONWantlistEntry};
use ipfs_resolver_common::Result;
use ipfs_resolver_db::canonicalize_cid_from_str_to_cidv1;
use ipfs_resolver_db::model::BitswapMessage;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use diesel::{Connection, PgConnection};

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    logging::set_up_logging(false)?;

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return Err(err_msg("missing arguments"));
    }
    let infile = args[1].clone();

    debug!("connecting to DB...");
    let conn = ipfs_resolver_db::establish_connection()?;
    info!("connected to DB");

    debug!("opening input file at {}", infile);
    let mut reader = BufReader::new(GzDecoder::new(
        std::fs::File::open(&infile).context("unable to open input files")?,
    ));
    info!("opened input file");

    let mut buf = String::new();
    let mut num_objects: u32 = 0;
    let mut num_bs_messages: u32 = 0;
    let mut num_wl_entries: u32 = 0;
    let mut num_inserted_messages: u32 = 0;
    let mut num_inserted_entries: u32 = 0;

    // Read messages
    let before = Instant::now();
    info!("reading messages...");
    loop {
        buf.clear();
        let n = reader
            .read_line(&mut buf)
            .context("unable to read from input file")?;
        if n == 0 {
            break;
        }

        let msg: JSONMessage = serde_json::from_str(&buf).context("unable to decode object")?;
        num_objects += 1;

        if msg.received_entries.is_some() {
            num_bs_messages += 1;
            num_wl_entries += msg.received_entries.as_ref().unwrap().len() as u32;

            let res =
                conn.transaction(|| ipfs_resolver_db::db::insert_bitswap_message(&conn, 1, &msg));
            match res {
                Ok(_) => {
                    num_inserted_messages += 1;
                    num_inserted_entries += msg.received_entries.unwrap().len() as u32;
                    debug!("inserted something.");
                }
                Err(e) => {
                    info!("unable to insert, skipping: {:?}", e)
                }
            }
        }
    }
    let elapsed = before.elapsed();

    info!("done processing.");
    info!(
        "saw {} objects, of which {} were Bitswap messages, containing {} entries in total",
        num_objects, num_bs_messages, num_wl_entries
    );
    info!(
        "inserted {} messages ({:.2}%), containing {} entries in total ({:.2}%)",
        num_inserted_messages,
        (f64::from(num_inserted_messages) / f64::from(num_bs_messages)) * 100_f64,
        num_inserted_entries,
        (f64::from(num_inserted_entries) / f64::from(num_wl_entries)) * 100_f64
    );
    info!(
        "took {:?} => {} ms/message, {} messages/s",
        elapsed,
        (elapsed.as_millis() as f64) / f64::from(num_inserted_messages),
        f64::from(num_inserted_messages) / elapsed.as_secs_f64()
    );

    Ok(())
}
