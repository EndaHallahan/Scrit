use hyper;
use hyper::net::HttpsConnector;
use hyper::Client;
use hyper_native_tls::NativeTlsClient;
use yup_oauth2;
use yup_oauth2::{Authenticator, FlowType, ApplicationSecret, DiskTokenStorage,
                 DefaultAuthenticatorDelegate, parse_application_secret, TokenStorage};
use google_drive3::Drive;

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