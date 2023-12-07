// (c) 2023 - NORICS GmbH - coding@migmedia.de
//! This crate provides generic trait to be implemented by a permission-provider.
pub use perm_err::PermissionError;

mod perm_err;
///
/// Simplest possible Trait to ask for permission. Can be implemented by a permission-provider
/// distinguishing between users.
///
/// # Example
/// ```rust
/// use permeable::{Permeable, PermissionError};
///
/// const RO_THIS : &str = "RoThis";
/// const RW_THIS : &str = "RwThis";
/// const ACCESS_THAT : &str = "AccessThat";
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
///             ("peter", ACCESS_THAT) => Ok(()),
///             ("peter", RO_THIS) => Ok(()),
///             ("paul", RO_THIS) => Ok(()),
///             ("paul", RW_THIS) => Ok(()),
///             // catch all
///             (_, perm) => Err(PermissionError::denied(permission, &self.name)),
///         }
///     }
/// }
///
/// let admin = User { name: String::from("admin") };
/// assert_eq!(admin.has_perm(RW_THIS), Ok(()));
/// let peter = User { name: String::from("peter") };
/// assert_eq!(peter.has_perm(ACCESS_THAT), Ok(()));
/// assert_eq!(peter.has_perm(RW_THIS).is_err(), true);
/// ```
pub trait Permeable {
    /// Returns `Result::Ok(())` if the given `permission` is granted.
    ///
    /// # Errors
    /// if the `permission` is not granted or unknown it returns a
    /// [`PermissionError`](./enum.PermissionError.html).
    #[allow(unused)]
    fn has_perm(&self, permission: &str) -> Result<(), PermissionError>;
}

/// A everything-allow `Permeable`. For testing-purposes.
pub struct AllowAllPermission();

impl Permeable for AllowAllPermission {
    fn has_perm(&self, _permission: &str) -> Result<(), PermissionError> {
        Ok(())
    }
}

/// A everything-deny `Permeable`. For testing-purposes.
pub struct DenyAllPermission();

impl Permeable for DenyAllPermission {
    fn has_perm(&self, permission: &str) -> Result<(), PermissionError> {
        Err(PermissionError::denied(permission, "DenyAllPermission"))
    }
}
