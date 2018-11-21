use std::collections::HashMap;
use rctree::Node;
use std::str::Chars;
use compiler::{Attribute, DOMElement};

struct HTMLReader {

}
impl HTMLReader {
	
}

struct HTMLWriter {

}
impl HTMLWriter {
	
}

pub fn process_html(html: &String) -> Node<DOMElement> {
	let mut document_root: Node<DOMElement> = Node::new(DOMElement::new("document"));
	document_root
}


pub fn write_html(dom: Node<DOMElement>) /*-> &String*/ {

}
