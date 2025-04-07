mod startup;
mod controlpanel;
mod controllertelem;

pub use startup::StartupPage;
pub use controllertelem::ControllerTelem;
pub use controlpanel::*;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Debug, Display, Clone, Copy, Default, PartialEq, EnumIter, EnumString, Eq)]
pub enum Page {
    #[default]
    Startup,
    ControllerTelem,
}
