// (c) 2023 - NORICS GmbH - coding@migmedia.de
//! This crate provides generic trait to be implemented by a permission-provider.
pub use perm_err::PermissionError;

mod perm_err;
///
/// Trait to be implemented by a permission-provider.
///  *  `User` - in the simplest version a `&str`.
///  *  `Perm` - identifier of a permission, eg. `enum` or `String`.
///
/// # Example
/// ```
/// use permeable::{Permeable, PermissionError};
///
/// #[allow(dead_code)]
/// #[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// enum Permission {
///     RoThis,
///     RwThis,
///     AccessThat,
/// }
///
/// struct User {
///     name: String,
/// };
///
/// // A naive implementation of a the Permeable-Trait.
/// impl Permeable for User {
///     type Perm = Permission;
///     fn has_perm(&self, permission: impl Into<Permission>) -> Result<(), PermissionError> {
///         use Permission::*;
///         match (self.name.as_str(), permission.into()) {
///             ("admin", _) => Ok(()),
///             ("peter", AccessThat) => Ok(()),
///             ("peter", RoThis) => Ok(()),
///             ("paul", RoThis) => Ok(()),
///             ("paul", RwThis) => Ok(()),
///             // catch all
///             (_, perm) => Err(PermissionError::denied(format!("{perm:?}"), &self.name)),
///         }
///     }
/// }
///
/// use Permission::*;
/// let admin = User { name: String::from("admin") };
/// assert_eq!(admin.has_perm(RwThis), Ok(()));
/// let peter = User { name: String::from("peter") };
/// assert_eq!(peter.has_perm(AccessThat), Ok(()));
/// assert_eq!(peter.has_perm(RwThis).is_err(), true);
/// ```
pub trait Permeable: Sync + Send {
    type Perm;
    /// Returns `Result::Ok(())` if the given `user` has the given `permission`.
    #[allow(unused)]
    fn has_perm(&self, permission: impl Into<Self::Perm>) -> Result<(), PermissionError>;
}
