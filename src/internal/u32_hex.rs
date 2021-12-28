use once_cell::sync::Lazy as SyncLazy;
use std::{collections::HashMap, sync::Mutex};

static HEXMAP: SyncLazy<Mutex<HashMap<u32, &'static str>>> =
	SyncLazy::new(|| Mutex::new(HashMap::new()));

pub(crate) fn u32_hex(v: u32) -> &'static str {
	HEXMAP
		.lock()
		.expect("I got poisoned")
		.entry(v)
		.or_insert_with(|| Box::leak(Box::new(format!("{:08X}", v))))
}
