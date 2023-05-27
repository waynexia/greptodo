use std::sync::{Arc, Mutex};

use crate::schema::Record;

pub trait Consumer {
    fn record(&self, record: Record);
}

pub struct PrintConsumer {}

impl Consumer for PrintConsumer {
    fn record(&self, record: Record) {
        println!("{:?}", record);
    }
}

#[derive(Default)]
pub struct DatabaseConsumer {
    rows: Arc<Mutex<Vec<String>>>,
}

impl Consumer for DatabaseConsumer {
    fn record(&self, record: Record) {
        let new_row = Self::format_record(record);
        let mut rows = self.rows.lock().unwrap();
        rows.push(new_row);
    }
}

impl DatabaseConsumer {
    pub fn new() -> Self {
        Self {
            rows: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Format a [Record] to one line of insert data
    ///
    /// The last field `calc_time` is filled by default value (current timestamp)
    fn format_record(record: Record) -> String {
        format!(
            "(\'{}\',\'{}\',\'{}\',\'{}\',\'{}\',\'{}\',\'{}\',\'{}\',\'{}\')",
            record.repo_name,
            record.commit_time,
            record.author_name,
            record.author_email,
            record.operation,
            record.file_path.unwrap_or_else(|| String::new()),
            record.commit_id,
            record.commit_messaage.replace('\'', "_").replace('\"', "_"),
            record.content.replace('\'', "_").replace('\"', "_"),
        )
    }

    pub fn into_insert(self) -> String {
        let rows = self.rows.lock().unwrap();
        let mut insert = String::from("INSERT INTO `records` (`repo_name`, `commit_time`, `author_name`, `author_email`, `operation`, `file_path`, `commit_id`, `commit_message`, `content`) VALUES ");
        insert.push_str(&rows.join(","));
        insert.push(';');
        insert
    }
}
