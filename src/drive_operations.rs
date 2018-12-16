use map_operations::*;
use push::ScritFile;
use hyper;
use hyper::net::HttpsConnector;
use hyper::Client;
use hyper_native_tls::NativeTlsClient;
use yup_oauth2::{Authenticator, FlowType, ApplicationSecret, DiskTokenStorage,
                 DefaultAuthenticatorDelegate, parse_application_secret};
use google_drive3::{Drive, File};
use std::io::Cursor;
use std::fs;
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

pub fn make_document(name: &String, contents: &String, dir_id: &String, 
	hub: &Drive<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, DiskTokenStorage, Client>>
) -> String {
	let mut doc = File::default();
	doc.name = Some(name.to_string());
	doc.mime_type = Some("application/vnd.google-apps.document".to_string());
	doc.parents = Some(vec![dir_id.clone()]);
	match hub.files().create(doc)
		.param("fields", "id")
		.upload(Cursor::new(contents.as_bytes()), "text/html".parse().unwrap()) 
	{
		Ok((_, y)) => {
			println!("OK! Successfully uploaded '{}'.", name);
			y.id.unwrap()
		},
		Err(x) => {panic!("ERROR! {:?}",x);}
	}
}

pub fn update_document(name: &String, contents: &String, dir_id: &String, file_id: &str, 
	hub: &Drive<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, DiskTokenStorage, Client>>
) -> String {
	let mut doc = File::default();
	doc.mime_type = Some("application/vnd.google-apps.document".to_string());
	match hub.files().update(doc, file_id)
		.param("fields", "id")
		.upload(Cursor::new(contents.as_bytes()), "text/html".parse().unwrap()) 
	{
		Ok((_, y)) => {
			println!("OK! Successfully updated '{}'.", name);
			y.id.unwrap()
		},
		Err(x) => {panic!("ERROR! {:?}",x);}
	}
}

pub fn make_directory(name: String, 
	hub: &Drive<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, DiskTokenStorage, Client>>
) -> String {
	let mut dir = File::default();
	dir.name = Some(name.to_string());
	dir.mime_type = Some("application/vnd.google-apps.folder".to_string());
	match hub.files().create(dir)
		.param("fields", "id")
		.upload(Cursor::new(name.as_bytes()), "application/vnd.google-apps.folder".parse().unwrap()) 
	{
		Ok((_, y)) => {
			println!("OK! Successfully created directory '{}'.", name);
			y.id.unwrap()
		},
		Err(x) => {panic!("ERROR! {:?}",x)}
	}
}

pub fn check_file_in_folder(file_id: &str, folder_id : &String, file_name: &String,
	hub: &Drive<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, DiskTokenStorage, Client>>
) -> bool {
	match hub.files().get(file_id).param("fields", "parents, trashed").doit() {
		Ok((_, y)) => {
			match y.parents {
				Some(parents) => {
					if !y.trashed.unwrap() && parents.contains(folder_id) {
						true
					} else {
						println!("Couldn't find file '{}' in project folder, creating new file...", file_name);
						false
					}	
				}
				None => {
					println!("Couldn't find file '{}' in project folder, creating new file...", file_name);
					false
				}
			}
		},
		Err(_) => {
			println!("Couldn't find file '{}' in project folder, creating new file...", file_name);
			false
		}
	}
}

pub fn check_folder(folder_id : &String,
	hub: &Drive<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, DiskTokenStorage, Client>>
) -> bool {
	match hub.files().get(folder_id).param("fields", "trashed").doit() {
		Ok((_, y)) => {
			if !y.trashed.unwrap() {
				true
			} else {
				println!("Couldn't find project folder, creating new directory...");
				false
			}
		},
		Err(x) => {
			println!("Couldn't find project folder, creating new directory...");
			false
		}
	}
}

pub fn upload(compiled_set: &mut Vec<ScritFile>) {
	let hub = get_hub();
	let mut map = get_map();
	let title = get_title_text(&map);
	let mut dir_id = get_directory_id(&map);
	if dir_id.is_empty() || !check_folder(&dir_id, &hub) {
		dir_id = make_directory(title.to_string(), &hub);
		set_directory_id(&mut map, &dir_id);
	}
	for scrit_file in compiled_set {
		let mut file_id: String;
		match check_existing_files(&mut map, scrit_file.title()) {
			Some(ele) => {
				if check_file_in_folder(ele.attr("id").unwrap(), &dir_id, scrit_file.title(), &hub) {
					file_id = update_document(scrit_file.title(), scrit_file.body(), &dir_id, ele.attr("id").unwrap(), &hub);
				} else {
					file_id = make_document(scrit_file.title(), scrit_file.body(), &dir_id, &hub);
					replace_file(ele, &file_id, scrit_file.title());
				}		
			},
			None => {file_id = make_document(scrit_file.title(), scrit_file.body(), &dir_id, &hub);}
		}
		scrit_file.set_id(file_id);
	}	
	map.write_to(&mut fs::File::create("Scrit/scrit_map.xml").unwrap());
}

pub fn download() {
	let hub = get_hub();
}



