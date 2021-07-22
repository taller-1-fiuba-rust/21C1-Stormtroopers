//! Common behavior that all commands must have.
use crate::errors::run_error::RunError;
use crate::server::app_info::AppInfo;

/// All commands must implement this trait. This works as a sort of interface for standarize the commands.
pub trait Command: Send + CommandClone {
    /// This is the shape of the functions that a command needs to perform it's logic.
    /// The three required parameters are:
    ///
    /// args: The user input minus the command i.e. << key "value1 value2" >>
    ///
    /// app_info: The facade of the app.
    ///
    /// id_client: The id of the thread in charge.
    fn run(
        &self,
        args: Vec<&str>,
        app_info: &AppInfo,
        id_client: usize,
    ) -> Result<String, RunError>;
}

/// Since we pass the commands around as Box<dyn Command>, this trait is a workaround to be able to implement Clone for the commands.
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
