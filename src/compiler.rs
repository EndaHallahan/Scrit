use std::collections::HashMap;
use rtf_operations::{process_rtf,write_rtf};
use html_operations::{process_html, write_html};

#[derive(Debug, Clone)]
pub enum Attribute {
	Null,
	AttColor(i32, i32, i32),
	AttBoolean(bool),
	AttString(String),
	AttInteger(i32),
	AttVec(Vec<String>)
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GroupType {
	Null,
	Anchor,
	Document,
	Text,
	Fragment,
	Paragraph
}

#[derive(Debug, Clone)]
pub struct ASTElement<'a> {
	attributes: HashMap<&'a str, Attribute>,
	ele_type: GroupType,
	text_contents: String
}
impl<'a> ASTElement<'a> {
	pub fn new(ele_type: GroupType) -> ASTElement<'a> {
		let attributes = HashMap::new();
		let text_contents = String::new();
		ASTElement{attributes, ele_type, text_contents}
	}
	pub fn ele_type(&self) -> &GroupType {
		&self.ele_type
	}
	pub fn set_ele_type(&mut self, new_type: GroupType) {
		self.ele_type = new_type;
	}
	pub fn add_att(&mut self, name:&'a str, value: Attribute) {
		self.attributes.insert(name, value);
	}
	pub fn add_text(&mut self, new_text: &String) {
		self.text_contents = format!("{}{}", self.text_contents, new_text);
	}
}

fn rtf_to_html(rtf: &String) /*-> &String*/ {
	write_html(process_rtf(rtf))
}

fn html_to_rtf(html: &String) /*-> &String*/ {
	write_rtf(process_html(html))
}

pub fn compile () {

}

pub fn decompile() {

}
