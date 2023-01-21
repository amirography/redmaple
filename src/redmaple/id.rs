use std::fmt::Debug;

use uuid::Uuid;

/// The Implementation of the ID that the crate uses
///
/// # Example
///
/// ```rust
/// use redmaple::redmaple::id::ID;
///
/// let new_id = ID::new();
///
/// assert_eq!(4usize, new_id.uuid().get_version_num())
///
/// ```

#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct ID(Uuid);

impl ID {
    /// creats a new instance of the ID
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Returns the uuid of this [`ID`].
    #[must_use]
    pub const fn uuid(&self) -> Uuid {
        self.0
    }
}
