use std::{
    collections::HashSet,
    sync::LazyLock
};

pub static REQUEST_HEADERS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        "accept", "accept-charset", "accept-encoding", "accept-language",
        "authorization", "expect", "from", "host", "if-match", "if-modified-since",
        "if-none-match", "if-range", "if-unmodified-since", "max-forwards",
        "proxy-authorization", "range", "referer", "te", "user-agent",
    ])
});

pub static GENERAL_HEADERS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        "cache-control", "connection", "date", "pragma", "trailer", "transfer-encoding",
        "upgrade", "via", "warning",
    ])
});

pub static ENTITY_HEADERS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        "allow", "content-encoding", "content-language", "content-length", "content-location",
        "content-md5", "content-range", "content-type", "expires", "last-modified",
    ])
});