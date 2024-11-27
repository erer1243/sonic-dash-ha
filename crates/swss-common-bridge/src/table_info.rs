use serde::{Deserialize, Serialize};
use swss_common::DbConnectionInfo;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum TableInfo {
    ConsumerStateTable(RedisTableInfo),
    SubscriberStateTable(RedisTableInfo),
    ZmqConsumerStateTable(ZmqTableInfo),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct RedisTableInfo {
    pub db_connection_info: DbConnectionInfo,
    pub db_id: i32,
    pub table_name: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ZmqTableInfo {
    pub db_connection_info: DbConnectionInfo,
    pub db_id: i32,
    pub table_name: String,
    pub zmq_endpoint: String,
}
