mod composite;
mod local;
mod global;
mod exception;
mod context;

pub use local::{LocalError, LocalResult};
pub use global::{GlobalError, GlobalResult};

pub use exception::ExceptionConvertible;

pub use context::CompositeErrorContext;
pub use context::JavaErrorContext;

pub use composite::{CompositeError, CompositeErrorInclude};