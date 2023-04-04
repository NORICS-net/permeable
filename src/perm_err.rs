use crate::perm_err::PermissionError::*;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum PermissionError {
    /// Can inform which `permission` is denied for `user`.
    PermissionDenied { permission: String, user: String },

    /// Can be used to inform about a not-known Permission (e.g. typos).
    UnknownPermission { permission: String, user: String },
}

impl Display for PermissionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PermissionDenied { permission, user } => {
                write!(f, "Permission: '{permission}' not set for {user}.")
            }
            UnknownPermission { permission, user } => {
                write!(f, "Unknown Permission: '{permission}' asked for {user}.")
            }
        }
    }
}

impl PermissionError {
    pub fn denied(permission: impl Display, user: impl Display) -> Self {
        PermissionDenied {
            permission: permission.to_string(),
            user: user.to_string(),
        }
    }

    pub fn unknown(permission: impl Display, user: impl Display) -> Self {
        UnknownPermission {
            permission: permission.to_string(),
            user: user.to_string(),
        }
    }
}
