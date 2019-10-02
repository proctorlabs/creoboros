use super::*;

impl RunnableAgent for Arc<Timer> {
    fn execute(&self) -> Result<()> {
        println!("beep");
        capture!(self:slf {
            async_std::task::spawn(async move {
                async_std::task::sleep(slf.interval).await;
                info!("Timer event occurred" agent: slf.name => slf.logger);
                slf.execute()
            });
        });
        Ok(())
    }
}
