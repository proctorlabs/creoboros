macro_rules! spawn {
    ($to_spawn:expr) => {{
        let spawnable: Spawnable = Box::new($to_spawn.into_future().map(|_| ()).map_err(|_| ()));
        crate::runtime::BOOMSLANG.spawn(spawnable)
    }};
}

macro_rules! log_event {
    ($agent:expr, $type:expr => $log:expr) => {{
        let mut log = std::collections::BTreeMap::new();
        log.insert("agent".into(), $agent.into());
        log.insert("event".into(), $type.into());
        log.insert("timestamp".into(), chrono::Local::now().to_rfc3339().into());
        log.insert("log".into(), $log.into());
        crate::runtime::BOOMSLANG
            .sender()
            .try_send(crate::runtime::Message::Log { log: log.into() })
            .unwrap_or_default();
    }};
}
