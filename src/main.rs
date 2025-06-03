#![allow(clippy::doc_markdown)]
#![doc = include_str!("../README.md")]

mod schema;

use schema::TakeoutData;

use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::Path;

extern crate google_tasks1 as tasks1;
use crate::hyper_rustls::HttpsConnector;
use crate::hyper_util::client::legacy::connect::HttpConnector;
use crate::hyper_util::client::legacy::Client;
use http_body_util::combinators::BoxBody;
use tasks1::yup_oauth2::authenticator::Authenticator;
use tasks1::{api::Task, hyper_rustls, hyper_util, yup_oauth2, TasksHub};

/// File path for `OAuth2` client secret.
const CLIENT_SECRET_FILE: &str = "client_secret.json";
/// File path for caching the authentication token.
const TOKEN_CACHE_FILE: &str = ".tokencache.json";

/// Setup https client
async fn setup_client() -> (
    Client<HttpsConnector<HttpConnector>, BoxBody<hyper::body::Bytes, tasks1::hyper::Error>>,
    Authenticator<HttpsConnector<HttpConnector>>,
) {
    // Get an ApplicationSecret from `CLIENT_SECRET_FILE`.
    // It contains the `client_id` and `client_secret`, among other things.
    let secret: yup_oauth2::ApplicationSecret =
        yup_oauth2::read_application_secret(Path::new(CLIENT_SECRET_FILE))
            .await
            .expect("client_secret.json not found or unreadable. Place it in the correct path.");

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
#[allow(clippy::field_reassign_with_default)]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get exported tasks
    let takeout_json_path = "Tasks.json";
    let json_content = fs::read_to_string(takeout_json_path)
        .map_err(|e| format!("Failed to read {takeout_json_path}: {e}"))?;

    // Parse as TakeoutData
    let takeout_data_root: TakeoutData = serde_json::from_str(&json_content).map_err(|e| {
        format!(
            "Failed to parse JSON data: {e}. Check TakeoutData/TakeoutTaskList/TakeoutTask struct definitions."
        )
    })?;

    println!("Authentication client set up.");

    let (client, auth) = setup_client().await;
    let hub = TasksHub::new(client, auth);
    let (_resp, task_list) = hub.tasklists().list().doit().await.unwrap();

    let task_id_dict: HashMap<String, String> = task_list
        .items
        .clone()
        .unwrap()
        .iter()
        .map(|task| (task.title.clone().unwrap(), task.id.clone().unwrap()))
        .collect();

    println!("task lists: {task_id_dict:#?}");

    for (list_index, takeout_list) in takeout_data_root.items.iter().enumerate() {
        println!(
            "Processing task list {}/{}: '{}' ({} tasks)...",
            list_index + 1,
            takeout_data_root.items.len(),
            takeout_list.title,
            takeout_list.items.len()
        );

        let target_google_tasklist_id = task_id_dict
            .get(&takeout_list.title)
            .unwrap_or_else(|| todo!("create a task via api when task list is not found"));

        for (task_index, takeout_task_item) in takeout_list.items.iter().enumerate() {
            println!(
                "  Migrating task {}/{}: '{}'",
                task_index + 1,
                takeout_list.items.len(),
                takeout_task_item.title
            );

            let mut new_api_task = Task::default();
            new_api_task.title = Some(takeout_task_item.title.clone());
            new_api_task
                .notes
                .clone_from(&takeout_task_item.description);

            if let Some(due_str) = &takeout_task_item.due_date {
                new_api_task.due = Some(due_str.clone());
            }

            // Status should also match API expectations ("needsAction", "completed").
            new_api_task.status.clone_from(&takeout_task_item.status);

            match hub
                .tasks()
                .insert(new_api_task.clone(), target_google_tasklist_id)
                .doit()
                .await
            {
                Ok((_response, created_task)) => {
                    println!(
                        "  => Success: ID '{}', Title '{}'",
                        created_task.id.as_deref().unwrap_or("N/A"),
                        created_task.title.as_deref().unwrap_or("N/A")
                    );
                }
                Err(e) => {
                    eprintln!(
                        "  => Error: Failed to create task '{}': {}",
                        takeout_task_item.title, e
                    );
                    // Handle specific errors, e.g., rate limits (403 Forbidden or 429 Too Many Requests).
                    // Check the error message details for determination.
                    let error_string = e.to_string().to_lowercase();
                    if error_string.contains("ratelimitexceeded")
                        || error_string.contains("userratelimitexceeded")
                        || error_string.contains("too many requests")
                    {
                        println!("Rate limit likely reached. Waiting for 10 seconds.");
                        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                        // Retry logic could be added here, but it's omitted in this sample.
                        // Decide whether to retry the same task or skip it.
                        eprintln!(
                            "  => Proceeding to the next task without retry (potential skip due to rate limit)."
                        );
                    } else if error_string.contains("invalid")
                        && (error_string.contains("due date") || error_string.contains("due"))
                    {
                        eprintln!(
                            "  => Due date format might be invalid: {:?}. Skipping.",
                            new_api_task.due
                        );
                    }
                }
            }

            // Add a short delay between requests to avoid API rate limits.
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        }
    }

    println!("All task migration processing completed.");

    Ok(())
}
