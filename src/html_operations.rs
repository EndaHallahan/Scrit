use std::collections::HashMap;
use rctree::Node;
use std::str::Chars;
use compiler::{Attribute, ASTElement, GroupType};

struct HTMLReader {

}
impl HTMLReader {
	
}

struct HTMLWriter {

}
impl HTMLWriter {
	
}

pub fn process_html(html: &String) -> Node<ASTElement> {
	let mut document_root: Node<ASTElement> = Node::new(ASTElement::new(GroupType::Document));
	document_root
}


pub fn write_html(dom: Node<ASTElement>) /*-> &String*/ {

}
