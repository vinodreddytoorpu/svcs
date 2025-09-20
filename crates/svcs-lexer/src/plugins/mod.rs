//! Plugin implementations for different token categories

pub mod module_plugin;
pub mod interface_plugin;
pub mod datatype_plugin;
pub mod control_plugin;
pub mod operator_plugin;
pub mod general_plugin;

pub use module_plugin::*;
pub use interface_plugin::*;
pub use datatype_plugin::*;
pub use control_plugin::*;
pub use operator_plugin::*;
pub use general_plugin::*;
