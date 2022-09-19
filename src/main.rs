use std::collections::HashMap;
fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().expect("Key was not there");
    let value = args.next().unwrap();

    let mut database = Database::new().expect("Database::new() crashed");
    database.insert(key.to_uppercase(), value.clone());
    database.insert(key, value);
    match database.flush() {
        Ok(()) => println!("DB Flushed"),
        Err(err) => println!("Flush failed! Error: {}", err),
    }
}

struct Database {
    map: HashMap<String, String>,
    flush: bool,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        if !std::path::Path::new("kv.db").exists() {
            std::fs::File::create("kv.db")?;
        }
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupt DB");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database { map, flush: false })
    }
    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(mut self) -> std::io::Result<()> {
        self.flush = true;
        do_flush(&self)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        if !self.flush {
            let _  = do_flush(self);
        }  
    }
}

fn do_flush(database: &Database) -> std::io::Result<()> {
    let mut contents = String::new();
    for (key, value) in &database.map {
        contents.push_str(key);
        contents.push('\t');
        contents.push_str(value);
        contents.push('\n');
    }
    std::fs::write("kv.db", contents)
}