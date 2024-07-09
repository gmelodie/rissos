use rissos::Database;
use std::path::Path;

fn main() {
    let mut db = Database::load(Path::new("./test-data/example.db")).unwrap();
    // db.add_channel("https://blog.apnic.net/feed/").unwrap();
    // db.add_channel_from_file(Path::new("./test-data/example.xml"))
    //     .unwrap();
    println!("{:#?}", db);
}
