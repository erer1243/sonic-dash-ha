use crate::swss_bridge::{Key, OwnedKey};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    any::Any,
    collections::HashMap,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};
use swss_common::FieldValues;

/// Actor state. A set of input and output tables identified by key.
#[derive(Debug, Default)]
pub struct State {
    input_tables: HashMap<OwnedKey, InputTable>,
    output_tables: HashMap<OwnedKey, OutputTable>,
}

impl State {
    /// Get an input table by key.
    pub fn input<'a, K: Into<Key<'a>>>(&mut self, k: K) -> &mut InputTable {
        self.input_tables.get_mut(&k.into()).unwrap()
    }

    /// Get an output table by key.
    pub fn output<'a, K: Into<Key<'a>>>(&mut self, k: K) -> &mut OutputTable {
        self.output_tables.get_mut(&k.into()).unwrap()
    }

    /// Add an input table.
    pub(crate) fn add_input_table(&mut self, k: OwnedKey, initial_fvs: FieldValues) {
        self.input_tables.insert(k, InputTable::new(initial_fvs));
    }

    /// Add an output table.
    pub(crate) fn add_output_table(&mut self, k: OwnedKey, initial_fvs: FieldValues) {
        self.output_tables.insert(k, OutputTable::new(initial_fvs));
    }

    /// Iterate over all output tables that are dirty (were modified).
    ///
    /// Unsets the dirty flag automatically.
    pub(crate) fn iter_dirty_output_tables(&mut self) -> impl Iterator<Item = &OutputTable> {
        self.output_tables.values_mut().filter(|t| t.dirty).map(|t| {
            t.dirty = false;
            &*t
        })
    }
}

/// A read-only input table.
///
/// An in-memory copy of an swss table that is input to the actor.
#[derive(Debug)]
pub struct InputTable {
    table: CachedTable,
}

impl InputTable {
    fn new(fvs: FieldValues) -> Self {
        Self {
            table: CachedTable::new(fvs),
        }
    }

    /// Get the raw `FieldValues` of this table.
    pub fn fvs(&self) -> &FieldValues {
        self.table.fvs()
    }

    /// Get the field values of this table in a deserialized form, using `swss_serde`.
    pub fn deserialized<T: DeserializeOwned + 'static>(&mut self) -> Result<&T, swss_serde::Error> {
        self.table.deserialized()
    }
}

/// A read-write output/state table.
///
/// An in-memory copy of an swss table that is output or internal state of the actor.
#[derive(Debug)]
pub struct OutputTable {
    table: CachedTable,
    dirty: bool,
}

impl OutputTable {
    fn new(fvs: FieldValues) -> Self {
        Self {
            table: CachedTable::new(fvs),
            dirty: false,
        }
    }

    /// Get the raw `FieldValues` of this table.
    pub fn fvs(&self) -> &FieldValues {
        self.table.fvs()
    }

    /// Get the raw mutable `FieldValues` of this table.
    pub fn fvs_mut(&mut self) -> &mut FieldValues {
        self.dirty = true;
        self.table.invalidate_deserialized_cache();
        self.table.fvs_mut()
    }

    /// Get the field values of this table in a deserialized form, using `swss_serde`.
    pub fn deserialized<T: DeserializeOwned + 'static>(&mut self) -> Result<&T, swss_serde::Error> {
        self.table.deserialized()
    }

    /// Get a mutable the field values of this table in a deserialized form, using `swss_serde`.
    ///
    /// When the returned guard is dropped, the raw `FieldValues` will be updated by re-serializing the `T` back.
    /// A panic will occur if that serialization (using `swss_serde::`) fails.
    pub fn deserialized_mut<T: Serialize + DeserializeOwned + 'static>(
        &mut self,
    ) -> Result<DeserializedMutGuard<'_, T>, swss_serde::Error> {
        // Ensure that the deserialized cache is a T
        self.table.deserialize_into_cache::<T>()?;
        Ok(DeserializedMutGuard {
            table: self,
            t: PhantomData,
        })
    }
}

/// Wrapper around a mutable deserialized `OutputTable` that will re-serialize back to the table's
/// `FieldValues` when it is dropped.
///
/// Drop will panic if `swss_serde` fails to serialize `T` to `FieldValues` in the table.
pub struct DeserializedMutGuard<'a, T: Serialize + 'static> {
    table: &'a mut OutputTable,
    t: PhantomData<&'a mut T>,
}

impl<T: Serialize + 'static> Drop for DeserializedMutGuard<'_, T> {
    /// Panics if swss_serde::to_field_values fails.
    fn drop(&mut self) {
        let t = self.table.table.downcast_deserialized_cache::<T>().unwrap();
        let new_fvs = swss_serde::to_field_values(&t).unwrap();
        *self.table.fvs_mut() = new_fvs;
    }
}

impl<T: Serialize + 'static> Deref for DeserializedMutGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // unwrap: We ensure that the deserialized cache is a T before constructing this guard
        self.table.table.downcast_deserialized_cache().unwrap()
    }
}

impl<T: Serialize + 'static> DerefMut for DeserializedMutGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // unwrap: same as deref
        self.table.table.downcast_deserialized_cache_mut().unwrap()
    }
}

/// A cached, in-memory copy of an swss table.
///
/// Also provides a secondary, deserialization cache so `Actor`s can
/// operate on the field values as a rust struct.
#[derive(Debug)]
struct CachedTable {
    fvs: FieldValues,

    /// Deserializing the fvs is relatively expensive, so we save the result.
    /// This is a secondary caching mechanism, unrelated to how the fvs are cached relative to redis.
    ///
    /// self.fvs is the source of truth, this is a lazy mirror of self.fvs that is invalidated when self.fvs is updated from the database
    deserialized_cache: Option<Box<dyn Any>>,
}

impl CachedTable {
    fn new(fvs: FieldValues) -> Self {
        Self {
            fvs,
            deserialized_cache: None,
        }
    }

    /// When the fvs are updated, the deserialized_cache should be invalidated, because now
    /// the deserialized representation will not match the fvs
    fn invalidate_deserialized_cache(&mut self) {
        self.deserialized_cache = None;
    }

    fn fvs(&self) -> &FieldValues {
        &self.fvs
    }

    fn fvs_mut(&mut self) -> &mut FieldValues {
        &mut self.fvs
    }

    /// Deserialize table's `FieldValues` into a `T` and store it in the deserialized cache.
    ///
    /// This ensures that future calls to downcast methods will return `Some(t)`.
    fn deserialize_into_cache<T: DeserializeOwned + 'static>(&mut self) -> Result<(), swss_serde::Error> {
        if self.downcast_deserialized_cache::<T>().is_none() {
            self.deserialized_cache = Some(Box::new(swss_serde::from_field_values::<T>(&self.fvs)?));
        }

        Ok(())
    }

    /// Attempt to downcast the deserialized cache into `&T`.
    fn downcast_deserialized_cache<T: 'static>(&self) -> Option<&T> {
        self.deserialized_cache.as_ref().and_then(|any| any.downcast_ref())
    }

    /// Attempt to downcast the deserialized cache into `&mut T`.
    fn downcast_deserialized_cache_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.deserialized_cache.as_mut().and_then(|any| any.downcast_mut())
    }

    /// Deserialize the table's `FieldValues` into the cache, then return a downcast `&T`.
    fn deserialized<T: DeserializeOwned + 'static>(&mut self) -> Result<&T, swss_serde::Error> {
        self.deserialize_into_cache::<T>()?;
        Ok(self.downcast_deserialized_cache().unwrap())
    }
}
