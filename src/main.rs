extern crate hyper;
extern crate hyper_native_tls;
extern crate yup_oauth2;
extern crate google_drive3;
extern crate quick_xml;
extern crate minidom;
extern crate walkdir;

//mod scrivx_reader;

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

use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;

use yup_oauth2::{Authenticator, FlowType, ApplicationSecret, DiskTokenStorage,
                 DefaultAuthenticatorDelegate, read_application_secret};
use google_drive3::Drive;

const CLIENT_SECRET_FILE: &'static str = "ClientSecret.json";

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

#[derive(Debug)]
struct Scrivening {
	title: String,
	id: String,
	include: bool,
	children: Option<Vec<Scrivening>>
}

#[derive(Debug)]
struct WrongDirError;
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

fn get_scrivx() -> Result<String, WrongDirError> {
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

fn process_scrivx() -> Vec<Scrivening> {
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

fn list_scriv_contents(blueprint: &Vec<Scrivening>, depth: i32, active_levels: &mut Vec<bool>) {
	for (index, scrivening) in blueprint.iter().enumerate() {
		let mut out: String = String::new();
		let mut i: i32 = 0;
		while i < depth {
			if i == depth - 1 {
				if index == blueprint.len() - 1 {
					out.push_str("└── ");
					active_levels[(depth - 1) as usize] = false;
				} else {
					out.push_str("├── ");
				}	
			} else {
				if active_levels[i as usize] {
					out.push_str("|   ");
				} else {
					out.push_str("    ");
				}	
			}		
			i += 1;
		}
		out.push_str(&format!("{} [{}]", &scrivening.title, &scrivening.id));
		println!("{}", out);
		match scrivening.children {
			None => continue,
			Some(ref children) => {
				if depth >= active_levels.len() as i32{
					active_levels.push(true);
				} else {
					active_levels[depth as usize] = true;
				}
				list_scriv_contents(&children, depth + 1, active_levels);
			}
		}
	}
}

fn init() {
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

fn push(args: Vec<String>) {

}

fn pull(args: Vec<String>) {

}

fn binder_tree() {
	let ref blueprint: Vec<Scrivening> = process_scrivx();
	println!("{}", "Binder");
	list_scriv_contents(&blueprint, 1, &mut vec![true]);
}

fn help(args: Vec<String>) {

}



fn main() {
    let args: Vec<String> = env::args().collect();
    if !(args.len() > 1) {return;}
    match args[1].as_str() {
    	"init" => init(),
    	"push" => push(args),
    	"pull" => pull(args),
    	"tree" => binder_tree(),
    	"help" => help(args),
    	"version" => return,
    	_ => println!("Unknown command '{}'. Type 'help' for a list of valid commands.", args[1].as_str())
    }	
}




/*
// reads the provided example client secret, the quick and dirty way.
fn read_client_secret(file: String) -> ApplicationSecret {
    read_application_secret(Path::new(&file)).unwrap()
}
*/

	/*
	//Get access to drive
    let secret = read_client_secret(CLIENT_SECRET_FILE.to_string());
    let client = hyper::Client::with_connector(
        HttpsConnector::new(NativeTlsClient::new().unwrap()));
    let authenticator = Authenticator::new(&secret,
                                           DefaultAuthenticatorDelegate,
                                           client,
                                           DiskTokenStorage::new(&"token_store.json".to_string())
                                               .unwrap(),
                                           Some(FlowType::InstalledInteractive));
    let client = hyper::Client::with_connector(
        HttpsConnector::new(NativeTlsClient::new().unwrap()));

    let hub = Drive::new(client, authenticator);

    */

   	//scrivx_reader::prep_files("Woop".to_string());

