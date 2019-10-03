macro_rules! info {
    ($($tt:tt)*) => {
        log!(INFO: $( $tt )*)
    };
}

macro_rules! warn {
    ($($tt:tt)*) => {
        log!(WARN: $( $tt )*)
    };
}

macro_rules! log {
    ($level:ident : $l:literal $( $key:ident : $val:expr ),* ) => {
        log!($level : $l [] $( $key : $val ),* => "default" )
    };
    ($level:ident : $l:literal $( $key:ident : $val:expr ),* => $( $logger:tt )* ) => {
        log!($level : $l [] $( $key : $val ),* => $( $logger )* )
    };
    ($level:ident : $l:literal [ $($args:tt)* ] $( $key:ident : $val:expr ),* ) => {
        log!($level : $l [ $( $args )* ] $( $key : $val ),* => "default" )
    };
    ($level:ident : $l:literal [ $($args:tt)* ] $( $key:ident : $val:expr ),* => $( $logger:tt )* ) => {
        {
            let log_log = format!($l, $( $args )* );
            $( let $key = $val.to_string(); )*
            let logger_name = $( $logger )* .to_owned();
            task::spawn(async move {
                    let mut log: std::collections::BTreeMap<unstructured::Document, unstructured::Document> = std::collections::BTreeMap::new();
                    log.insert("timestamp".into(), chrono::Local::now().to_rfc3339().into());
                    log.insert("log".into(), log_log.into());
                    log.insert("level".into(), stringify!($level).into());
                    $( log.insert(stringify!($key).into(), $key.into()); )*
                    crate::runtime::CERBERUS.send(&logger_name, crate::runtime::Message::Log { log: log.into() });
                Ok::<(), crate::error::AppError>(())
            });
        }
    };
}
