use super::*;
use crate::*;
use std::collections::HashMap;

/// Rust wrapper around `swss::DBConnector`.
#[derive(Debug)]
pub struct DbConnector {
    pub(crate) ptr: SWSSDBConnector,
    connection: DbConnectionInfo,
}

/// Details about how a DbConnector is connected to Redis
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DbConnectionInfo {
    Tcp { hostname: String, port: u16, db_id: i32 },
    Unix { sock_path: String, db_id: i32 },
    Named { db_name: String, is_tcp_conn: bool },
}

impl DbConnector {
    /// Create a new DbConnector from [`DbConnectionInfo`].
    ///
    /// Timeout of 0 means block indefinitely.
    pub fn new(connection: DbConnectionInfo, timeout_ms: u32) -> DbConnector {
        let ptr = match &connection {
            DbConnectionInfo::Tcp { hostname, port, db_id } => {
                let hostname = cstr(hostname);
                unsafe { SWSSDBConnector_new_tcp(*db_id, hostname.as_ptr(), *port, timeout_ms) }
            }
            DbConnectionInfo::Unix { sock_path, db_id } => {
                let sock_path = cstr(sock_path);
                unsafe { SWSSDBConnector_new_unix(*db_id, sock_path.as_ptr(), timeout_ms) }
            }
            DbConnectionInfo::Named { db_name, is_tcp_conn } => {
                let db_name = cstr(db_name);
                unsafe { SWSSDBConnector_new_named(db_name.as_ptr(), timeout_ms, *is_tcp_conn as u8) }
            }
        };

        Self { ptr, connection }
    }

    /// Create a DbConnector from a named entry in the SONiC db config.
    ///
    /// Timeout of 0 means block indefinitely.
    pub fn new_named(db_name: impl Into<String>, is_tcp_conn: bool, timeout_ms: u32) -> DbConnector {
        let db_name = db_name.into();
        Self::new(DbConnectionInfo::Named { db_name, is_tcp_conn }, timeout_ms)
    }

    /// Create a DbConnector over a tcp socket.
    ///
    /// Timeout of 0 means block indefinitely.
    pub fn new_tcp(db_id: i32, hostname: impl Into<String>, port: u16, timeout_ms: u32) -> DbConnector {
        let hostname = hostname.into();
        Self::new(DbConnectionInfo::Tcp { hostname, port, db_id }, timeout_ms)
    }

    /// Create a DbConnector over a unix socket.
    ///
    /// Timeout of 0 means block indefinitely.
    pub fn new_unix(db_id: i32, sock_path: impl Into<String>, timeout_ms: u32) -> DbConnector {
        let sock_path = sock_path.into();
        Self::new(DbConnectionInfo::Unix { sock_path, db_id }, timeout_ms)
    }

    /// Clone a DbConnector with a timeout.
    ///
    /// Timeout of 0 means block indefinitely.
    pub fn clone_timeout(&self, timeout_ms: u32) -> Self {
        Self::new(self.connection.clone(), timeout_ms)
    }

    pub fn connection(&self) -> &DbConnectionInfo {
        &self.connection
    }

    pub fn del(&self, key: &str) -> bool {
        let key = cstr(key);
        unsafe { SWSSDBConnector_del(self.ptr, key.as_ptr()) == 1 }
    }

    pub fn set(&self, key: &str, val: &CxxStr) {
        let key = cstr(key);
        unsafe { SWSSDBConnector_set(self.ptr, key.as_ptr(), val.as_raw()) };
    }

    pub fn get(&self, key: &str) -> Option<CxxString> {
        let key = cstr(key);
        unsafe {
            let mut ans = SWSSDBConnector_get(self.ptr, key.as_ptr());
            CxxString::take_raw(&mut ans)
        }
    }

    pub fn exists(&self, key: &str) -> bool {
        let key = cstr(key);
        unsafe { SWSSDBConnector_exists(self.ptr, key.as_ptr()) == 1 }
    }

    pub fn hdel(&self, key: &str, field: &str) -> bool {
        let key = cstr(key);
        let field = cstr(field);
        unsafe { SWSSDBConnector_hdel(self.ptr, key.as_ptr(), field.as_ptr()) == 1 }
    }

    pub fn hset(&self, key: &str, field: &str, val: &CxxStr) {
        let key = cstr(key);
        let field = cstr(field);
        unsafe { SWSSDBConnector_hset(self.ptr, key.as_ptr(), field.as_ptr(), val.as_raw()) };
    }

    pub fn hget(&self, key: &str, field: &str) -> Option<CxxString> {
        let key = cstr(key);
        let field = cstr(field);
        unsafe {
            let mut ans = SWSSDBConnector_hget(self.ptr, key.as_ptr(), field.as_ptr());
            CxxString::take_raw(&mut ans)
        }
    }

    pub fn hgetall(&self, key: &str) -> HashMap<String, CxxString> {
        let key = cstr(key);
        unsafe {
            let ans = SWSSDBConnector_hgetall(self.ptr, key.as_ptr());
            take_field_value_array(ans)
        }
    }

    pub fn hexists(&self, key: &str, field: &str) -> bool {
        let key = cstr(key);
        let field = cstr(field);
        unsafe { SWSSDBConnector_hexists(self.ptr, key.as_ptr(), field.as_ptr()) == 1 }
    }

    pub fn flush_db(&self) -> bool {
        unsafe { SWSSDBConnector_flushdb(self.ptr) == 1 }
    }
}

impl Drop for DbConnector {
    fn drop(&mut self) {
        unsafe { SWSSDBConnector_free(self.ptr) };
    }
}

unsafe impl Send for DbConnector {}

#[cfg(feature = "async")]
impl DbConnector {
    async_util::impl_basic_async_method!(new_async <= new(connection: DbConnectionInfo, timeout_ms: u32) -> DbConnector);
    async_util::impl_basic_async_method!(new_named_async <= new_named(db_name: &str, is_tcp_conn: bool, timeout_ms: u32) -> DbConnector);
    async_util::impl_basic_async_method!(new_tcp_async <= new_tcp(db_id: i32, hostname: &str, port: u16, timeout_ms: u32) -> DbConnector);
    async_util::impl_basic_async_method!(new_unix_async <= new_unix(db_id: i32, sock_path: &str, timeout_ms: u32) -> DbConnector);
    async_util::impl_basic_async_method!(clone_timeout_async <= clone_timeout(&self, timeout_ms: u32) -> DbConnector);
    async_util::impl_basic_async_method!(del_async <= del(&self, key: &str) -> bool);
    async_util::impl_basic_async_method!(set_async <= set(&self, key: &str, value: &CxxStr));
    async_util::impl_basic_async_method!(get_async <= get(&self, key: &str) -> Option<CxxString>);
    async_util::impl_basic_async_method!(exists_async <= exists(&self, key: &str) -> bool);
    async_util::impl_basic_async_method!(hdel_async <= hdel(&self, key: &str, field: &str) -> bool);
    async_util::impl_basic_async_method!(hset_async <= hset(&self, key: &str, field: &str, value: &CxxStr));
    async_util::impl_basic_async_method!(hget_async <= hget(&self, key: &str, field: &str) -> Option<CxxString>);
    async_util::impl_basic_async_method!(hgetall_async <= hgetall(&self, key: &str) -> HashMap<String, CxxString>);
    async_util::impl_basic_async_method!(hexists_async <= hexists(&self, key: &str, field: &str) -> bool);
    async_util::impl_basic_async_method!(flush_db_async <= flush_db(&self) -> bool);
}
