use serde_json::{json, Value};
use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
};

pub struct Progress {
    token: Value,
    write: Box<dyn Fn(Value) + Send + Sync>,
    state: Mutex<(u64, bool)>,
}

impl Progress {
    pub fn new(token: Value, write: impl Fn(Value) + Send + Sync + 'static) -> Arc<Self> {
        Arc::new(Self {
            token,
            write: Box::new(write),
            state: Mutex::new((0, false)),
        })
    }
    pub fn emit(&self, message: &str) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        if !state.1 {
            state.0 += 1;
            (self.write)(
                json!({"jsonrpc":"2.0","method":"notifications/progress","params":{"progressToken":self.token,"progress":state.0,"message":message}}),
            );
        }
    }
    pub fn close(&self) {
        self.state.lock().unwrap_or_else(|e| e.into_inner()).1 = true;
    }
    pub fn count(&self) -> u64 {
        self.state.lock().unwrap_or_else(|e| e.into_inner()).0
    }
}

thread_local! { static CURRENT: RefCell<Option<Arc<Progress>>> = const { RefCell::new(None) }; }

pub fn current() -> Option<Arc<Progress>> {
    CURRENT.with(|p| p.borrow().clone())
}

pub fn scope<T>(p: Option<Arc<Progress>>, f: impl FnOnce() -> T) -> T {
    struct Guard {
        previous: Option<Arc<Progress>>,
        progress: Option<Arc<Progress>>,
    }
    impl Drop for Guard {
        fn drop(&mut self) {
            if let Some(p) = &self.progress {
                p.close();
            }
            CURRENT.with(|slot| *slot.borrow_mut() = self.previous.take());
        }
    }
    let previous = CURRENT.with(|slot| slot.replace(p.clone()));
    let _guard = Guard {
        previous,
        progress: p,
    };
    f()
}
