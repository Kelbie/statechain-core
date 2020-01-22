use std::path::Path;
use tempdir::TempDir;
use leveldb::database::Database;
use leveldb::kv::KV;
use rocksdb::{DB, Options};

pub fn main() {
    let path = "/Users/kevinkelbie/Documents/GitHub/statechain-core/src/db";
    
    let db = DB::open_default(path).unwrap();
    db.put(b"my key", b"my value").unwrap();
    match db.get(b"my key") {
        Ok(Some(value)) => println!("retrieved value {}", String::from_utf8(value).unwrap()),
        Ok(None) => println!("value not found"),
        Err(e) => println!("operational problem encountered: {}", e),
    }
    db.delete(b"my key").unwrap();
}
