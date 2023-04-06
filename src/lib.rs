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
/// const RoThis : &str = "RoThis";
/// const RwThis : &str = "RwThis";
/// const AccessThat : &str = "AccessThat";
///
/// struct User {
///     name: String,
/// };
///
/// // A naive implementation of a the Permeable-Trait.
/// impl Permeable for User {
///
///     fn has_perm(&self, permission: &str) -> Result<(), PermissionError>{
///         match (self.name.as_str(), permission) {
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
/// let admin = User { name: String::from("admin") };
/// assert_eq!(admin.has_perm(RwThis), Ok(()));
/// let peter = User { name: String::from("peter") };
/// assert_eq!(peter.has_perm(AccessThat), Ok(()));
/// assert_eq!(peter.has_perm(RwThis).is_err(), true);
/// ```
pub trait Permeable {
    /// Returns `Result::Ok(())` if the given `user` has the given `permission`.
    #[allow(unused)]
    fn has_perm(&self, permission: &str) -> Result<(), PermissionError>;
}
