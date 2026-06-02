use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

pub struct Store {
    data: HashMap<String, String>,
    log: File,
}

impl Store {
    /// Open (or create) the append-only log at `path` and replay it into memory.
    pub fn open(path: &str) -> io::Result<Self> {
        let mut data = HashMap::new();

        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            // TODO: uncomment once load_from_reader is implemented
            Self::load_from_reader(reader, &mut data);
        }

        let log = OpenOptions::new().create(true).append(true).open(path)?;

        Ok(Self { data, log })
    }

    /// Replay the log file line by line to rebuild in-memory state.
    /// Each line is either `SET key value` or `DEL key`.
    fn load_from_reader(_reader: impl BufRead, _data: &mut HashMap<String, String>) {
        // TODO: iterate over lines
        // split each line into parts
        // match "SET" → insert into data
        // match "DEL" → remove from data
        // ignore anything else (corrupt lines)
        for line in _reader.lines().flatten() {
            match line.splitn(3, ' ').collect().as_slice() {
                ["SET", key, value] => { _data.insert(key.to_string(), value.to_string()); },
                ["DEL", key]        => { _data.del(key.to_string); }
                _                   => {}
            } 
        } 
    }

    /// Insert key → value and append `SET key value` to the log.
    pub fn set(&mut self, key: String, value: String) -> io::Result<()> {
        writeln!(self.log, "SET {key} {value}")?;
        self.log.flush()?;
        self.data.insert(key, value);
        Ok(())
    }

    /// Look up a key. Returns None if not present.
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    /// Remove a key. Appends `DEL key` to the log. Returns true if key existed.
    pub fn del(&mut self, key: &str) -> io::Result<bool> {
        // TODO: remove from data, check if it existed
        // TODO: if it existed, append DEL line to log
        let existed = self.data.del(key).is_some();
        if existed {
            writeln!(self.log, "DEL {key}")?;
            self.log.flush()?;
        }
        Ok(existed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn temp_path(name: &str) -> String {
        format!("/tmp/redis-lite-test-{name}.log")
    }

    // load_from_reader tests use Cursor as a fake in-memory file — no disk needed

    #[test]
    fn test_load_set_lines() {
        let input = b"SET foo bar\nSET hello world\n";
        let mut data = HashMap::new();
        Store::load_from_reader(Cursor::new(input), &mut data);
        assert_eq!(data.get("foo"), Some(&"bar".to_string()));
        assert_eq!(data.get("hello"), Some(&"world".to_string()));
    }

    #[test]
    fn test_load_del_removes_key() {
        let input = b"SET foo bar\nDEL foo\n";
        let mut data = HashMap::new();
        Store::load_from_reader(Cursor::new(input), &mut data);
        assert_eq!(data.get("foo"), None);
    }

    #[test]
    fn test_load_ignores_corrupt_lines() {
        let input = b"SET foo bar\nGARBAGE\nDEL\n";
        let mut data = HashMap::new();
        Store::load_from_reader(Cursor::new(input), &mut data);
        assert_eq!(data.get("foo"), Some(&"bar".to_string()));
    }

    #[test]
    fn test_set_and_get() {
        let path = temp_path("set_get");
        let _ = std::fs::remove_file(&path);
        let mut store = Store::open(&path).unwrap();
        store.set("foo".into(), "bar".into()).unwrap();
        assert_eq!(store.get("foo"), Some(&"bar".to_string()));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_get_missing_key() {
        let path = temp_path("get_missing");
        let _ = std::fs::remove_file(&path);
        let store = Store::open(&path).unwrap();
        assert_eq!(store.get("nonexistent"), None);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_del_existing_key() {
        let path = temp_path("del_existing");
        let _ = std::fs::remove_file(&path);
        let mut store = Store::open(&path).unwrap();
        store.set("foo".into(), "bar".into()).unwrap();
        assert_eq!(store.del("foo").unwrap(), true);
        assert_eq!(store.get("foo"), None);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_del_missing_key() {
        let path = temp_path("del_missing");
        let _ = std::fs::remove_file(&path);
        let mut store = Store::open(&path).unwrap();
        assert_eq!(store.del("nonexistent").unwrap(), false);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_set_overwrites_existing_key() {
        let path = temp_path("overwrite");
        let _ = std::fs::remove_file(&path);
        let mut store = Store::open(&path).unwrap();
        store.set("foo".into(), "bar".into()).unwrap();
        store.set("foo".into(), "baz".into()).unwrap();
        assert_eq!(store.get("foo"), Some(&"baz".to_string()));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_data_survives_reopen() {
        let path = temp_path("persistence");
        let _ = std::fs::remove_file(&path);

        {
            let mut store = Store::open(&path).unwrap();
            store.set("foo".into(), "bar".into()).unwrap();
            store.set("hello".into(), "world".into()).unwrap();
            store.del("foo").unwrap();
        }

        let store = Store::open(&path).unwrap();
        assert_eq!(store.get("foo"), None);
        assert_eq!(store.get("hello"), Some(&"world".to_string()));
        let _ = std::fs::remove_file(&path);
    }
}
