use core::sync::atomic::{AtomicU8, Ordering};

static LAST_KEY: AtomicU8 = AtomicU8::new(0);

pub fn push_key(key: u8) {
    LAST_KEY.store(key, Ordering::SeqCst);
}

pub fn pop_key() -> Option<u8> {
    let key = LAST_KEY.swap(0, Ordering::SeqCst);
    if key == 0 {
        None
    } else {
        Some(key)
    }
}
