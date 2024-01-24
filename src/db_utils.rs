use sqlite;
use crate::block_utils::Block;

fn get_connection() -> sqlite::Connection {
    sqlite::open("db.sqlite").unwrap()
}

pub fn create_tables() -> Result<(), sqlite::Error> {
    println!("[💾] Creating database tables (if they don't already exist)");
    let conn = get_connection();
    conn.execute("CREATE TABLE IF NOT EXISTS blocks (
        ix INTEGER NOT NULL,
        timestamp INTEGER NOT NULL,
        data TEXT NOT NULL,
        previous TEXT NOT NULL,
        nonce INTEGER NOT NULL,
        hash TEXT PRIMARY KEY NOT NULL
    );").unwrap();
    Ok(())
}

pub fn add_block(block: Block) -> Result<(), sqlite::Error> {
    println!("[💾] Adding block {:?} to database", block);
    let conn = get_connection();
    let query = "INSERT INTO blocks (
        ix, timestamp, data, previous, nonce, hash
    ) VALUES (
        ?, ?, ?, ?, ?, ?
    );";
    let mut statement = conn.prepare(query).unwrap();
    statement.bind::<&[(_, sqlite::Value)]>(&[
        (1, (block.index as i64).into()),
        (2, (block.timestamp as i64).into()),
        (3, block.data.into()),
        (4, block.previous.into()),
        (5, (block.nonce as i64).into()),
        (6, block.hash.into())
    ]).unwrap();
    statement.next();

    Ok(())
}

pub fn get_block(hash: &str) -> Option<Block> {
    println!("[💾] Getting block {:?} from database", hash);
    let conn = get_connection();
    let query = "SELECT * FROM blocks WHERE hash = ?";
    let mut statement = conn.prepare(query).unwrap();
    statement.bind((1, hash)).unwrap();

    if let Ok(sqlite::State::Row) = statement.next() {
        Some(Block::create_block(
            statement.read::<i64, _>("ix").unwrap().try_into().unwrap(),
            statement.read::<i64, _>("timestamp").unwrap().try_into().unwrap(),
            statement.read::<String, _>("data").unwrap().try_into().unwrap(),
            statement.read::<String, _>("previous").unwrap(),
            statement.read::<i64, _>("nonce").unwrap().try_into().unwrap(),
            statement.read::<String, _>("hash").unwrap()
        ))
    } else {
        None
    }
}