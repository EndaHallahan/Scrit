use std::collections::HashMap;
use rctree::Node;
use std::str::Chars;
use compiler::{Attribute, DOMElement};

#[derive(Debug, PartialEq)]
enum ReadMode {
	ParseText,
	ParseEscape,
	ParseControl,
	ParseHex
}

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
	Null,
	Control(String),
	Hex(String),
	Text(String),
	GroupStart,
	GroupEnd,
	Ignorable,
	ListBreak,
	Break
}

struct RTFReader<'b> {
	mode: ReadMode,
	rtf: &'b String,
	current_instruction: Instruction,
	instructions: Vec<Instruction>
}
impl<'b> RTFReader<'b> {
	fn new(rtf: &String) -> RTFReader {
		let mode = ReadMode::ParseText;
		let current_instruction = Instruction::Null;
		let instructions = Vec::new();
		RTFReader{rtf, mode, current_instruction, instructions}
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
			'{' => {
				self.set_cur_instruction();
				self.set_new_instruction(Instruction::GroupStart);
			}
			'}' => {
				self.set_cur_instruction();
				self.set_new_instruction(Instruction::GroupEnd);
			}
			'\n' | '\r' => {
				self.set_cur_instruction();
				self.set_new_instruction(Instruction::Break)
			}
			_ => {
				match self.current_instruction {
					Instruction::Text(ref mut contents) => {
						contents.push(character);
					},
					_ => {
						self.current_instruction = Instruction::Text(character.to_string());
					}
				}
			}
		}
	}
	fn parse_escape(&mut self, character: char) {
		match character {
			' '|'\\'|'{'|'}'|'\n'|'\r'|'\t' => {
				self.mode = ReadMode::ParseText;
				match self.current_instruction {
					Instruction::Text(ref mut contents) => {
						contents.push(character);
					},
					_ => {
						self.set_cur_instruction();
						self.current_instruction = Instruction::Text(character.to_string());
					}
				}
			},
			_ => {
				self.set_cur_instruction();
				self.mode = ReadMode::ParseControl;
				self.parse_control(character)
			}
		}
	}
	fn parse_control(&mut self, character: char) {
		match character {
			'*' => {
				self.set_new_instruction(Instruction::Ignorable);
			},
			'\'' => {
				self.mode = ReadMode::ParseHex;
				self.current_instruction = Instruction::Hex(String::new());
			},
			' ' => {
				self.set_cur_instruction();
				self.mode = ReadMode::ParseText;
			},
			';' => {
				self.set_cur_instruction();
				self.set_new_instruction(Instruction::ListBreak);
				self.mode = ReadMode::ParseText;
			},
			'\\'|'{'|'}'|'\n'|'\r'|'\t' => {
				self.set_cur_instruction();
				self.mode = ReadMode::ParseText;
				self.parse_text(character);
			},
			_ => {
				match self.current_instruction {
					Instruction::Control(ref mut contents) => {
						contents.push(character);
					},
					_ => {
						self.current_instruction = Instruction::Control(character.to_string());
					}
				}
			}
		}
	}
	fn parse_hex(&mut self, character: char) {
		match self.current_instruction {
			Instruction::Hex(ref mut contents) => {
				if contents.len() < 2 {
					contents.push(character);
					return;
				} 
			}
			_ => {}
		}
		self.set_cur_instruction();
		self.mode = ReadMode::ParseText;
		self.parse_text(character);
	}
	fn set_cur_instruction(&mut self) {
		if self.current_instruction != Instruction::Null {
			self.instructions.push(self.current_instruction.clone());
			self.current_instruction = Instruction::Null;
		}
	}
	fn set_new_instruction(&mut self, instruction: Instruction) {
		self.instructions.push(instruction.clone());
		self.current_instruction = Instruction::Null;
	}
}

struct RTFBuilder {

}
impl RTFBuilder {
	
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

