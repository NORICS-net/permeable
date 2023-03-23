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
/// #[derive(Debug, Eq, PartialEq)]
/// enum Permissions {
///     RoThis,
///     RwThis,
///     AccessThat,
/// }
///
/// struct Perm;
///
/// impl Permeable<&str, Permissions> for Perm {
///     fn has_perm(&self, user: &str, permission: &Permissions) -> Result<(), PermissionError> {
///         use Permissions::*;
///         match (user, permission) {
///             ("admin", _) => Ok(()),
///             ("peter", &AccessThat) => Ok(()),
///             ("peter", &RoThis) => Ok(()),
///             ("paul", &RoThis) => Ok(()),
///             ("paul", &RwThis) => Ok(()),
///             _ => Err(PermissionError::denied(format!("{permission:?}"), "moduleName")),
///         }
///     }
/// }
///
/// use Permissions::*;
/// let perm = Perm;
/// assert_eq!(Ok(()), perm.has_perm("admin", &RwThis));
/// assert_eq!(Ok(()), perm.has_perm("peter", &AccessThat));
/// assert!(perm.has_perm("peter", &RwThis).is_err());
///
/// ```
pub trait Permeable<User, Perm>: Sync + Send {
    /// Returns `Result::Ok(())` if the given `user` has the given `permission`.
    #[allow(unused)]
    fn has_perm(&self, user: User, permission: &Perm) -> Result<(), PermissionError> {
        Ok(())
    }
}
