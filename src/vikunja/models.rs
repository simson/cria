use serde::{Deserialize, Deserializer, Serialize};
use chrono::{DateTime, Utc, Datelike};

#[derive(Deserialize)]
#[serde(untagged)]
enum PercentDoneValue {
    Integer(i64),
    Float(f64),
    String(String),
}

fn normalize_percent_done(value: f64) -> Option<u8> {
    if !value.is_finite() || value < 0.0 {
        return None;
    }

    let normalized = if value <= 1.0 { value * 100.0 } else { value };
    Some(normalized.round().clamp(0.0, 100.0) as u8)
}

pub(crate) fn deserialize_optional_percent_done<'de, D>(deserializer: D) -> Result<Option<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<PercentDoneValue>::deserialize(deserializer)?;
    Ok(match opt {
        None => None,
        Some(PercentDoneValue::Integer(value)) => normalize_percent_done(value as f64),
        Some(PercentDoneValue::Float(value)) => normalize_percent_done(value),
        Some(PercentDoneValue::String(value)) => {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                None
            } else {
                trimmed.parse::<f64>().ok().and_then(normalize_percent_done)
            }
        }
    })
}

fn deserialize_optional_datetime<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<String> = Option::deserialize(deserializer)?;
    match opt {
        Some(s) if s.is_empty() => Ok(None),
        Some(s) => {
            match DateTime::parse_from_rfc3339(&s) {
                Ok(dt) => {
                    // Check if it's the epoch start or year 1 (typical for null dates)
                    if dt.year() <= 1900 {
                        Ok(None)
                    } else {
                        Ok(Some(dt.with_timezone(&Utc)))
                    }
                }
                Err(_) => Ok(None),
            }
        }
        None => Ok(None),
    }
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)] // API response fields may not all be used
pub struct Project {
    pub id: i64,
    pub title: String,
    pub hex_color: String,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)] // API response fields may not all be used
pub struct Label {
    pub id: i64,
    pub title: String,
    pub hex_color: Option<String>,
    pub description: Option<String>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub created_by: Option<User>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(dead_code)] // API response fields may not all be used
pub struct User {
    pub id: i64,
    pub username: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub created: Option<String>,
    pub updated: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)] // API response fields may not all be used
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub done: bool,
    pub done_at: Option<String>,
    pub project_id: i64,
    pub labels: Option<Vec<Label>>,
    pub assignees: Option<Vec<User>>,
    pub priority: Option<i32>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    pub due_date: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<String>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub created_by: Option<User>,
    #[serde(default, alias = "percentDone", deserialize_with = "deserialize_optional_percent_done")]
    pub percent_done: Option<u8>,
    pub is_favorite: bool,
    pub position: Option<i64>,
    pub index: Option<i64>,
    pub identifier: Option<String>,
    pub hex_color: Option<String>,
    pub cover_image_attachment_id: Option<i64>,
    pub bucket_id: Option<i64>,
    pub buckets: Option<Vec<Bucket>>,
    pub attachments: Option<Vec<Attachment>>,
    pub comments: Option<Vec<Comment>>,
    pub reactions: Option<std::collections::HashMap<String, Vec<User>>>,
    pub related_tasks: Option<std::collections::HashMap<String, Vec<Task>>>,
    pub reminders: Option<Vec<Reminder>>,
    pub repeat_after: Option<i64>,
    pub repeat_mode: Option<i64>,
    pub subscription: Option<Subscription>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)] // API response fields may not all be used
pub struct Attachment {
    pub id: i64,
    pub task_id: i64,
    pub created: Option<String>,
    pub created_by: Option<User>,
    pub file: Option<FileAttachment>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)] // API response fields may not all be used
pub struct FileAttachment {
    pub id: i64,
    pub name: Option<String>,
    pub mime: Option<String>,
    pub size: Option<i64>,
    pub created: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)] // API response fields may not all be used
pub struct Comment {
    pub id: i64,
    pub author: Option<User>,
    pub comment: Option<String>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub reactions: Option<std::collections::HashMap<String, Vec<User>>>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)] // API response fields may not all be used
pub struct Bucket {
    pub id: i64,
    pub title: Option<String>,
    pub position: Option<i64>,
    pub limit: Option<i64>,
    pub count: Option<i64>,
    pub project_view_id: Option<i64>,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub created_by: Option<User>,
    pub tasks: Option<Vec<Task>>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)] // API response fields may not all be used
pub struct Reminder {
    pub reminder: Option<String>,
    pub relative_to: Option<String>,
    pub relative_period: Option<i64>,
}

#[derive(Deserialize, Debug, Clone)]
#[allow(dead_code)] // API response fields may not all be used
pub struct Subscription {
    pub id: i64,
    pub entity: Option<i64>,
    pub entity_id: Option<i64>,
    pub created: Option<String>,
}

impl Default for Task {
    fn default() -> Self {
        Task {
            id: 0,
            title: String::new(),
            description: None,
            done: false,
            done_at: None,
            project_id: 0,
            labels: None,
            assignees: None,
            priority: None,
            due_date: None,
            start_date: None,
            end_date: None,
            created: None,
            updated: None,
            created_by: None,
            percent_done: None,
            is_favorite: false,
            position: None,
            index: None,
            identifier: None,
            hex_color: None,
            cover_image_attachment_id: None,
            bucket_id: None,
            buckets: None,
            attachments: None,
            comments: None,
            reactions: None,
            related_tasks: None,
            reminders: None,
            repeat_after: None,
            repeat_mode: None,
            subscription: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Task;

    #[test]
    fn test_task_deserializes_fractional_percent_done() {
        let json = r##"{
            "id": 32,
            "title": "Move todo to vikunja",
            "description": "",
            "done": false,
            "done_at": "0001-01-01T00:00:00Z",
            "project_id": 3,
            "labels": null,
            "assignees": null,
            "priority": 0,
            "due_date": "0001-01-01T00:00:00Z",
            "start_date": "0001-01-01T00:00:00Z",
            "end_date": "0001-01-01T00:00:00Z",
            "created": "2026-04-22T14:53:48Z",
            "updated": "2026-04-22T15:04:02Z",
            "created_by": {
                "id": 1,
                "username": "simeon",
                "name": "",
                "email": null,
                "created": "2026-04-22T09:00:32Z",
                "updated": "2026-04-22T12:22:24Z"
            },
            "percent_done": 0.1,
            "is_favorite": false,
            "position": 0,
            "index": 2,
            "identifier": "#2",
            "hex_color": "",
            "cover_image_attachment_id": 0,
            "bucket_id": 0,
            "buckets": null,
            "attachments": null,
            "comments": null,
            "reactions": null,
            "related_tasks": {},
            "reminders": null,
            "repeat_after": 0,
            "repeat_mode": 0,
            "subscription": null,
            "future_web_field": { "nested": true }
        }"##;

        let task: Task = serde_json::from_str(json).unwrap();
        assert_eq!(task.percent_done, Some(10));
    }

    #[test]
    fn test_task_deserializes_camel_case_percent_done() {
        let json = r##"{
            "id": 1,
            "title": "Task",
            "description": null,
            "done": false,
            "done_at": null,
            "project_id": 1,
            "labels": null,
            "assignees": null,
            "priority": null,
            "due_date": null,
            "start_date": null,
            "end_date": null,
            "created": null,
            "updated": null,
            "created_by": null,
            "percentDone": "25",
            "is_favorite": false,
            "position": null,
            "index": null,
            "identifier": null,
            "hex_color": null,
            "cover_image_attachment_id": null,
            "bucket_id": null,
            "buckets": null,
            "attachments": null,
            "comments": null,
            "reactions": null,
            "related_tasks": null,
            "reminders": null,
            "repeat_after": null,
            "repeat_mode": null,
            "subscription": null,
            "added_by_future_version": [1, 2, 3]
        }"##;

        let task: Task = serde_json::from_str(json).unwrap();
        assert_eq!(task.percent_done, Some(25));
    }
}

impl Task {
    #[allow(dead_code)]
    pub fn to_vikunja_task(&self) -> crate::vikunja_client::tasks::VikunjaTask {
        crate::vikunja_client::tasks::VikunjaTask {
            id: Some(self.id as u64),
            title: self.title.clone(),
            description: self.description.clone(),
            done: Some(self.done),
            priority: self.priority.map(|p| p as u8),
            due_date: self.due_date,
            start_date: self.start_date,
            project_id: self.project_id as u64,
            labels: self.labels.as_ref().map(|labels| labels.iter().map(|l| crate::vikunja_client::tasks::VikunjaLabel {
                id: Some(l.id as u64),
                title: l.title.clone(),
                hex_color: l.hex_color.clone(),
            }).collect()),
            assignees: self.assignees.as_ref().map(|assignees| assignees.iter().map(|a| crate::vikunja_client::VikunjaUser {
                id: Some(a.id as u64),
                username: a.username.clone(),
                name: a.name.clone(),
                email: a.email.clone(),
            }).collect()),
            is_favorite: Some(self.is_favorite),
        }
    }
    pub fn from_vikunja_task(vikunja_task: crate::vikunja_client::tasks::VikunjaTask) -> Self {
        Self {
            id: vikunja_task.id.unwrap_or(0) as i64,
            title: vikunja_task.title,
            description: vikunja_task.description,
            done: vikunja_task.done.unwrap_or(false),
            done_at: None,
            project_id: vikunja_task.project_id as i64,
            labels: vikunja_task.labels.map(|labels| labels.into_iter().map(|l| Label {
                id: l.id.unwrap_or(0) as i64,
                title: l.title,
                hex_color: l.hex_color,
                description: None,
                created: None,
                updated: None,
                created_by: None,
            }).collect()),
            assignees: vikunja_task.assignees.map(|assignees| assignees.into_iter().map(|a| User {
                id: a.id.unwrap_or(0) as i64,
                username: a.username,
                name: a.name,
                email: a.email,
                created: None,
                updated: None,
            }).collect()),
            priority: vikunja_task.priority.map(|p| p as i32),
            due_date: vikunja_task.due_date,
            start_date: vikunja_task.start_date,
            end_date: None,
            created: None,
            updated: None,
            created_by: None,
            percent_done: None,
            is_favorite: vikunja_task.is_favorite.unwrap_or(false),
            position: None,
            index: None,
            identifier: None,
            hex_color: None,
            cover_image_attachment_id: None,
            bucket_id: None,
            buckets: None,
            attachments: None,
            comments: None,
            reactions: None,
            related_tasks: None,
            reminders: None,
            repeat_after: None,
            repeat_mode: None,
            subscription: None,
        }
    }
}
