use bincode::{serialize, deserialize, Infinite};

use std::path::PathBuf;
use std::io::prelude::*;
use std::fs::OpenOptions;

use accounting::{Entry};

#[derive(Debug)]
pub struct Registry {
    path: PathBuf
}

#[derive(Debug)]
struct EntryDeserializer {
    bin: Vec<u8>,
    total_read: usize
}

impl Registry {
    pub fn new(path: PathBuf) -> Result<Registry, String> {
        if !path.is_dir() {
            return Err("Должен быть указан путь до директории".to_owned());
        }

        Ok(Registry{
            path
        })
    }

    pub fn add_entry(&self, entry: Entry) -> Result<(), String> {
        let mut entries_path = self.path.clone();
        entries_path.push("entries.bin");
        

        let mut file = if !entries_path.exists() {
            OpenOptions::new()
                .write(true)
                .create(true)
                .open(entries_path)
                .map_err(|e| format!("{}", e))?
        } else {
            OpenOptions::new()
                .append(true)
                .open(entries_path)
                .map_err(|e| format!("{}", e))?
        };
        let encoded: Vec<u8> = serialize(&entry, Infinite).map_err(|e| format!("{}", e))?;

        let encoded_len: u64 = encoded.len() as u64;

        let encoded_encoded_len = serialize(&encoded_len, Infinite).map_err(|e| format!("{}", e))?;

        file.write_all(&encoded_encoded_len).map_err(|e| format!("{}", e))?;
        file.write_all(&encoded).map_err(|e| format!("{}", e))?;

        Ok(())
    }

    pub fn list(&self) -> Result<Vec<Entry>, String> {
        let mut entries_path = self.path.clone();
        entries_path.push("entries.bin");
        let mut file = OpenOptions::new()
            .read(true)
            .open(entries_path)
            .map_err(|e| format!("{}", e))?;

        let mut encoded = Vec::with_capacity(1024);

        file.read_to_end(&mut encoded).map_err(|e| format!("could not read file: {}", e))?;

        let mut deserializer = EntryDeserializer::new(encoded);

        let mut decoded = Vec::new();
        while let Some(entry) = deserializer.next()? {
            decoded.push(entry);
        }

        Ok(decoded)
    }
}

impl EntryDeserializer {
    fn new(bin: Vec<u8>) -> EntryDeserializer {
        EntryDeserializer { 
            bin,
            total_read: 0
        }
    }

    fn next(&mut self) -> Result<Option<Entry>, String> {
        if self.bin.len() - self.total_read > 8 {
            let entry_size: u64 = deserialize(&self.bin[self.total_read..self.total_read + 8]).map_err(|e| format!("could not deserialize entry size: {}", e))?;
            let entry: Entry = deserialize(&self.bin[self.total_read + 8..self.total_read + 8 + entry_size as usize]).map_err(|e| format!("could not deserialize entry: {}", e))?;
            self.total_read += 8 + entry_size as usize;
            return Ok(Some(entry));
        }

        Ok(None)
    }
}

