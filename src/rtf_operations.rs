use std::collections::HashMap;
use rctree::Node;
use std::str::Chars;
use std::rc::Rc;
use compiler::{Attribute, ASTElement, GroupType};

const WIN_1252: [char; 255] = [' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',' ',
	' ','!','\"','#','$','%','&','\'','(',')','*','+',',','-','.','/',
	'0','1','2','3','4','5','6','7','8','9',':',';','<','=','>','@','A',
	'B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R',
	'S','T','U','V','W','X','Y','Z','[','\\',']','^','_','`','a','b','c',
	'd','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t',
	'u','v','w','x','y','z','{','|','}','~',' ','€','�','‚','ƒ','„','…',
	'†','‡','ˆ','‰','Š','‹','Œ','�','Ž','�','�','‘','’','“','”','•','–',
	'—','˜','™','š','›','œ','�','ž','Ÿ',' ','¡','¢','£','¤','¥','¦','§',
	'¨','©','ª','«','-','®','¯','°','±','²','³','´','µ','¶','·','¸','¹',
	'º','»','¼','½','¾','¿','À','Á','Â','Ã','Ä','Å','Æ','Ç','È','É','Ê',
	'Ë','Ì','Í','Î','Ï','Ð','Ñ','Ò','Ó','Ô','Õ','Ö','×','Ø','Ù','Ú','Û',
	'Ü','Ý','Þ','ß','à','á','â','ã','ä','å','æ','ç','è','é','ê','ë','ì',
	'í','î','ï','ð','ñ','ò','ó','ô','õ','ö','÷','ø','ù','ú','û','ü','ý',
	'þ','ÿ'];

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
impl RTFReader {
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

struct RTFBuilder {
	current_instruction: Instruction,
	current_node: Node<ASTElement>,
	def_char_state: Vec<Attribute>,
	def_par_state: Vec<Attribute>,
	last_paragraph: Node<ASTElement>,
	anchor: Node<ASTElement>,
	skip: i32,
}
impl RTFBuilder {
	fn new() -> RTFBuilder {	
		let current_instruction = Instruction::Null;
		let anchor = Node::new(ASTElement::new(GroupType::Anchor));
		let current_node = Node::new(ASTElement::new(GroupType::Document));
		let def_char_state = Vec::new();
		let def_par_state = Vec::new();
		let last_paragraph = Node::new(ASTElement::new(GroupType::Null));
		let skip = 0;
		RTFBuilder{current_instruction, current_node, def_char_state, def_par_state, last_paragraph, anchor, skip}
	}
	fn build(&mut self, instructions: &Vec<Instruction>) -> Node<ASTElement> {
		self.current_node = Node::new(ASTElement::new(GroupType::Document));
		self.anchor.append(Node::new(ASTElement::new(GroupType::Document)));
		self.current_node = self.anchor.first_child().unwrap();
		for instruction in instructions {
			//println!("{:?}", instruction);
			self.execute(instruction);
		}

		/*for node in self.current_node.root().descendants() {
			println!("{:?}",node.borrow());
		}*/
		
		self.current_node.root()
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
				} else if self.current_node.borrow_mut().ele_type() == &GroupType::Text {
					self.current_node.borrow_mut().add_text(&param);
				} else {
					self.new_group(GroupType::Fragment);
					self.current_node.borrow_mut().add_text(&param);
					self.end_group();
				}
			}
			Instruction::GroupStart => {self.new_group(GroupType::Null);}
			Instruction::GroupEnd => {self.end_group();}
			Instruction::Ignorable => {self.current_node.borrow_mut().add_att(Attribute::Ignorable)}
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
		let mut att_value = 0;
		let mut control_name = control;
		for (i, c) in control.chars().enumerate() {
			if c.is_digit(10) {
				let (a, b) = control.split_at(i);
				control_name = a;
				att_value = match b.parse() {
					Ok(val) => val,
					Err(_) => 1
				};
				break;
			}
		}

		//Need to find a better way of doing this; hashmaps let me down.
		match control_name {
			"b" => self.cmd_b(att_value),
			"i" =>self.cmd_i(att_value),
			"strike" =>self.cmd_strike(att_value),
			"scaps" =>self.cmd_scaps(att_value),
			"ul" =>self.cmd_ul(att_value),
			"ulnone" =>self.cmd_ulnone(),
			"sub" =>self.cmd_sub(),
			"super" =>self.cmd_super(),
			"nosupersub" =>self.cmd_nosupersub(),
			"fs" =>self.cmd_fs(att_value),
			"par" => self.cmd_par(),
			"pgnrestart" => self.cmd_pgnrestart(),
			"scrivpath" => self.cmd_scrivpath(),
			"emdash" => self.cmd_emdash(),
			"endash" => self.cmd_endash(),
			"tab" => self.cmd_tab(),
			"line" => self.cmd_line(),
			"hrule" => self.cmd_hrule(),
			_ => {}
		}
	}

	fn parse_hex(&mut self, hex: &String) {
		let re_hex = (i64::from_str_radix(hex, 16).unwrap()) as usize;
		if self.current_node.borrow_mut().ele_type() == &GroupType::Text {
			self.current_node.borrow_mut().add_text(&WIN_1252[re_hex].to_string());
		} else {
			self.new_group(GroupType::Fragment);
			self.current_node.borrow_mut().add_text(&WIN_1252[re_hex].to_string());
			self.end_group();
		}
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

	fn cmd_emdash(&mut self) {
		self.current_node.borrow_mut().add_text(&"—".to_string());
	}
	fn cmd_endash(&mut self) {
		self.current_node.borrow_mut().add_text(&"–".to_string());
	}
	fn cmd_tab(&mut self) {
		self.current_node.borrow_mut().add_text(&"\t".to_string());
	}
	fn cmd_line(&mut self) {
		self.current_node.borrow_mut().add_text(&"\n".to_string());
	}
	fn cmd_hrule(&mut self) {
		self.new_group(GroupType::Hr);
		self.end_group();
	}


	fn cmd_b(&mut self, val: i32) {
		self.current_node.borrow_mut().add_att(Attribute::Bold(val == 1));
	}
	fn cmd_i(&mut self, val: i32) {
		self.current_node.borrow_mut().add_att(Attribute::Italics(val == 1));
	}
	fn cmd_strike(&mut self, val: i32) {
		self.current_node.borrow_mut().add_att(Attribute::Strikethrough(val == 1));
	}
	fn cmd_scaps(&mut self, val: i32) {
		self.current_node.borrow_mut().add_att(Attribute::Smallcaps(val == 1));
	}
	fn cmd_ul(&mut self, val: i32) {
		self.current_node.borrow_mut().add_att(Attribute::Underline(val == 1));
	}
	fn cmd_ulnone(&mut self) {
		self.current_node.borrow_mut().add_att(Attribute::Underline(false));
	}
	fn cmd_sub(&mut self) {
		self.current_node.borrow_mut().add_att(Attribute::Subscript(true));
	}
	fn cmd_super(&mut self) {
		self.current_node.borrow_mut().add_att(Attribute::Superscript(true));
	}
	fn cmd_nosupersub(&mut self) {
		self.current_node.borrow_mut().add_att(Attribute::Superscript(false));
		self.current_node.borrow_mut().add_att(Attribute::Subscript(false));
	}
	fn cmd_fs(&mut self, val:i32) {
		self.current_node.borrow_mut().add_att(Attribute::FontSize(val));
	}

	fn cmd_pgnrestart(&mut self) {
		while self.current_node.borrow().ele_type() != &GroupType::Document {
			self.end_group();
		}
		self.new_group(GroupType::Body);
	}

	fn cmd_par(&mut self) {
		while self.current_node.borrow().ele_type() != &GroupType::Body {
			self.end_group();
		}
		self.new_group(GroupType::Paragraph);
	}

	fn cmd_scrivpath(&mut self) {
		self.new_group(GroupType::ScrivPath);
	}
}

struct RTFWriter {

}
impl RTFWriter {

}

pub fn process_rtf(rtf: &String) -> Node<ASTElement> {
	let mut reader = RTFReader::new();
	let mut builder = RTFBuilder::new();	
	builder.build(reader.read(&rtf))
}

pub fn write_rtf(dom: Node<ASTElement>) /*-> &String*/ {
	
}