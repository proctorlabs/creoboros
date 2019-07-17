use unstructured::Document;

pub enum Message {
    Log { log: Document },
}
