use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use yup_oauth2::{Authenticator, FlowType, ApplicationSecret, DiskTokenStorage,
                 DefaultAuthenticatorDelegate, read_application_secret};
use google_drive3::Drive;

pub fn push(args: Vec<String>) {

}

//const CLIENT_SECRET_FILE: &'static str = "ClientSecret.json";

/*
// reads the provided example client secret, the quick and dirty way.
fn read_client_secret(file: String) -> ApplicationSecret {
    read_application_secret(Path::new(&file)).unwrap()
}
*/

	/*
	//Get access to drive
    let secret = read_client_secret(CLIENT_SECRET_FILE.to_string());
    let client = hyper::Client::with_connector(
        HttpsConnector::new(NativeTlsClient::new().unwrap()));
    let authenticator = Authenticator::new(&secret,
                                           DefaultAuthenticatorDelegate,
                                           client,
                                           DiskTokenStorage::new(&"token_store.json".to_string())
                                               .unwrap(),
                                           Some(FlowType::InstalledInteractive));
    let client = hyper::Client::with_connector(
        HttpsConnector::new(NativeTlsClient::new().unwrap()));

    let hub = Drive::new(client, authenticator);

    */

   	//scrivx_reader::prep_files("Woop".to_string());

