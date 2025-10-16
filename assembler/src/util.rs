use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::{
    fmt::Debug,
    sync::atomic::{AtomicU32, Ordering},
};

/// A compact identifier for an interned string.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct StringId(u32);

impl Debug for StringId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        resolve(*self).fmt(f)
    }
}

static INTERNER: Lazy<StringInterner> = Lazy::new(StringInterner::new);

struct StringInterner {
    next_id: AtomicU32,
    str_to_id: DashMap<String, StringId>,
    id_to_str: DashMap<StringId, String>,
}

impl StringInterner {
    fn new() -> Self {
        Self {
            next_id: AtomicU32::new(0),
            str_to_id: DashMap::new(),
            id_to_str: DashMap::new(),
        }
    }

    fn intern(&self, s: &str) -> StringId {
        // Fast path: check if already interned
        if let Some(id) = self.str_to_id.get(s) {
            return *id;
        }

        // Allocate new ID
        let id = StringId(self.next_id.fetch_add(1, Ordering::Relaxed));
        let s_owned = s.to_string();

        // Insert both directions
        self.str_to_id.insert(s_owned.clone(), id);
        self.id_to_str.insert(id, s_owned);

        id
    }

    fn resolve(&self, id: StringId) -> String {
        self.id_to_str
            .get(&id)
            .map(|s| s.clone())
            .expect("invalid StringId")
    }
}

/// Interns a string and returns its unique `StringId`.
pub fn intern<S: AsRef<str>>(s: S) -> StringId {
    INTERNER.intern(s.as_ref())
}

/// Resolves a `StringId` back into the original string.
pub fn resolve(id: StringId) -> String {
    INTERNER.resolve(id)
}
