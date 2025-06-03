# GoogleTasksImporter
Import tasks to the Google tasks from json file.

This program is designed to migrate Google Tasks data from a JSON file (exported via Google Takeout) to another Google Account using the Google Tasks API and Rust. 

> [!WARNING]
> This code was written for my personal use and may not cover all possible scenarios. Please feel free to modify it to suit your needs.
> Important: This script directly manipulates tasks. Please review the code carefully before execution.

## Prerequisites
1. Rust Development Environment: Ensure you have the latest stable version of Rust installed. You can get it from [www.rust-lang.org](https://www.rust-lang.org/).
1. Google Cloud Platform (GCP) Project:
    - A GCP project is required to enable the Google Tasks API and obtain OAuth 2.0 credentials.
1. Google Takeout Tasks JSON File: You need the Tasks.json file exported from your source Google Account via Google Takeout.

## Getting started

### Clone this repository
```sh
$ git clone https://github.com/Alignof/GoogleTasksImporter.git
```

### Google Cloud Platform (GCP) Setup
To use the Google Tasks API, you need to set up a project in GCP and get OAuth 2.0 credentials.

a. Create or Select a GCP Project:
1. Go to the Google Cloud Console.
1. Create a new project or select an existing one.

b. Enable the Google Tasks API:
1. In the GCP Console, navigate to "APIs & Services" > "Library".
1. Search for "Google Tasks API" and enable it for your project.

c. Configure OAuth Consent Screen:
1. Go to "APIs & Services" > "OAuth consent screen".
1. Choose "External" for User Type if you're using a personal Gmail account, or "Internal" if you're within a Google Workspace organization. Click "Create".
1. Fill in the required information:
    - App name: e.g., "Rust Tasks Migrator"
    - User support email: Your email address.
    - Developer contact information: Your email address.
1. Click "Save and Continue" through the "Scopes" and "Test users" sections. For scopes, you don't need to add them here; the application will request them.
1. For "Test users", add the Google Account email address you intend to migrate tasks to. This is important if your app is in "testing" publishing status.
1. Review the summary and go back to the dashboard.

d. Create OAuth 2.0 Client ID:
1. Go to "APIs & Services" > "Credentials".
1. Click "+ CREATE CREDENTIALS" and select "OAuth client ID".
1. For "Application type", select "Desktop app".
1. Give it a name, e.g., "Tasks Migrator Desktop Client".
1. Click "Create".
1. A dialog will appear showing your "Client ID" and "Client secret". Click "DOWNLOAD JSON" to download the client secret file.
1. Rename this downloaded file to client\_secret.json and place it in the root directory of your Rust project.

> [!IMPORTANT]
> Treat client\_secret.json as a sensitive file. Do NOT commit it to your Git repository.

### Create task lists
Ensure that you create task lists in the destination with the same names as the task lists in the source you are migrating from.
If you have a large number of task lists, it is recommended that you implement the todo macro section yourself.

### Run the script
> [!WARNING]
> Review the code carefully before execution.

```sh
$ cargo run
```

