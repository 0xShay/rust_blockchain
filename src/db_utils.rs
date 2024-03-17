use sqlite;
use lazy_static;
use std::sync::{Mutex, Arc};
use tokio::sync::OnceCell;
use crate::block_utils::Block;

lazy_static::lazy_static! {
    static ref CONNECTION: Arc<Mutex<sqlite::Connection>> = {
        let conn = sqlite::Connection::open("db.sqlite").unwrap();
        Arc::new(Mutex::new(conn))
    };
}

pub fn create_tables() -> Result<(), sqlite::Error> {
    println!("[💾] Creating database tables (if they don't already exist)");
    let conn = CONNECTION.lock().unwrap(); 
    conn.execute("CREATE TABLE IF NOT EXISTS blocks (
        ix INTEGER NOT NULL,
        timestamp INTEGER NOT NULL,
        data TEXT NOT NULL,
        previous TEXT NOT NULL,
        nonce INTEGER NOT NULL,
        hash TEXT PRIMARY KEY NOT NULL,
        diff_bits INTEGER NOT NULL,
        acc_diff INTEGER NOT NULL
    );").unwrap();
    Ok(())
}

pub fn add_block(block: Block) -> Result<(), sqlite::Error> {
    println!("[💾] Adding block {:?} to database", block);
    let conn = CONNECTION.lock().unwrap(); 
    let query = "INSERT INTO blocks (
        ix, timestamp, data, previous, nonce, hash, diff_bits, acc_diff
    ) VALUES (
        ?, ?, ?, ?, ?, ?, ?, ?
    );";
    let mut statement = conn.prepare(query).unwrap();
    statement.bind::<&[(_, sqlite::Value)]>(&[
        (1, (block.index as i64).into()),
        (2, (block.timestamp as i64).into()),
        (3, block.data.into()),
        (4, block.previous.into()),
        (5, (block.nonce as i64).into()),
        (6, block.hash.into()),
        (7, (block.diff_bits as i64).into()),
        (8, (block.acc_diff as i64).into())
    ]).unwrap();
    statement.next();

    Ok(())
}

pub fn get_block(hash: &str) -> Option<Block> {
    println!("[💾] Getting block {:?} from database", hash);
    let conn = CONNECTION.lock().unwrap(); 
    let query = "SELECT * FROM blocks WHERE hash = ?";
    let mut statement = conn.prepare(query).unwrap();
    statement.bind((1, hash)).unwrap();

    if let Ok(sqlite::State::Row) = statement.next() {
        Some(Block {
            index: statement.read::<i64, _>("ix").unwrap().try_into().unwrap(),
            timestamp: statement.read::<i64, _>("timestamp").unwrap().try_into().unwrap(),
            data: statement.read::<String, _>("data").unwrap().try_into().unwrap(),
            previous: statement.read::<String, _>("previous").unwrap(),
            nonce: statement.read::<i64, _>("nonce").unwrap().try_into().unwrap(),
            hash: statement.read::<String, _>("hash").unwrap(),
            diff_bits: statement.read::<i64, _>("diff_bits").unwrap().try_into().unwrap(),
            acc_diff: statement.read::<i64, _>("acc_diff").unwrap().try_into().unwrap()
        })
    } else {
        None
    }
}

pub fn get_frontier_block() -> Option<Block> {
    println!("[💾] Getting frontier block from database");
    let conn = CONNECTION.lock().unwrap(); 
    let query = "SELECT * FROM blocks WHERE acc_diff = (SELECT MAX(acc_diff) FROM blocks) ORDER BY timestamp ASC;";
    let mut statement = conn.prepare(query).unwrap();

    if let Ok(sqlite::State::Row) = statement.next() {
        Some(Block {
            index: statement.read::<i64, _>("ix").unwrap().try_into().unwrap(),
            timestamp: statement.read::<i64, _>("timestamp").unwrap().try_into().unwrap(),
            data: statement.read::<String, _>("data").unwrap().try_into().unwrap(),
            previous: statement.read::<String, _>("previous").unwrap(),
            nonce: statement.read::<i64, _>("nonce").unwrap().try_into().unwrap(),
            hash: statement.read::<String, _>("hash").unwrap(),
            diff_bits: statement.read::<i64, _>("diff_bits").unwrap().try_into().unwrap(),
            acc_diff: statement.read::<i64, _>("acc_diff").unwrap().try_into().unwrap()
        })
    } else {
        None
    }
}