#![doc = include_str!("../README.md")]

mod schema;

use std::error::Error;
use std::fs;
use std::path::Path;

extern crate google_tasks1 as tasks1;
use crate::hyper_rustls::HttpsConnector;
use crate::hyper_util::client::legacy::connect::HttpConnector;
use crate::hyper_util::client::legacy::Client;
use http_body_util::combinators::BoxBody;
use tasks1::yup_oauth2::authenticator::Authenticator;
use tasks1::{hyper_rustls, hyper_util, yup_oauth2, FieldMask, TasksHub};

use schema::TakeoutData;

/// File path for OAuth2 client secret.
const CLIENT_SECRET_FILE: &str = "client_secret.json";
/// File path for caching the authentication token.
const TOKEN_CACHE_FILE: &str = ".tokencache.json";

/// Setup https client
async fn setup_client() -> (
    Client<HttpsConnector<HttpConnector>, BoxBody<hyper::body::Bytes, tasks1::hyper::Error>>,
    Authenticator<HttpsConnector<HttpConnector>>,
) {
    // Get an ApplicationSecret instance by some means. It contains the `client_id` and
    // `client_secret`, among other things.
    let secret: yup_oauth2::ApplicationSecret =
        yup_oauth2::read_application_secret(Path::new(CLIENT_SECRET_FILE))
            .await
            .expect("client_secret.json not found or unreadable. Place it in the correct path.");

    // Instantiate the authenticator. It will choose a suitable authentication flow for you,
    // unless you replace  `None` with the desired Flow.
    // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
    // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
    // retrieve them from storage.
    let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .persist_tokens_to_disk(Path::new(TOKEN_CACHE_FILE))
    .build()
    .await
    .unwrap();

    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new())
        .build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .unwrap()
                .https_or_http()
                .enable_http1()
                .build(),
        );

    (client, auth)
}

/// Only read the json file for now.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get exported tasks
    let takeout_json_path = "Tasks.json";
    let json_content = fs::read_to_string(takeout_json_path)
        .map_err(|e| format!("Failed to read {}: {}", takeout_json_path, e))?;

    // Parse as TakeoutData
    let takeout_data_root: TakeoutData = serde_json::from_str(&json_content).map_err(|e| {
        format!(
            "Failed to parse JSON data: {}. Check TakeoutData/TakeoutTaskList/TakeoutTask struct definitions.",
            e
        )
    })?;

    let _my_tasks = &takeout_data_root.items[0];
    let want_to_do_list = &takeout_data_root.items[1];

    println!("Authentication client set up.");

    let (client, auth) = setup_client().await;
    let mut hub = TasksHub::new(client, auth);
    Ok(())
}
