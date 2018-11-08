use std::path::Path;
use std::fs;

pub fn init() {
	let map_path = Path::new("scrit_map.txt");
	if map_path.exists() {
		println!("Scrit has already been initialized in this directory.");
		return
	} else {
		println!("Initializing...");
		let mut map = fs::File::create("scrit_map.txt").unwrap();
		println!("Done!");
	}
}