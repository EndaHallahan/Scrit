use std::collections::HashMap;
use rctree::Node;
use std::str::Chars;
use compiler::{Attribute, DOMElement};

enum ReadMode {
	ParseText,
	ParseEscape,
	ParseControl,
	ParseHex
}

struct RTFReader<'b> {
	mode: ReadMode,
	rtf: &'b String
}
impl<'b> RTFReader<'b> {
	fn new(rtf: &String) -> RTFReader {
		let mode = ReadMode::ParseText;
		RTFReader{rtf, mode}
	}
	fn read(&mut self) {
		let rtf_chars = self.rtf.chars();
		for character in rtf_chars {
			match &self.mode {
				ReadMode::ParseText => {self.parse_text(character);},
				ReadMode::ParseEscape => {self.parse_escape(character);},
				ReadMode::ParseControl => {self.parse_control(character);},
				ReadMode::ParseHex => {self.parse_hex(character);}
			}
		}
	}
	fn parse_text(&mut self, character: char) {
		match character {
			'\\' => {self.mode = ReadMode::ParseEscape},
			'{' => {}
			'}' => {}
			'\n' | '\r' => {}
			_ => {}
		}
	}
	fn parse_escape(&mut self, character: char) {

	}
	fn parse_control(&mut self, character: char) {
		
	}
	fn parse_hex(&mut self, character: char) {
		
	}
}

struct RTFWriter {

}
impl RTFWriter {

}

pub fn process_rtf(rtf: &String) -> Node<DOMElement> {
	let mut document_root: Node<DOMElement> = Node::new(DOMElement::new("document"));
	document_root
}

pub fn write_rtf(dom: Node<DOMElement>) /*-> &String*/ {
	
}








/*
pub fn rtf_to_html(rtf: &String) -> &String {
	build_html(read_rtf(rtf))
}

pub fn html_to_rtf(html: &String) -> &String {
	build_rtf(read_html(html))
}



fn read_rtf(rtf: &String) -> &String {

}

fn build_rtf(dom: &String) -> &String {

}



fn read_html(html: &String) -> &String {

}

fn build_html(dom: &String) -> &String {

}
*/

