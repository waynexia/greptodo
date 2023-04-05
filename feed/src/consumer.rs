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
