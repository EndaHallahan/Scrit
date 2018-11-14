use scrivx_reader;
use scrivx_reader::Scrivening;
use drive_operations;
use std::collections::HashMap;

#[derive(Debug)]
enum PushArgument {
	Omit(Vec<String>),
	Include(Vec<String>),
	Split,
	Break(i32),
	Clean,
	Directory(String),
	Item(String)
}

enum PushState {
	Initial,
	Omit,
	Include,
	Break,
	Directory,
	Null
}

pub fn push(args: &[String]) {
	let arguments: Vec<PushArgument> = process_args(args);
	let blueprint: Vec<Scrivening> = scrivx_reader::process_scrivx();
	for arg in arguments {
		match arg {
			PushArgument::Item(name) => {
				println!("{:?}",scrivx_reader::get_scrivening(&name, &blueprint));
			},
			_ => {}
		}
	}
	//let hub = drive_operations::get_hub();
}

fn process_args(args: &[String]) -> Vec<PushArgument> {
	let mut state: PushState = PushState::Initial;
	let mut options = Vec::new();
	for arg in args {
		match arg.as_str() {
			"-break" | "-b" => {state = PushState::Break;},
			"-omit" | "-o" => {state = PushState::Omit;},
			"-include" | "-i" => {state = PushState::Include;},
			"-directory" | "-d" => {state = PushState::Directory;},
			"-split" | "-s" => {
				options.push(PushArgument::Split);
				state = PushState::Null;
			},
			"-clean" | "-c" => {
				options.push(PushArgument::Clean);
				state = PushState::Null;
			},	
			_ => {
				match state {
					PushState::Initial => {
						options.push(PushArgument::Item(arg.to_string()));
					},
					PushState::Omit => {
						let arg_list: Vec<String> = arg.trim().split(',').map(|s| s.trim().to_string()).collect();
						options.push(PushArgument::Omit(arg_list));
					},
					PushState::Include => {
						let arg_list: Vec<String> = arg.trim().split(',').map(|s| s.trim().to_string()).collect();
						options.push(PushArgument::Include(arg_list));
					}
					PushState::Break => {
						options.push(PushArgument::Break(arg.parse().unwrap()));
					},
					PushState::Directory => {
						options.push(PushArgument::Directory(arg.to_string()));
					},
					_ => {println!("Invalid argument: {}", arg);}
				}
				state = PushState::Null;
			}
		}
	}

	options
}

/*
scrit push <files> <options>

Options:
-omit (-o) 		Omit specified files from compilation. Argument is a comma-separated 
				 list of file names or ids.
-include (-i)	Ignore files' include/exclude value from compile when compiling.
-split (-s)		Split files into separate documents on break point.
-break (-b)		Breaks documents in specified point. In normal operation, this adds 
				 page breaks and titles. Behaviour is different if -s is 
				 signified. Argument is an integer. Default is no breaks.
-clean (-c)		Pushes to GDocs without break placeholders. Documents exported 
				 in this manner cannot be pulled back into Scrivener.
-directory (-d)	Specefies a filepath in the Google Drive to upload to. Defaults to the root.
*/

