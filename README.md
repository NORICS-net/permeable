# Permeable

Permeable is a permission-demand trait. It helps to decouple the permission-demander from the permission / auth provider.

Our goal was the simplest possible trait to ask for permission. Can be implemented by a permission-provider
distinguishing between users.

### Example

```rust
use permeable::{Permeable, PermissionError};

const RO_THIS: &str = "RoThis";
const RW_THIS: &str = "RwThis";
const ACCESS_THAT: &str = "AccessThat";

struct User {
    name: String,
};

// A naive implementation of a the Permeable-Trait.
impl Permeable for User {
    fn has_perm(&self, permission: &str) -> Result<(), PermissionError> {
        match (self.name.as_str(), permission) {
            ("admin", _) => Ok(()),
            ("peter", ACCESS_THAT) => Ok(()),
            ("peter", RO_THIS) => Ok(()),
            ("paul", RO_THIS) => Ok(()),
            ("paul", RW_THIS) => Ok(()),
            // catch all
            (_, perm) => Err(PermissionError::denied(permission, &self.name)),
        }
    }
}

fn main() {
    let admin = User { name: String::from("admin") };
    assert_eq!(admin.has_perm(RW_THIS), Ok(()));
    let peter = User { name: String::from("peter") };
    assert_eq!(peter.has_perm(ACCESS_THAT), Ok(()));
    assert_eq!(peter.has_perm(RW_THIS).is_err(), true);
}
```
