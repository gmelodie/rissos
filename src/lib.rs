use anyhow::{anyhow, Error, Result};
use rss::Channel;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufReader,
    path::Path,
    str::FromStr,
};

#[derive(Debug)]
pub struct Database {
    map: HashMap<String, Channel>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Loads Database from file
    pub fn load(path: &Path) -> Result<Database> {
        let contents = fs::read_to_string(path)?;
        Ok(Database::from_str(&contents)?)
    }

    /// Saves Database to file
    pub fn save(&self, path: &Path) -> Result<()> {
        Ok(fs::write(path, self.to_string())?)
    }

    pub fn get_channel(&self, url: &str) -> Option<&Channel> {
        self.map.get(url)
    }

    /// Adds a new channel from a given File to the database
    pub fn add_channel_from_file(&mut self, path: &Path) -> Result<()> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let channel = Channel::read_from(reader)?;

        // get feed URL from atom:link
        let url: &str = channel
            .extensions()
            .get("atom")
            .and_then(|m| m.get("link"))
            .and_then(|l| l.iter().find(|&r| r.name == "atom:link"))
            .and_then(|e| e.attrs.get("href"))
            .ok_or(anyhow!("Could not find feed link in file"))?;
        println!("channel link is: {url}");

        // error: channel already exists
        if self.map.contains_key(url) {
            return Err(anyhow!("URL already present"));
        }
        self.map.insert(url.to_string(), channel);

        Ok(())
    }

    /// Adds a new channel from a given URL to the database
    /// Returns an Err if it already exists
    pub fn add_channel(&mut self, url: &str) -> Result<()> {
        if self.map.contains_key(url) {
            return Err(anyhow!("URL already present"));
        }
        self.update_channel(url)
    }

    /// Removes a channel from the database
    /// Takes the URL of the channel as key
    pub fn rm_channel(&mut self, url: &str) -> Result<Channel> {
        self.map.remove(url).ok_or(anyhow!("URL not present"))
    }

    /// Updates a single channel if present, adds it otherwise
    pub fn update_channel(&mut self, url: &str) -> Result<()> {
        // fetch URL, build Channel struct, add to database
        let body: String = ureq::get(url).call()?.into_string()?;
        let channel = Channel::from_str(body.as_str())?;

        self.map
            .entry(url.to_string())
            .and_modify(|e| *e = channel.clone())
            .or_insert(channel);

        Ok(())
    }

    /// Updates all channels in Database
    pub fn update(&mut self) -> Result<()> {
        let urls: Vec<String> = self.map.keys().map(|url| url.to_string()).collect();
        for url in urls {
            self.update_channel(&url)?;
        }
        Ok(())
    }
}

impl ToString for Database {
    fn to_string(&self) -> String {
        // convert Database<String, Channel> to HashMap<String, String>
        let h: HashMap<String, String> = self
            .map
            .iter()
            .map(|(k, v)| (k.clone(), v.to_string()))
            .collect();
        // serialize HashMap<String, String>
        serde_json::to_string(&h).unwrap()
    }
}
impl FromStr for Database {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // deserialize string into HashMap<String, String>
        let h: HashMap<String, String> = serde_json::from_str(s)?;

        // convert h: HashMap<String, String> into hc: HashMap<String, Channel>
        let hc: HashMap<String, Channel> = h
            .iter()
            .map(|(k, v)| (k.clone(), Channel::from_str(v).unwrap()))
            .collect();

        // return Database{map: hc}
        Ok(Database { map: hc })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_FEED: &str = "https://blog.apnic.net/feed/";

    #[test]
    fn test_add() {
        let mut db = Database::new();
        assert!(db.add_channel(EXAMPLE_FEED).is_ok());
        assert!(db.get_channel(EXAMPLE_FEED).is_some());
    }

    #[test]
    fn test_duplicate_add() {
        let mut db = Database::new();
        assert!(db.add_channel(EXAMPLE_FEED).is_ok()); // first is ok
        assert!(db.add_channel(EXAMPLE_FEED).is_err()); // duplicate is err
    }

    #[test]
    fn add_from_file() {
        let mut db = Database::new();
        assert!(db
            .add_channel_from_file(Path::new("./test-data/example.xml"))
            .is_ok());
    }

    #[test]
    fn test_rm() {
        let mut db = Database::new();
        assert!(db.add_channel(EXAMPLE_FEED).is_ok());
        assert!(db.rm_channel(EXAMPLE_FEED).is_ok());
        assert!(db.get_channel(EXAMPLE_FEED).is_none());
    }

    #[test]
    fn test_duplicate_rm() {
        let mut db = Database::new();
        assert!(db.add_channel(EXAMPLE_FEED).is_ok());
        assert!(db.rm_channel(EXAMPLE_FEED).is_ok());
        assert!(db.rm_channel(EXAMPLE_FEED).is_err());
    }

    #[test]
    fn save_db_to_file() {
        let mut db = Database::new();
        assert!(db.add_channel(EXAMPLE_FEED).is_ok());
        assert!(db.save(Path::new("/tmp/test-save-db.db")).is_ok());
    }

    #[test]
    fn load_db_from_file() {
        // load db from file
        assert!(Database::load(Path::new("./test-data/example.db")).is_ok());
    }
}
