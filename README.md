# Permeable

Permeable is a permission-demand trait. It helps to decouple the permission-demander from the permission / auth provider.

### Example
```rust
use permeable::{Permeable, PermissionError};

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
enum Permissions {
    RoThis,
    RwThis,
    AccessThat,
}

struct Perm;

impl Permeable<&str, Permissions> for Perm {
    fn has_perm(&self, user: &str, permission: &Permissions) -> Result<(), PermissionError> {
        use Permissions::*;
        match (user, permission) {
            ("admin", _) => Ok(()),
            ("peter", &AccessThat) => Ok(()),
            ("peter", &RoThis) => Ok(()),
            ("paul", &RoThis) => Ok(()),
            ("paul", &RwThis) => Ok(()),
            _ => Err(PermissionError::denied(format!("{permission:?}"), "moduleName")),
        }
    }
}

use Permissions::*;

fn main() {
    let perm = Perm;
    assert_eq!(Ok(()), perm.has_perm("admin", &RwThis));
    assert_eq!(Ok(()), perm.has_perm("peter", &AccessThat));
    assert!(perm.has_perm("peter", &RwThis).is_err());
}
```
