use unstructured::Document;

#[derive(Debug)]
pub enum Message {
    Log { log: Document },
    Unit,
}
