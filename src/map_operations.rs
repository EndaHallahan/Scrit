use scrivx_reader::get_scrivx;
use std::path::Path;
use std::error::Error;
use std::io::{Read, Write};
use std::fs;
use minidom::Element;

trait DeleteElement {
	fn delete_children(&mut self);
}
impl DeleteElement for Element {
	fn delete_children(&mut self) {
		let ele = Element::builder(self.name())
						.attr("id", self.attr("id").unwrap())
						.attr("title", self.attr("title").unwrap())
						.build();
		*self = ele;
	}
}

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
		<Owner/>
		<Directory/>
		<LastPush/>
		<Files/>
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

pub fn get_map() -> Element {
	let mut f = fs::File::open("Scrit/scrit_map.xml").expect("File not found!");
	let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file.");
    let mut root: Element = contents.parse().unwrap();
    root
}

pub fn get_title_text(map: &Element) -> String {
	map.get_child("Project", "argabarga").unwrap()
		.get_child("Title", "argabarga").unwrap().text()
}

pub fn get_directory_id(map: &Element) -> String {
	map.get_child("Drive", "argabarga").unwrap()
		.get_child("Directory", "argabarga").unwrap().text()
}

pub fn set_directory_id(map: &mut Element, id: &String) {
	map.get_child_mut("Drive", "argabarga").unwrap()
		.get_child_mut("Directory", "argabarga").unwrap()
			.append_text_node(id.to_string());
}

pub fn check_existing_files<'a>(map: &'a Element, in_title: &'a String) -> Option<&'a str> {
	let files = map.get_child("Drive", "argabarga").unwrap()
		.get_child("Files", "argabarga").unwrap()
			.children();
	for file in files {
		match file.attr("title") {
			Some(title) => {
				if title == in_title {
					return file.attr("id");
				}
			}
			None => {}
		}
	}
	None
}

pub fn get_files(map: &mut Element) -> &mut Element {
	map.get_child_mut("Drive", "argabarga").unwrap()
		.get_child_mut("Files", "argabarga").unwrap()
}

pub fn make_file<'a>(map: &'a mut Element, id: &'a String, title: &'a String) -> &'a mut Element {
	get_files(map)
		.append_child(Element::builder("File")
		.attr("id", id)
		.attr("title", title)
		.build())	
}

pub fn does_file_exist(map: &Element, in_id: &String) -> bool {
	let files = map.get_child("Drive", "argabarga").unwrap()
		.get_child("Files", "argabarga").unwrap()
			.children();
	let mut found: bool = false;
	for file in files {
		match file.attr("id") {
			Some(id) => {
				if id == in_id {
					return true
				}
			}
			None => {}
		}
	}
	false
}

pub fn get_file_by_id<'b>(map: &'b mut Element, in_id: &'b String) -> Option<&'b mut Element> {
	let files = get_files(map).children_mut();
	let mut found: bool = false;
	for file in files {
		match file.attr("id") {
			Some(id) => {
				if id == in_id {
					found = true;
				}
			}
			None => {}
		}
		if found {
			return Some(file);
		}
	}
	None
}

pub fn get_me_a_file_with_id_and_title<'a>(map: &'a mut Element, in_id: &'a String, in_title: &'a String) -> &'a mut Element {
	if does_file_exist(map, in_id) {
		let ele = get_file_by_id(map, in_id).unwrap();
		ele.delete_children();
		ele.set_attr("title", in_title);
		ele
	} else {
		make_file(map, in_id, in_title)
	}	
}