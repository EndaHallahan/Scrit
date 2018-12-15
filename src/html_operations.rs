use rctree::{Node, NodeEdge};
use std::cell::Ref;
use compiler::{Attribute, ASTElement, GroupType};

struct HTMLReader {

}
impl HTMLReader {
	
}

struct HTMLWriter {
	output_string: String,
	in_body: bool
}
impl HTMLWriter {
	fn new () -> HTMLWriter {
		let output_string = String::new();
		let in_body = false;
		HTMLWriter{output_string, in_body}
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
		let mut tag: &str;
		let mut attributes = String::new();
		let mut styles = "style='".to_string();
		match element.ele_type() {
			GroupType::Text | GroupType::Fragment => {tag = "span";},
			GroupType::Paragraph => {tag = "p";},
			GroupType::Hr => {tag = "hr";},
			//GroupType::Document => "html",
			GroupType::Body => {self.in_body = true; return;},
			_ => return
		};
		if !self.in_body {return;}
		let atts = element.attributes();
		for att in atts {
			match *att {
				Attribute::Italics(true) => {
					styles = format!("{}font-style:italic;", styles);
				},
				Attribute::Bold(true) => {
					styles = format!("{}font-weight:bold;", styles);
				},
				Attribute::Underline(true) => {
					styles = format!("{}text-decoration-line:underline;", styles);
				},
				Attribute::Strikethrough(true) => {
					styles = format!("{}text-decoration-line:line-through;", styles);
				},
				Attribute::Smallcaps(true) => {
					styles = format!("{}font-variant:small-caps;", styles);
				},
				Attribute::Superscript(true) => {
					styles = format!("{}vertical-align:super;font-size:smaller;", styles);
				},
				Attribute::Subscript(true) => {
					styles = format!("{}vertical-align:sub;font-size:smaller;", styles);
				},
				Attribute::FontSize(val) => {
					styles = format!("{}font-size:{}pt;", styles, (val/2).to_string());
				},
				_ => {}
			}
		}
		let tag_string = format!("<{}{} {}'>", tag, attributes, styles);
		self.output_string = format!("{}{}{}", self.output_string, tag_string, element.text_contents());
	}
	fn end_element(&mut self, element: Ref<ASTElement>) {
		let tag: &str = match element.ele_type() {
			GroupType::Text => "</span>",
			GroupType::Paragraph => "</p><br>",
			GroupType::Hr => "</hr>",
			GroupType::Body => {self.in_body = false; return;},
			_ => return
		};
		if !self.in_body {return;}
		self.output_string = format!("{}{}", self.output_string, tag);
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
