#[derive(Debug)]
pub struct Record {
    pub commit_time: String,
    pub author_name: String,
    pub author_email: String,
    pub operation: Operation,
    pub file_path: String,
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

#[derive(Debug)]
pub struct RecordBuilder {
    pub commit_time: String,
    pub author_name: String,
    pub author_email: String,
    pub commit_message: String,
}

impl RecordBuilder {
    pub fn new_base(
        commit_time: String,
        author_name: String,
        author_email: String,
        commit_message: String,
    ) -> Self {
        Self {
            commit_time,
            author_name,
            author_email,
            commit_message,
        }
    }

    pub fn build(&self, operation: Operation, file_path: String, content: String) -> Record {
        Record {
            commit_time: self.commit_time.clone(),
            author_name: self.author_name.clone(),
            author_email: self.author_email.clone(),
            operation,
            file_path,
            commit_messaage: self.commit_message.clone(),
            content,
            calc_time: String::new(),
        }
    }
}
