use std::fmt::Display;

/// `CREATE TABLE` clause:
/// ```sql
/// CREATE TABLE records (
///     commit_time String,
///     repo_name String,
///     author_name String,
///     author_email String,
///     operation String,
///     file_path String,
///     commit_id String,
///     commit_message String,
///     content String,
///     calc_time TIMESTAMP TIME INDEX DEFAULT CURRENT_TIMESTAMP,
///     PRIMARY KEY (repo_name, commit_id, file_path, content)
/// );
/// ```
#[derive(Debug)]
pub struct Record {
    pub repo_name: String,
    pub commit_time: String,
    pub author_name: String,
    pub author_email: String,
    pub operation: Operation,
    pub file_path: Option<String>,
    pub commit_id: String,
    pub commit_messaage: String,
    /// Todo content
    pub content: String,
    /// Time this record is calculated
    pub calc_time: String,
}

#[derive(Debug)]
pub enum Operation {
    Add,
    Remove,
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add => f.write_str("add"),
            Operation::Remove => f.write_str("remove"),
        }
    }
}

#[derive(Debug)]
pub struct RecordBuilder {
    pub repo_name: String,
    pub commit_time: String,
    pub author_name: String,
    pub author_email: String,
    pub commit_id: String,
    pub commit_message: String,
}

impl RecordBuilder {
    pub fn new_base(
        repo_name: String,
        commit_time: String,
        author_name: String,
        author_email: String,
        commit_id: String,
        commit_message: String,
    ) -> Self {
        Self {
            repo_name,
            commit_time,
            author_name,
            author_email,
            commit_id,
            commit_message,
        }
    }

    pub fn build(
        &self,
        operation: Operation,
        file_path: Option<String>,
        content: String,
    ) -> Record {
        Record {
            repo_name: self.repo_name.clone(),
            commit_time: self.commit_time.clone(),
            author_name: self.author_name.clone(),
            author_email: self.author_email.clone(),
            operation,
            file_path,
            commit_id: self.commit_id.clone(),
            commit_messaage: self.commit_message.clone(),
            content,
            calc_time: String::new(),
        }
    }
}
