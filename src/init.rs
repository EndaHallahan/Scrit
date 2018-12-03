use scrivx_reader::get_scrivx;
use std::path::Path;
use std::error::Error;
use std::io::{Read, Write};
use std::fs;
use minidom::Element;

pub fn check_init() -> bool {
	let map_path = Path::new("Scrit/scrit_map.xml");
	if map_path.exists() {
		true
	} else {
		false
	}
}

pub fn init() {
	if check_init() {
		println!("Scrit has already been initialized in this directory.");
		return
	} else {
		println!("Initializing...");
		match fs::create_dir("Scrit") {
			Err(why) => panic!("{}", why.description()),
			Ok(_) => println!("Successfully created scrit folder.")
		}
		let mut map = match fs::File::create("Scrit/scrit_map.xml") {
			Err(why) => panic!("{}", why.description()),
			Ok(file) => file
		};
		match get_scrivx() {
			Ok(name) => {
				let project_name = name.trim_right_matches(".scrivx").clone();
				let map_template = format!(r#"
<?xml version="1.0" encoding="UTF-8"?>
<ScritMap  xmlns="argabarga">
	<Project>
		<Title>{}</Title>
	</Project>
	<Drive>
		<Owner></Owner>
		<Directory></Directory>
		<LastPush></LastPush>
		<Files></Files>
	</Drive>
</ScritMap>
"#, project_name);
				match map.write_all(map_template.as_bytes()) {
					Err(why) => panic!("{}", why.description()),
					Ok(_) => println!("Successfully wrote map file.")
				}
			},
			Err(_) => {return;}
		}
		
		println!("Done!");
	}
}

pub fn get_map<'l>() -> Element {
	let mut f = fs::File::open("Scrit/scrit_map.xml").expect("File not found!");
	let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file.");
    let mut root: Element = contents.parse().unwrap();
    root
}