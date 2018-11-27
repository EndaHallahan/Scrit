use std::collections::HashMap;
use rctree::Node;
use std::str::Chars;
use std::rc::Rc;
use compiler::{Attribute, ASTElement, GroupType};



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

struct RTFReader {
	mode: ReadMode,
	current_instruction: Instruction,
	instructions: Vec<Instruction>
}
impl<'b> RTFReader {
	fn new() -> RTFReader {
		let mode = ReadMode::ParseText;
		let current_instruction = Instruction::Null;
		let instructions = Vec::new();
		RTFReader{mode, current_instruction, instructions}
	}
	fn read(&mut self, rtf: &String) -> &Vec<Instruction> {
		let rtf_chars = rtf.chars();
		for character in rtf_chars {
			match &self.mode {
				ReadMode::ParseText => {self.parse_text(character);},
				ReadMode::ParseEscape => {self.parse_escape(character);},
				ReadMode::ParseControl => {self.parse_control(character);},
				ReadMode::ParseHex => {self.parse_hex(character);}
			}
		}
		&self.instructions
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

pub fn test(rtf: &String) {
	let mut anchor: Rc<Node<ASTElement>> = Rc::new(Node::new(ASTElement::new(GroupType::Null)));
	let mut reader = RTFReader::new();
	let mut builder = RTFBuilder::new();
	
	builder.build(reader.read(rtf));
}

struct RTFBuilder<'c> {
	current_instruction: Instruction,
	current_node: Node<ASTElement<'c>>,
	def_char_state: HashMap<&'c str, Attribute>,
	def_par_state: HashMap<&'c str, Attribute>,
	last_paragraph: Node<ASTElement<'c>>,
	anchor: Node<ASTElement<'c>>,
	skip: i32,
}
impl<'c> RTFBuilder<'c> {
	fn new() -> RTFBuilder<'c> {	
		let current_instruction = Instruction::Null;
		let anchor = Node::new(ASTElement::new(GroupType::Anchor));
		let current_node = Node::new(ASTElement::new(GroupType::Document));
		let def_char_state = HashMap::new();
		let def_par_state = HashMap::new();
		let last_paragraph= Node::new(ASTElement::new(GroupType::Null));
		let skip = 0;
		RTFBuilder{current_instruction, current_node, def_char_state, def_par_state, last_paragraph, anchor, skip}
	}
	fn build(&mut self, instructions: &Vec<Instruction>) /*-> Node<ASTElement<'c>>*/ {
		self.current_node = Node::new(ASTElement::new(GroupType::Document));
		self.anchor.append(Node::new(ASTElement::new(GroupType::Document)));
		self.current_node = self.anchor.first_child().unwrap();
		for instruction in instructions {
			self.execute(instruction);
		}
		for kid in self.current_node.root().descendants() {
			println!("{:?}", kid);
		}
	}

	fn execute(&mut self, instruction: &Instruction) {
		if self.skip >0 {
			self.skip -= 1;
			return;
		}
		match instruction {
			Instruction::Control(param) => {self.parse_control(&param);},
			Instruction::Text(param) => {
				if self.current_node.borrow_mut().ele_type() == &GroupType::Null {
					self.current_node.borrow_mut().set_ele_type(GroupType::Text);
					self.current_node.borrow_mut().add_text(&param);
				} else {
					self.new_group(GroupType::Fragment);
					self.current_node.borrow_mut().add_text(&param);
					self.end_group();
				}
			}
			Instruction::GroupStart => {self.new_group(GroupType::Null);}
			Instruction::GroupEnd => {self.end_group();}
			Instruction::Ignorable => {self.current_node.borrow_mut().add_att("Ignorable", Attribute::AttBoolean(true))}
			Instruction::Hex(param) => {self.parse_hex(&param);}
			Instruction::Break => {
				if self.current_node.borrow_mut().ele_type() == &GroupType::Fragment {
					self.end_group();
				}
			}
			Instruction::ListBreak => {}
			_ => {}
		}
	}

	fn parse_control(&mut self, control: &str) {
		let mut att_value = Attribute::Null;
		let mut control_name = control;
		for (i, c) in control.chars().enumerate() {
			if c.is_digit(10) {
				let (a, b) = control.split_at(i);
				control_name = a;
				att_value = Attribute::AttInteger(b.parse().unwrap());
				break;
			}
		}
		
		//Move elsewhere—compile time lookup table via rust-phf?
		let mut control_map: HashMap<&str, Box<FnMut(Attribute)>> = HashMap::new();
		control_map.insert("par", Box::new(|_|{
			while self.current_node.borrow().ele_type() != &GroupType::Document {
				self.end_group();
			}
			self.new_group(GroupType::Paragraph);
		}));
		control_map.insert("b", Box::new(|val|{
			
		}));
		

		match control_map.get_mut(control_name) {
			Some(funct) => {funct(att_value)},
			None => {}
		}

	}

	fn parse_hex(&mut self, hex: &String) {

	}

	fn new_group(&mut self, ele_type: GroupType) {
		self.current_node.append(Node::new(ASTElement::new(ele_type)));
		self.current_node = self.current_node.last_child().unwrap();
	}

	fn end_group(&mut self) {
		match self.current_node.parent() {
			None => {},
			Some(parent) => {self.current_node = parent;}
		};
	}	
}

struct RTFWriter {

}
impl RTFWriter {

}

pub fn process_rtf(rtf: &String) -> Node<ASTElement> {
	let mut document_root: Node<ASTElement> = Node::new(ASTElement::new(GroupType::Document));
	document_root
}

pub fn write_rtf(dom: Node<ASTElement>) /*-> &String*/ {
	
}