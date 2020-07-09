/// The implementation of palisade. This is most likely not useful to you,
/// however the internals are exposed in order for the integration tests to work.

pub mod changelog;
pub mod cmd;
pub mod git;
pub mod version;

pub use cmd::{GitHubAction, CircleCIEnv, Common, Cmd};
