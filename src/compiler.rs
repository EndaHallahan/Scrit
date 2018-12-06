use std::collections::HashMap;
use push::{Document, ScritFile};
use rtf_operations::{process_rtf, write_rtf};
use html_operations::{process_html, write_html};

#[derive(Debug, Clone, PartialEq)]
pub enum Attribute {
	Ignorable,
	Italics(bool),
	Bold(bool),
	Strikethrough(bool),
	Smallcaps(bool),
	Underline(bool),
	Subscript(bool),
	Superscript(bool),
	FontSize(i32)
}

#[derive(Debug, PartialEq, Clone)]
pub enum GroupType {
	Null,
	Anchor,
	Document,
	Text,
	Fragment,
	Paragraph,
	Body,
	ScrivPath,
	List(char),
	ListItem,
	FontTable,
	Font(String),
	ColourTable,
	Colour,
	ListTable,
	ListLabel,
	ListOverrideTable,
	Hr
}

#[derive(Debug, Clone)]
pub struct ASTElement {
	attributes: Vec<Attribute>,
	ele_type: GroupType,
	text_contents: String
}
impl<'a> ASTElement {
	pub fn new(ele_type: GroupType) -> ASTElement {
		let attributes = Vec::new();
		let text_contents = String::new();
		ASTElement{attributes, ele_type, text_contents}
	}
	pub fn ele_type(&self) -> &GroupType {
		&self.ele_type
	}
	pub fn set_ele_type(&mut self, new_type: GroupType) {
		self.ele_type = new_type;
	}
	pub fn add_att(&mut self, att: Attribute) {
		self.attributes.push(att);
	}
	pub fn add_text(&mut self, new_text: &String) {
		self.text_contents = format!("{}{}", self.text_contents, new_text);
	}
	pub fn text_contents(&self) -> &String {
		&self.text_contents
	}
	pub fn attributes(&self) -> &Vec<Attribute> {
		&self.attributes
	}
}

fn rtf_to_html<'t>(rtf: &String) -> String {
	write_html(process_rtf(rtf))
}

fn html_to_rtf(html: &String) /*-> &String*/ {
	write_rtf(process_html(html))
}

pub fn compile(documents: Vec<Document>, clean: bool, split: bool) -> Vec<ScritFile> {
	let mut scrit_file_list: Vec<ScritFile> = Vec::new();
	if !split {
		scrit_file_list.push(ScritFile::new(documents));
	} else {
		for doc in documents {
			scrit_file_list.push(ScritFile::new(vec![doc]));
		}
	}
	for mut scrit_file in &mut scrit_file_list {
		scrit_file.body_build(clean);
		let mut compiled_string = String::new();
		for doc in scrit_file.contents() {		
			compiled_string.push_str(&format!("<h2 data-scrivtitle='true'>{}</h2>{}", doc.title(), rtf_to_html(doc.body())));
		}
		scrit_file.set_body(format!("<!DOCTYPE html><body>{}</body></html>", compiled_string));
	}
	scrit_file_list
}

pub fn decompile() {

}
