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
            match line.splitln(3, ' ').collect().as_slice() {
                ["SET", key, value] => { self.data.insert(key.to_string(), value.to_string()); },
                ["DEL", key]        => { self.data.del(key.to_string); }
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
