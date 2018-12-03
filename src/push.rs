use scrivx_reader;
use scrivx_reader::Scrivening;
use drive_operations;
use compiler;
use init::check_init;
use std::fs;
use std::fs::File;
use std::io::Read;

use rtf_operations;

enum PushState {
	Initial,
	Omit,
	Include,
	Directory,
	Null
}

#[derive(Debug)]
pub struct Document {
	title: String,
	contents: Vec<String>, 
	body: String,
}
impl Document {
	fn new(title: String, contents: Vec<String>) -> Document {
		let body: String = String::new();
		Document {title, contents, body}
	}
	pub fn get_title(&self) -> &String {
		&self.title
	}
	pub fn get_contents(&self) -> &Vec<String> {
		&self.contents
	}
	pub fn get_body(&self) -> &String {
		&self.body
	}
	pub fn body_build(&mut self, clean: bool) {
		for filepath in &self.contents {
			let mut f = File::open(filepath).expect("file not found");
		    let mut contents = String::new();
		    f.read_to_string(&mut contents)
		        .expect("Something went wrong reading the file!");
		    if !clean {
		    	&self.body.push_str(&format!("{{\\scrivpath {{[[[{}]]]}}}}", filepath.trim_left_matches("./Files/Docs\\")));
		    }
		    &self.body.push_str(&contents);
		}
	}
}

pub fn push(args: &[String]) {
	if !check_init() {
		println!("
Scrit must be initialized for this project before you can use this command.
Type 'scrit init' to intialize, or type 'scrit help init' for more information.
			");
		return;
	}
	let blueprint: Vec<Scrivening> = scrivx_reader::process_scrivx();
	
	let mut exports: Vec<&Scrivening> = Vec::new();
	let mut omit: Option<Vec<String>> = None;
	let mut include: bool = false;
	let mut split: bool = false;
	let mut clean: bool = false;
	let mut directory: Option<String> = None;

	/* Process command line arguments */
	let mut state: PushState = PushState::Initial;
	for arg in args {
		match arg.as_str() {
			"-omit" | "-o" => {state = PushState::Omit;},
			"-directory" | "-d" => {state = PushState::Directory;},
			"-include" | "-i" => {
				include = true;
				state = PushState::Null;
			},
			"-split" | "-s" => {
				split = true;
				state = PushState::Null;
			},
			"-clean" | "-c" => {
				clean = true;
				state = PushState::Null;
			},
			"Binder" => {
				for item in &blueprint {exports.push(&item);}
				state = PushState::Null;
			},
			_ => {
				match state {
					PushState::Initial => {
						match scrivx_reader::get_scrivening(&arg.trim().to_string(), &blueprint) {
							None => {println!("File {} not found!", arg);}
							Some(scrivening) => {exports.push(scrivening);}
						}
					},
					PushState::Omit => {
						let arg_list: Vec<String> = arg.trim().split(',').map(|s| s.trim().to_string()).collect();
						omit = Some(arg_list);
						state = PushState::Null;
					},
					PushState::Directory => {
						directory = Some(arg.trim().to_string());
						state = PushState::Null;
					},
					_ => {
						println!("Invalid argument: {}", arg);
						state = PushState::Null;
					}
				}
			}
		}
	}
	if exports.is_empty() {println!("No documents selected for push!"); return;}
	let mut doc_list: Vec<Document> = Vec::new();
	for item in exports {
		let mut new_doc = Document::new(item.get_title().to_string(), collect_filepaths(item, &omit, &include));
		doc_list.push(new_doc);
	}
	export(doc_list, split, clean, directory);	
}

fn collect_filepaths(scrivening: &Scrivening, omit: &Option<Vec<String>>, include: &bool) -> Vec<String> {
	let mut out_vec = Vec::new();
	if !include {
		if !scrivening.get_include() {return out_vec;}
	}
	match omit {
		None => {},
		Some(omits) => {
			if omits.contains(scrivening.get_title()) {return out_vec;}
		}
	}
	match scrivening.get_filepath() {
		Some(filepath) => out_vec.push(filepath.to_string()),
		None => {}
	}
	match scrivening.get_children() {
		Some(kids) => {
			for kid in kids {
					out_vec.extend(collect_filepaths(kid, &omit, &include));
				}
			}
		None => {}
	}
	out_vec
}

fn export (documents: Vec<Document>, split: bool, clean: bool, directory: Option<String>) {
	let compiled_set = compiler::compile(documents, clean, split);	
	for (a,b) in compiled_set.iter() {
		println!("{}:\n{}\n\n", a, b);
	}
	drive_operations::upload(compiled_set, directory);
}

/*
scrit push <files> <options>

Options:
-omit (-o) 		Omit specified files from compilation. Argument is a comma-separated 
				 list of file names or ids.
-include (-i)	Ignore files' include/exclude value from compile when compiling.
-split (-s)		Split pushed files into separate documents.
-clean (-c)		Pushes to GDocs without break placeholders. Documents exported 
				 in this manner cannot be pulled back into Scrivener.
-directory (-d)	Specefies a filepath in the Google Drive to upload to. Defaults to the root.
*/

