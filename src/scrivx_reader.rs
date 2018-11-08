use std::path::Path;
use std::fs;
use std::io;
use std::io::Read;
use std::env;
use std::error::Error;
use std::fmt;

use quick_xml::Reader;
use quick_xml::events::Event;
use minidom::Element;
use minidom::Node;

use walkdir::WalkDir;

#[derive(Debug)]
pub struct WrongDirError;
impl fmt::Display for WrongDirError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.description())
    }
}
impl Error for WrongDirError {
	 fn description(&self) -> &str {
        "No scrivx found. Check that you are in a .scriv directory."
    }
}

#[derive(Debug)]
pub struct Scrivening {
	title: String,
	id: String,
	include: bool,
	children: Option<Vec<Scrivening>>
}
impl Scrivening {
	pub fn get_title(&self) -> &String {
		&self.title
	}
	pub fn get_id(&self) -> &String  {
		&self.id
	}
	pub fn get_include(&self) -> &bool {
		&self.include
	}
	pub fn get_children(&self) -> &Option<Vec<Scrivening>> {
		&self.children
	}
}

trait NoNamespaceElement {
	fn get_named_child( &self, ele_name: &str) -> Option<&Element>;
	fn get_named_children(&self, ele_name: &str) -> Vec<Option<&Element>>;
}
impl NoNamespaceElement for Element {
	fn get_named_child( &self, ele_name: &str) -> Option<&Element> {
		for e in self.children() {
            if e.name() == ele_name {
                return Some(e);
            }
        }
        None
	}

	fn get_named_children(&self, ele_name: &str) -> Vec<Option<&Element>> {
		let mut children = Vec::new();
		for e in self.children() {
            if e.name() == ele_name {
                children.push(Some(e));
            }
        }
        return children;
	}
}

pub fn get_scrivx() -> Result<String, WrongDirError> {
	let mut scrivx_name = String::new();
	for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
		let filename = entry.file_name().to_string_lossy();
		if filename.ends_with(".scrivx") {
			scrivx_name = filename.to_string();
			break;
		}
	}
	if scrivx_name != "" {
		Ok(scrivx_name)
	} else {
		Err(WrongDirError)
	}
}

pub fn process_scrivx() -> Vec<Scrivening> {
	let mut scrivx_path: String = String::new();
	match get_scrivx() {
		Ok(scrivx) => scrivx_path = scrivx,
		Err(e) => panic!("Error: {}", e.description())
	}
	let mut f = fs::File::open(scrivx_path).expect("File not found!");
	let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Something went wrong reading the file.");
    let root: Element = contents.parse().unwrap();
    let mut outvec: Vec<Scrivening> = Vec::new();
    for child in root.get_named_child("Binder").unwrap().children() {
    	let title: String = child.get_named_child("Title").unwrap().text();
    	let id: String = child.attr("ID").unwrap().to_string();
    	let include: bool = match child.get_named_child("MetaData")
    								.unwrap()
    								.get_named_child("IncludeInCompile") {
    		None => false,
    		Some(t) => t.text().as_str() == "Yes"
    	};
    	let children: Option<Vec<Scrivening>> = make_child_scrivenings(&child);
    	let new_scriv = Scrivening {title, id, include, children};
    	outvec.push(new_scriv);
    } 
    return outvec;
}

fn make_child_scrivenings(ele: &Element) -> Option<Vec<Scrivening>> {
	match ele.get_named_child("Children") {
		None => None,
		Some(e) => {
			let mut outvec: Vec<Scrivening> = Vec::new();
			for child in e.children() {
				let title: String = child.get_named_child("Title").unwrap().text();
		    	let id: String = child.attr("ID").unwrap().to_string();
		    	let include: bool = match child.get_named_child("MetaData").unwrap().get_named_child("IncludeInCompile") {
		    		None => false,
		    		Some(t) => t.text().as_str() == "Yes"
		    	};
				let children: Option<Vec<Scrivening>> = make_child_scrivenings(&child);
				let new_scriv = Scrivening {title, id, include, children};
				outvec.push(new_scriv);
			}
			return Some(outvec);
		}
	}
}