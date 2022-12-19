use std::collections::HashMap;

use crate::internal::prelude::*;
use crate::json::{from_number, Value};
use crate::model::id::ForumTagId;

#[derive(Debug, Clone, Default)]
pub struct EditThread(pub HashMap<&'static str, Value>);

impl EditThread {
    /// The name of the thread.
    ///
    /// **Note**: Must be between 2 and 100 characters long.
    pub fn name<D: ToString>(&mut self, name: D) -> &mut Self {
        self.0.insert("name", Value::from(name.to_string()));

        self
    }

    /// Duration in minutes to automatically archive the thread after recent activity.
    ///
    /// **Note**: Can only be set to 60, 1440, 4320, 10080 currently.
    pub fn auto_archive_duration(&mut self, duration: u16) -> &mut Self {
        self.0.insert("auto_archive_duration", from_number(duration));

        self
    }

    /// The archive status of the thread.
    ///
    /// **Note**: A thread that is `locked` can only be unarchived if the user has the `MANAGE_THREADS` permission.
    pub fn archived(&mut self, archived: bool) -> &mut Self {
        self.0.insert("archived", Value::from(archived));

        self
    }

    /// The lock status of the thread.
    pub fn locked(&mut self, lock: bool) -> &mut Self {
        self.0.insert("locked", Value::from(lock));

        self
    }

    /// Whether non-moderators can add other non-moderators to a thread.
    ///
    /// **Note**: Only available on private threads.
    pub fn invitable(&mut self, invitable: bool) -> &mut Self {
        self.0.insert("invitable", Value::from(invitable));

        self
    }

    /// Set the list of applied tags.
    ///
    /// **Note**: Limted to 5; Only available in forum threads.
    pub fn set_applied_tags(&mut self, tags: Vec<ForumTagId>) -> &mut Self {
        let mut applied_tags = Value::from(Vec::<Value>::new());
        let applied_tags_array =
            applied_tags.as_array_mut().expect("applied_tags must be an array");

        for tag in tags {
            applied_tags_array.push(Value::from(tag.to_string()));
        }

        self.0.insert("applied_tags", applied_tags);

        self
    }
}
