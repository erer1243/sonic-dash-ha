use std::{
    borrow::{Borrow, Cow},
    ops::Deref,
    sync::Arc,
};

/// Identifier of a single entry in an swss table.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display)]
#[display("{db}:{table}:{key}")]
pub struct Key<'a> {
    db: Cow<'a, str>,
    table: Cow<'a, str>,
    key: Cow<'a, str>,
}

impl<'a> Key<'a> {
    pub fn new(db: &'a str, table: &'a str, key: &'a str) -> Self {
        Self {
            db: Cow::Borrowed(db),
            table: Cow::Borrowed(table),
            key: Cow::Borrowed(key),
        }
    }

    pub fn new_owned(db: String, table: String, key: String) -> Key<'static> {
        Key {
            db: Cow::Owned(db),
            table: Cow::Owned(table),
            key: Cow::Owned(key),
        }
    }
    pub fn as_ref(&self) -> Key<'_> {
        Key {
            db: Cow::Borrowed(&self.db),
            table: Cow::Borrowed(&self.table),
            key: Cow::Borrowed(&self.key),
        }
    }

    pub fn into_owned_key(self) -> OwnedKey {
        OwnedKey::new(self.db.into_owned(), self.table.into_owned(), self.key.into_owned())
    }

    pub fn db(&self) -> &str {
        &self.db
    }

    pub fn table(&self) -> &str {
        &self.table
    }

    pub fn key(&self) -> &str {
        &self.key
    }
}

impl<'a> From<(&'a str, &'a str, &'a str)> for Key<'a> {
    fn from(tup: (&'a str, &'a str, &'a str)) -> Self {
        Self::from(&tup)
    }
}

impl<'a, 'b> From<&'b (&'a str, &'a str, &'a str)> for Key<'a> {
    fn from((db, table, key): &'b (&'a str, &'a str, &'a str)) -> Self {
        Self::new(db, table, key)
    }
}

/// Always owned, sharable (via Arc) `Key`.
///
/// OwnedKey is necessary for covariance over Borrowing for use as a hashmap key.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Deref, derive_more::Display)]
#[display("{_0}")]
pub struct OwnedKey(Arc<Key<'static>>);

impl OwnedKey {
    pub fn new(db: String, table: String, key: String) -> Self {
        OwnedKey(Arc::new(Key::new_owned(db, table, key)))
    }
}

impl<'a> Borrow<Key<'a>> for OwnedKey {
    fn borrow(&self) -> &Key<'a> {
        &self.0
    }
}

/// Identifier of a single swss table.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Display)]
#[display("{db}:{table}")]
pub(crate) struct TableId<'a> {
    db: Cow<'a, str>,
    table: Cow<'a, str>,
}

impl<'a> TableId<'a> {
    pub(crate) fn new(db: String, table: String) -> Self {
        Self {
            db: Cow::Owned(db),
            table: Cow::Owned(table),
        }
    }

    pub(crate) fn of_key(key: &'a Key<'a>) -> Self {
        Self {
            db: Cow::Borrowed(&key.db),
            table: Cow::Borrowed(&key.table),
        }
    }
}

/// Always owned `TableId`.
///
/// Same as `OwnedKey` - covariance hack for use as hashmap key
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, derive_more::Deref, derive_more::Display)]
#[display("{_0}")]
pub(crate) struct OwnedTableId(Arc<TableId<'static>>);

impl OwnedTableId {
    pub fn new(db: String, table: String) -> Self {
        OwnedTableId(Arc::new(TableId::new(db, table)))
    }
}

impl<'a> Borrow<TableId<'a>> for OwnedTableId {
    fn borrow(&self) -> &TableId<'a> {
        &self.0
    }
}
