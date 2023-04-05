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
