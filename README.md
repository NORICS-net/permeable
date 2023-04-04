# Permeable

Permeable is a permission-demand trait. It helps to decouple the permission-demander from the permission / auth provider.

### Example
```rust
use permeable::{Permeable, PermissionError};

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Permission {
    RoThis,
    RwThis,
    AccessThat,
}

struct User {
    name: String
}

// A naive implementation of a the Permeable-Trait.
impl Permeable for User {
    type Perm = Permission;
    fn has_perm(&self, permission: impl Into<Permission>) -> Result<(), PermissionError> {
        use Permission::*;
        match (self.name.as_str(), permission.into()) {
            ("admin", _) => Ok(()),
            ("peter", AccessThat) => Ok(()),
            ("peter", RoThis) => Ok(()),
            ("paul", RoThis) => Ok(()),
            ("paul", RwThis) => Ok(()),
            // catch all
            (_, perm) => Err(PermissionError::denied(format!("{perm:?}"), &self.name)),
        }
    }
}

use Permission::*;

fn main() {
    let admin = User { name: String::from("admin") };
    assert_eq!(admin.has_perm(RwThis), Ok(()));
    let peter = User { name: String::from("peter") };
    assert_eq!(peter.has_perm(AccessThat), Ok(()));
    assert_eq!(peter.has_perm(RwThis).is_err(), true);
}
```
