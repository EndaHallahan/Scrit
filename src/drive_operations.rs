use hyper;
use hyper::net::HttpsConnector;
use hyper::Client;
use hyper_native_tls::NativeTlsClient;
use yup_oauth2;
use yup_oauth2::{Authenticator, FlowType, ApplicationSecret, DiskTokenStorage,
                 DefaultAuthenticatorDelegate, parse_application_secret, TokenStorage};
use google_drive3::{Drive, Result, File};
use std::collections::HashMap;
use std::io::Cursor;
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
	                                       DiskTokenStorage::new(&"token_store.json".to_string())
	                                           .unwrap(),
	                                       Some(FlowType::InstalledInteractive));
	let client = hyper::Client::with_connector(
	    HttpsConnector::new(NativeTlsClient::new().unwrap())
	);
	Drive::new(client, authenticator)
}

pub fn make_document(name: &String, contents: &String, hub: &Drive<hyper::Client, Authenticator<DefaultAuthenticatorDelegate, DiskTokenStorage, Client>>) {
	let mut doc = File::default();
	doc.name = Some(name.to_string());
	doc.mime_type = Some("text/html".to_string());
	match hub.files().create(doc)
		.upload(Cursor::new(contents.as_bytes()), "application/vnd.google-apps.document".parse().unwrap()) 
	{
		Ok(x) => println!("OK! {:?}",x),
		Err(x) => println!("ERROR! {:?}",x)
	}
}

pub fn make_directory(name: String) {

}

pub fn upload(compiled_set: HashMap<String, String>, directory: Option<String>) {
	let hub = get_hub();
	for (name, contents) in compiled_set.iter() {
		make_document(name, contents, &hub);
	}
}

pub fn download() {
	let hub = get_hub();
}