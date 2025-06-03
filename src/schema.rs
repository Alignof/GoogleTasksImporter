//! Definition of Google Takeout json schema

use serde::Deserialize;

/// Represents an individual task item from the Google Takeout JSON.
#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct TakeoutTask {
    /// The ID of the task within the Takeout data (different from the ID after migration).
    pub id: Option<String>,
    /// The title of the task.
    pub title: String,
    /// The description or notes for the task.
    #[serde(rename = "notes")]
    pub description: Option<String>,
    /// The due date of the task in RFC3339 format (e.g., "YYYY-MM-DDTHH:MM:SS.mmmZ").
    #[serde(rename = "due")]
    pub due_date: Option<String>,
    /// The status of the task (e.g., "needsAction", "completed").
    pub status: Option<String>,
    /// The creation timestamp in RFC3339 format.
    pub created: Option<String>,
    /// The last update timestamp in RFC3339 format.
    pub updated: Option<String>,
    /// The completion timestamp in RFC3339 format, if the task is completed.
    pub completed: Option<String>,
    /// The type of the task (e.g., "`PERSONAL_TASK`").
    #[serde(rename = "task_type")]
    pub task_type: Option<String>,
    /// A link to the task in Google Tasks (from Takeout).
    #[serde(rename = "selfLink")]
    pub self_link: Option<String>,
}

/// Represents a task list (e.g., "My Tasks") from the Google Takeout JSON.
#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct TakeoutTaskList {
    /// The kind of the object, typically "tasks#tasks".
    pub kind: String,
    /// The ID of the task list within the Takeout data.
    pub id: String,
    /// The title of the task list (e.g., "My Tasks").
    pub title: String,
    /// The last update timestamp for the task list in RFC3339 format.
    pub updated: String,
    /// An array of task items within this list.
    pub items: Vec<TakeoutTask>,
    /// A link to the task list in Google Tasks (from Takeout).
    #[serde(rename = "selfLink")]
    pub self_link: Option<String>,
}

/// Represents the root structure of the Google Takeout Tasks JSON file.
#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct TakeoutData {
    /// The kind of the object, typically "tasks#taskLists".
    pub kind: String,
    /// An array of task lists.
    pub items: Vec<TakeoutTaskList>,
}
