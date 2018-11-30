use std::collections::HashMap;
use rctree::{Node, NodeEdge};
use std::str::Chars;
use std::cell::Ref;
use std::fmt;
use compiler::{Attribute, ASTElement, GroupType};

struct HTMLReader {

}
impl HTMLReader {
	
}

struct HTMLWriter {
	output_string: String
}
impl HTMLWriter {
	fn new () -> HTMLWriter {
		let ast = Node::new(ASTElement::new(GroupType::Null));
		let output_string = String::new();
		HTMLWriter{output_string}
	}
	fn write(&mut self, ast: Node<ASTElement>) -> String {
		for node_edge in ast.traverse() {
			match node_edge {
				NodeEdge::Start(node) => {self.start_element(node.borrow())},
				NodeEdge::End(node) => {self.end_element(node.borrow())}
			}
		}	
		self.output_string.clone()
	}
	fn start_element(&mut self, element: Ref<ASTElement>) {
		let mut tag: &str = "";
		let mut attributes = String::new();
		let mut styles = "style='".to_string();
		match element.ele_type() {
			GroupType::Text | GroupType::Fragment => {tag = "span";},
			GroupType::Paragraph => {tag = "p";},
			GroupType::ScrivPath => {
				tag = "div";
				attributes = format!("{} {}", attributes, "data-scrivpath='true'");
			},
			//GroupType::Document => "html",
			//GroupType::Body => "body",
			_ => return
		};
		let atts = element.attributes();
		for att in atts {
			match *att {
				Attribute::Italics(true) => {
					styles = format!("{}{}", styles, "font-style:italic;");
				}
				Attribute::Bold(true) => {
					styles = format!("{}{}", styles, "font-weight:bold;");
				}
				_ => {}
			}
		}
		let tag_string = format!("<{}{} {}'>", tag, attributes, styles);
		self.output_string = format!("{}{}{}", self.output_string, tag_string, element.text_contents());
	}
	fn end_element(&mut self, element: Ref<ASTElement>) {
		let tag: &str = match element.ele_type() {
			GroupType::Text => "span",
			GroupType::Paragraph => "p",
			GroupType::ScrivPath => "div",
			_ => return
		};
		let tag_string = format!("</{}>", tag);
		self.output_string = format!("{}{}", self.output_string, tag_string);
	}
}

pub fn process_html(html: &String) -> Node<ASTElement> {
	let mut document_root: Node<ASTElement> = Node::new(ASTElement::new(GroupType::Document));
	document_root
}


pub fn write_html(dom: Node<ASTElement>) -> String {
	let mut writer = HTMLWriter::new();
	writer.write(dom)
}
