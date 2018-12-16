use scrivx_reader;
use scrivx_reader::Scrivening;
use drive_operations;
use compiler;
use map_operations::*;
use std::fs::File;
use std::io::Read;
use minidom::Element;

enum PushState {
	Initial,
	Omit,
	Name,
	Null
}

#[derive(Debug)]
pub struct BodyText {
	id: String,
	body: String
}
impl BodyText {
	pub fn new(id: String, body: String) -> BodyText {
		BodyText {id, body}
	}
	pub fn id(&self) -> &String {
		&self.id
	}
	pub fn body(&self) -> &String {
		&self.body
	}
}

#[derive(Debug)]
pub struct Document {
	title: String,
	contents: Vec<String>, 
	body: Vec<BodyText>,
	location: String
}
impl Document {
	fn new(title: String, contents: Vec<String>) -> Document {
		let body: Vec<BodyText> = Vec::new();
		let location: String = String::new();
		Document {title, contents, body, location}
	}
	pub fn title(&self) -> &String {
		&self.title
	}
	pub fn body(&self) -> &Vec<BodyText> {
		&self.body
	}
	pub fn body_build(&mut self) {
		for id in &self.contents {
			let mut f = File::open(format!("./Files/Docs\\{}.rtf", id)).expect("file not found");
		    let mut contents = String::new();
		    f.read_to_string(&mut contents)
		        .expect("Something went wrong reading the file!");
		    &self.body.push(BodyText::new(id.to_string(), contents));
		}
	}
}

pub struct ScritFile {
	title: String,
	id: String,
	contents: Vec<Document>,
	body: String
}
impl ScritFile {
	pub fn new(contents: Vec<Document>) -> ScritFile {
		let id = String::new();
		let title = String::new();
		let body = String::new();
		ScritFile{title, id, contents, body}
	}
	pub fn title(&self) -> &String {
		&self.title
	}
	pub fn id(&self) -> &String {
		&self.id
	}
	pub fn contents(&self) -> &Vec<Document> {
		&self.contents
	}
	pub fn set_title(&mut self, in_title: String) {
		self.title = in_title;
	}
	pub fn set_id(&mut self, in_id: String) {
		self.id = in_id;
	}
	pub fn body(&self) -> &String {
		&self.body
	}
	pub fn set_body(&mut self, in_body: String) {
		self.body = in_body;
	}
	pub fn body_build(&mut self) {
		for mut doc in &mut self.contents {
			doc.body_build();
		}
		if self.title.is_empty() {
			self.title = self.contents[0].title().to_string();
		}		
	}
}

fn collect_ids(scrivening: &Scrivening, omit: &Option<Vec<String>>, include: &bool) -> Vec<String> {
	let mut out_vec = Vec::new();
	if !include {
		if !scrivening.include() {return out_vec;}
	}
	match omit {
		None => {},
		Some(omits) => {
			if omits.contains(scrivening.title()) {return out_vec;}
		}
	}
	match scrivening.filepath() {
		Some(_) => out_vec.push(scrivening.id().to_string()),
		None => {}
	}
	match scrivening.children() {
		Some(kids) => {
			for kid in kids {
					out_vec.extend(collect_ids(kid, &omit, &include));
				}
			}
		None => {}
	}
	out_vec
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
	let mut name: Option<String> = None;

	/* Process command line arguments */
	let mut state: PushState = PushState::Initial;
	for arg in args {
		match arg.as_str() {
			"-omit" | "-o" => {state = PushState::Omit;},
			"-name" | "-n" => {state = PushState::Name;},
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
					PushState::Name => {
						name = Some(arg.trim().to_string());
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
		let mut new_doc = Document::new(item.title().to_string(), collect_ids(item, &omit, &include));
		doc_list.push(new_doc);
	}

	let mut compiled_set = compiler::compile(doc_list, clean, split, name);	
	/*for scrit_file in &compiled_set {
		println!("{:?}\n", scrit_file.body());
	}*/
	println!("Beginning upload...");
	drive_operations::upload(&mut compiled_set);
	println!("Ok! Upload successful.");

	// Populate map
	println!("Updating map...");
	let mut map = get_map();
	for scrit_file in compiled_set {
		let file_element = get_me_a_file_with_id_and_title(&mut map, scrit_file.id(), scrit_file.title());		
		for doc in scrit_file.contents() {
			let document_node = file_element.append_child(Element::bare("Document"));
			for subdoc in &doc.contents {
				document_node.append_child(Element::builder("ID")
									.append(subdoc)
									.build());
				document_node.append_child(Element::builder("Checksum")
									.append("argabarga")
									.build());
			}
		}
	}
	map.write_to(&mut File::create("Scrit/scrit_map.xml").unwrap());
	
	println!("Done!")
}