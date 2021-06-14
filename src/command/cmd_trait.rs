use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;

pub trait Command: Send + CommandClone {
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        id_client: usize,
    ) -> Result<String, RunError>;
}

pub trait CommandClone {
    fn clone_box(&self) -> Box<dyn Command>;
}

impl<T> CommandClone for T
where
    T: 'static + Command + Clone,
{
    fn clone_box(&self) -> Box<dyn Command> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Command> {
    fn clone(&self) -> Box<dyn Command> {
        self.clone_box()
    }
}
