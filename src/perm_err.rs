use crate::perm_err::PermissionError::*;
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub enum PermissionError {
    /// Can inform which permission in which module is denied.
    PermissionDenied {
        permission: String,
        module: String,
    },
    UnknownPermission {
        permission: String,
        module: String,
    },
}

impl Display for PermissionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PermissionDenied { permission, module } => {
                write!(f, "Permission: '{permission}' needed by {module}.")
            }
            UnknownPermission { permission, module } => {
                write!(f, "Unknown Permission: '{permission}' asked by {module}.")
            }
        }
    }
}

impl PermissionError {
    pub fn denied(permission: impl Display, module: impl Into<String>) -> Self {
        PermissionDenied {
            permission: permission.to_string(),
            module: module.into(),
        }
    }

    pub fn unknown(permission: impl Display, module: impl Into<String>) -> Self {
        UnknownPermission {
            permission: permission.to_string(),
            module: module.into(),
        }
    }
}
