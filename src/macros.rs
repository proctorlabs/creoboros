macro_rules! spawn {
    ($to_spawn:expr) => {{
        let spawnable: Spawnable = Box::new($to_spawn.into_future().map(|_| ()).map_err(|_| ()));
        crate::runtime::BOOMSLANG.spawn(spawnable)
    }};
}