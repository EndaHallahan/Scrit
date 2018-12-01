extern crate hyper;
extern crate hyper_native_tls;
extern crate yup_oauth2;
extern crate google_drive3;
extern crate quick_xml;
extern crate minidom;
extern crate walkdir;
extern crate rctree;

mod scrivx_reader;
mod drive_operations;
mod rtf_operations;
mod html_operations;
mod compiler;
mod client_info;
mod tree;
mod init;
mod push;
mod pull;
mod help;
mod update;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if !(args.len() > 1) {return;}
    match args[1].as_str() {
    	"init" => init::init(),
    	"push" => push::push(&args[2..]),
    	"pull" => pull::pull(args),
    	"tree" => tree::binder_tree(),
    	"help" => help::help(args),
    	"update" => update::update(),
    	"version" => {
    		let version: &'static str = env!("CARGO_PKG_VERSION");
    		println!("Scrit version {}", version);
    	},
    	_ => println!("Unknown command '{}'. Type 'scrit help' for a list of valid commands.", args[1].as_str())
    }	
}