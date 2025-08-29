use crate::plugin::plugin_ctx::ctx::Proposal;

pub struct Plan {
    pub proposals: Vec<Proposal>,
}

impl Plan {
    pub fn new(proposals: Vec<Proposal>) -> Self {
        Self { proposals }
    }

    pub fn validate() {}

    pub fn apply_to_tmp() {}
}
