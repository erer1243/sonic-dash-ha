#![allow(unused)]
mod swss_bridge;

use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use serde::{de::DeserializeOwned, Deserialize};
use std::{
    any::Any,
    borrow::{Borrow, Cow},
    collections::HashMap,
    future::Future,
};
use swbus_edge::swbus_proto::swbus::{ServicePath, SwbusErrorCode};
use swss_bridge::Key;
use swss_common::FieldValues;

pub trait Actor {
    fn handle_request(
        &mut self,
        state: &mut State,
        outbox: &mut Outbox,
        source: ServicePath,
        payload: Vec<u8>,
    ) -> impl Future<Output = Result<(), (SwbusErrorCode, String)>> + Send + Sync;

    fn handle_table_update(
        &mut self,
        state: &mut State,
        outbox: &mut Outbox,
    ) -> impl Future<Output = Result<(), (SwbusErrorCode, String)>> + Send + Sync;
}

pub struct Outbox {
    outgoing_messages: Vec<()>,
}

// Drives a single actor
pub struct ActorDriver {
    state: State,
}

#[derive(Debug)]
pub struct State {
    input_tables: HashMap<Key<'static>, InputTable>,
    output_tables: HashMap<Key<'static>, OutputTable>,
}

impl State {}

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

    fn fvs(&self) -> &FieldValues {
        self.table.fvs()
    }

    // fn deserialized(&mut self)
}

#[derive(Debug)]
pub struct OutputTable {
    table: CachedTable,
    // Whether this table was modified and needs to be flushed to the db,
    // and, if so, which part was modified. If both were modified, we panic.
    // modified_fvs: bool,
    // modified_deser: bool,
}

impl OutputTable {
    fn new(fvs: FieldValues) -> Self {
        Self {
            table: CachedTable::new(fvs),
            // modified_fvs: false,
            // modified_deser: false,
        }
    }

    pub fn fvs_mut(&mut self) -> &mut FieldValues {
        // self.modified = true;
        self.table.fvs_mut()
    }

    pub fn deserialized<T: DeserializeOwned + 'static>(&mut self) -> Result<&T, swss_serde::Error> {
        self.table.deserialized()
    }

    pub fn deserialized_mut<T: DeserializeOwned + 'static>(&mut self) -> Result<&mut T, swss_serde::Error> {
        // self.modified = true;
        self.table.deserialized_mut()
    }
}

/// An input or output table
#[derive(Debug)]
struct CachedTable {
    fvs: FieldValues,

    // Deserializing the fvs is relatively expensive, so we save the result.
    // This is a secondary caching mechanism, unrelated to how the fvs are cached relative to redis.
    deserialized_cache: RwLock<Option<Box<dyn Any>>>,
}

impl CachedTable {
    fn new(fvs: FieldValues) -> Self {
        Self {
            fvs,
            deserialized_cache: RwLock::new(None),
        }
    }

    fn update(&mut self, fvs: FieldValues, invalidate_deser_cache: bool) {
        self.fvs = fvs;
        if invalidate_deser_cache {
            *self.deserialized_cache.get_mut() = None;
        }
    }

    fn fvs(&self) -> &FieldValues {
        &self.fvs
    }

    fn fvs_mut(&mut self) -> &mut FieldValues {
        &mut self.fvs
    }

    // Helper for get_deserialized_mut
    fn downcast_cache_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.deserialized_cache
            .as_mut()
            .map(|any| any.downcast_mut::<T>())
            .flatten()
    }

    fn deserialized_mut<T: DeserializeOwned + 'static>(&mut self) -> Result<&mut T, swss_serde::Error> {
        // This unwrap and the repeated work in downcasting twice are a polonious hack:
        // https://github.com/rust-lang/rfcs/blob/master/text/2094-nll.md#problem-case-3-conditional-control-flow-across-functions
        if self.downcast_cache_mut::<T>().is_none() {
            self.deserialized_cache = Some(Box::new(swss_serde::from_field_values::<T>(&self.fvs)?));
        }
        Ok(self.downcast_cache_mut().unwrap())
    }

    fn deserialized<T: DeserializeOwned + 'static>(&mut self) -> Result<&T, swss_serde::Error> {
        self.deserialized_mut().map(|r| &*r)
    }
}
