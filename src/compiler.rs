use std::collections::HashMap;
use push::Document;
use rtf_operations::{process_rtf,write_rtf};
use html_operations::{process_html, write_html};

#[derive(Debug, Clone, PartialEq)]
pub enum Attribute {
	Null,
	ScrivPath,
	Ignorable,
	Italics(bool),
	Bold(bool),
	Strikethrough(bool),
	Smallcaps(bool),
	Underline(bool),
	Subscript(bool),
	Superscript(bool)
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GroupType {
	Null,
	Anchor,
	Document,
	Text,
	Fragment,
	Paragraph,
	Body
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

pub fn rtf_to_html<'t>(rtf: &String) -> String {
	write_html(process_rtf(rtf))
}

fn html_to_rtf(html: &String) /*-> &String*/ {
	write_rtf(process_html(html))
}

pub fn compile<'t>(documents: Vec<Document>, clean: bool) -> String {
	let mut compiled_string: String = String::new();
	for mut doc in documents {
		doc.body_build(clean);
		compiled_string = format!("{}<h2 data-scrivtitle='true'>{}</h2>{}", compiled_string, doc.get_title(), rtf_to_html(doc.get_body()));
	}
	compiled_string = format!("<!DOCTYPE html><body>{}</body></html>", compiled_string);
	compiled_string
}

pub fn decompile() {

}
