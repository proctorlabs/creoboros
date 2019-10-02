use super::*;

impl RunnableAgent for Arc<Timer> {
    fn execute(&self) -> Result<()> {
        capture!(self:slf {
            task::spawn(async move {
                task::sleep(slf.interval).await;
                info!("Timer event occurred" agent: slf.name => slf.logger);
                slf.execute()
            });
        });
        Ok(())
    }
}
