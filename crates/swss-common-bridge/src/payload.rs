use crate::{table_info::TableInfo, table_watcher::TableWatcherMessage};
use std::error::Error;
use swbus_actor::prelude::ServicePath;
use swss_common::KeyOpFieldValues;

pub fn encode_subscribe(table_info: TableInfo, subscriber: ServicePath) -> Vec<u8> {
    serde_json::to_vec(&TableWatcherMessage::Subscribe { subscriber, table_info }).unwrap()
}

pub(crate) fn decode_table_watcher_message(payload: &[u8]) -> serde_json::Result<TableWatcherMessage> {
    serde_json::from_slice(payload)
}

pub(crate) fn encode_kfvs(ti: &TableInfo, kfvs: &KeyOpFieldValues) -> Vec<u8> {
    serde_json::to_vec(&(ti, kfvs)).unwrap()
}

pub fn decode_kfvs(payload: &[u8]) -> Result<(TableInfo, KeyOpFieldValues), Box<dyn Error>> {
    Ok(serde_json::from_slice(payload)?)
}
