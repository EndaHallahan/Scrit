use std::collections::HashMap;
use rtf_operations::{process_rtf,write_rtf};
use html_operations::{process_html, write_html};

#[derive(Debug)]
pub enum Attribute {
	AttColor(i32, i32, i32),
	AttBoolean(bool),
	AttString(String),
	AttInteger(i32),
	AttVec(Vec<String>)
}

#[derive(Debug)]
pub struct DOMElement<'a> {
	attributes: HashMap<&'a str, Attribute>,
	ele_type: &'a str
}
impl<'a> DOMElement<'a> {
	pub fn new(ele_type: &'a str) -> DOMElement<'a> {
		let attributes = HashMap::new();
		DOMElement{attributes, ele_type}
	}
	pub fn add_att(&mut self, name:&'a str, value: Attribute) {
		self.attributes.insert(name, value);
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
