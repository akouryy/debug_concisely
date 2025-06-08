pub use debug_concisely_derive::*;

pub struct DebugConciselyProxy(pub String);

impl std::fmt::Debug for DebugConciselyProxy {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}
