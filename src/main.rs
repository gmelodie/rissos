use anyhow::{anyhow, Result};
use rss::Channel;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct Database {
    map: HashMap<String, Channel>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn get_channel(&self, url: &str) -> Option<&Channel> {
        self.map.get(url)
    }

    /// Adds a new channel from a given URL to the database
    pub fn add_channel(&mut self, url: &str) -> Result<()> {
        // error: channel already exists
        if self.map.contains_key(url) {
            return Err(anyhow!("URL already present"));
        }
        // fetch URL, build Channel struct, add to database
        let body: String = ureq::get(url).call()?.into_string()?;
        let channel = Channel::from_str(body.as_str())?;
        self.map.insert(url.to_string(), channel);

        Ok(())
    }

    /// Removes a channel from the database
    /// Takes the URL of the channel as key
    pub fn rm_channel(&mut self, url: &str) -> Result<Channel> {
        // error: channel not included
        if !self.map.contains_key(url) {
            return Err(anyhow!("URL not present"));
        }

        self.map
            .remove(url)
            .ok_or(anyhow!("URL present but not removed"))
    }

    // update channel (only if last build date is newer than what we have, add force option)
    // pub fn update_channel(&mut self, url: &str) -> Result<()> {}
    // util: save channel to file
    // pub fn save_channel(&mut self, url: &str) -> Result<()> {}
}

fn main() {
    let mut db = Database::new();
    db.add_channel("https://blog.apnic.net/feed/").unwrap();
    println!("{:#?}", db);
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
        db.add_channel(EXAMPLE_FEED).unwrap();
        assert!(db.add_channel(EXAMPLE_FEED).is_err());
    }

    #[test]
    fn test_rm() {
        let mut db = Database::new();
        db.add_channel(EXAMPLE_FEED).unwrap();
        assert!(db.rm_channel(EXAMPLE_FEED).is_ok());
        assert!(db.get_channel(EXAMPLE_FEED).is_none());
    }

    #[test]
    fn test_duplicate_rm() {
        let mut db = Database::new();
        db.add_channel(EXAMPLE_FEED).unwrap();
        db.rm_channel(EXAMPLE_FEED).unwrap();
        assert!(db.rm_channel(EXAMPLE_FEED).is_err());
    }
}
