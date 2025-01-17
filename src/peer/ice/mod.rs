use serde::{Deserialize, Serialize};

pub mod ice_candidate;
pub mod ice_connection_state;
pub mod ice_credential_type;
pub mod ice_gather;
pub mod ice_protocol;
pub mod ice_role;
pub mod ice_server;

/// ICEParameters includes the ICE username fragment
/// and password and other ICE-related parameters.
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ICEParameters {
    pub username_fragment: String,
    pub password: String,
    pub ice_lite: bool,
}
