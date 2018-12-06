extern crate hyper;
extern crate hyper_native_tls;
extern crate yup_oauth2;
extern crate google_drive3;
extern crate minidom;
extern crate walkdir;
extern crate rctree;

mod scrivx_reader;
mod drive_operations;
mod map_operations;
mod rtf_operations;
mod html_operations;
mod compiler;
mod client_info;
mod tree;
mod push;
mod pull;
mod help;
mod update;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if !(args.len() > 1) {return;}
    match args[1].as_str() {
    	"init" => map_operations::init(),
    	"push" => push::push(&args[2..]),
    	"pull" => pull::pull(&args[2..]),
    	"tree" => tree::binder_tree(),
    	"help" => help::help(&args[2..]),
    	"update" => update::update(),
    	"info" => {
    		let version: &'static str = env!("CARGO_PKG_VERSION");
    		println!("
Scrit - A push/pull interface between Scrivener and Google Docs
Version {}
Scrit was created and is maintained by Enda Hallahan. Source code available at https://github.com/EndaHallahan/Scrit
Scrit is protected under an MIT license. Scrit is not affiliated with Google LLC or Literature & Latte Ltd.
Scrivener © Literature & Latte Ltd. Google Drive and Google Docs © Google LLC. Please don't sue me.
    			", version);
    	},
    	_ => println!("Unknown command '{}'. Type 'scrit help' for a list of valid commands.", args[1].as_str())
    }	
}