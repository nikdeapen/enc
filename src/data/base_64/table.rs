use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

/// A base-64 coding table.
///
/// This implementation supports static and atomic reference counted tables. This strategy avoids
/// all allocation for the standard table configurations and also avoids allocation when cloning
/// reference counted tables. The only allocation needed is when creating a new, non-standard
/// table.
#[derive(Clone, Debug)]
pub enum Table<const N: usize> {
    /// A static table.
    Static(&'static [u8; N]),

    /// An atomic reference counted table.
    Reference(Arc<[u8; N]>),
}

impl<const N: usize> PartialEq for Table<N> {
    fn eq(&self, other: &Self) -> bool {
        self.table() == other.table()
    }
}

impl<const N: usize> Eq for Table<N> {}

impl<const N: usize> Hash for Table<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.table().hash(state)
    }
}

impl<const N: usize> Ord for Table<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.table().cmp(other.table())
    }
}

impl<const N: usize> PartialOrd for Table<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const N: usize> Table<N> {
    //! Properties

    /// Gets the raw table.
    pub fn table(&self) -> &[u8; N] {
        match self {
            Self::Static(table) => table,
            Self::Reference(table) => table.as_ref(),
        }
    }
}
