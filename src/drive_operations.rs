use init::get_map;
use hyper;
use hyper::net::HttpsConnector;
use hyper::Client;
use hyper_native_tls::NativeTlsClient;
use yup_oauth2;
use yup_oauth2::{Authenticator, FlowType, ApplicationSecret, DiskTokenStorage,
                 DefaultAuthenticatorDelegate, parse_application_secret, TokenStorage};
use google_drive3::{Drive, Result, File};
use std::collections::HashMap;
use std::io::{Cursor, Write};
use std::fs;
use minidom::Element;
use client_info::CLIENT_SECRET;

fn read_client_secret(client_info: &'static str) -> ApplicationSecret {
    parse_application_secret(&client_info.to_string()).unwrap()
}

pub fn get_hub() -> Drive<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, DiskTokenStorage, Client>> {
	let secret = read_client_secret(CLIENT_SECRET);
	let client = hyper::Client::with_connector(
	    HttpsConnector::new(NativeTlsClient::new().unwrap()));
	let authenticator = Authenticator::new(&secret,
	                                       DefaultAuthenticatorDelegate,
	                                       client,
	                                       DiskTokenStorage::new(&"Scrit/token_store.json".to_string())
	                                           .unwrap(),
	                                       Some(FlowType::InstalledInteractive));
	let client = hyper::Client::with_connector(
	    HttpsConnector::new(NativeTlsClient::new().unwrap())
	);
	Drive::new(client, authenticator)
}

pub fn make_document(name: &String, contents: &String, dir_id: &String, hub: &Drive<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, DiskTokenStorage, Client>>) {
	let mut doc = File::default();
	doc.name = Some(name.to_string());
	doc.mime_type = Some("text/html".to_string());
	doc.parents = Some(vec![dir_id.clone()]);
	match hub.files().create(doc)
		.upload(Cursor::new(contents.as_bytes()), "application/vnd.google-apps.document".parse().unwrap()) 
	{
		Ok(x) => println!("OK! Successfully uploaded {}.", name),
		Err(x) => println!("ERROR! {:?}",x)
	}
}

pub fn make_directory(name: String, hub: &Drive<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, DiskTokenStorage, Client>>)
-> String {
	let mut dir = File::default();
	dir.name = Some(name.to_string());
	dir.mime_type = Some("application/vnd.google-apps.folder".to_string());
	match hub.files().create(dir)
		.param("fields", "id")
		.upload(Cursor::new(name.as_bytes()), "application/vnd.google-apps.folder".parse().unwrap()) 
	{
		Ok((x, y)) => {
			println!("OK! Successfully created directory '{}'", name);
			y.id.unwrap()
		},
		Err(x) => {println!("ERROR! {:?}",x); String::new()}
	}
}

pub fn upload(compiled_set: HashMap<String, String>, directory: Option<String>) {
	let hub = get_hub();
	let mut map = get_map();
	let title = map.get_child("Project", "argabarga").unwrap()
		.get_child("Title", "argabarga").unwrap().text();
	{
		let mut map_id = map.get_child_mut("Drive", "argabarga").unwrap()
			.get_child_mut("Directory", "argabarga").unwrap();
		if  map_id.text().is_empty() {
			map_id.append_text_node(make_directory(title.to_string(), &hub));			
		}
		let dir_id = map_id.text();
		for (name, contents) in compiled_set.iter() {
			make_document(name, contents, &dir_id, &hub);
		}
	}
	map.write_to(&mut fs::File::create("Scrit/scrit_map.xml").unwrap());
}

pub fn download() {
	let hub = get_hub();
}