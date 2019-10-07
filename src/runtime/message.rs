use unstructured::Document;

#[derive(Debug, From)]
pub enum Message {
    Unit,
    Log { log: Document },
}
