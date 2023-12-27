use std::{ops::Deref, sync::Arc};

use shuttle_persist::PersistInstance;

/// Defines the Api State that can contain things like persistance, db, env vars, etc.
///
/// It is behind an AtomicReferenceCounter so it is safe to clone between threads and points to the
/// shared memory space.
#[derive(Clone)]
pub struct ApiState(Arc<ApiStateInner>);

pub struct ApiStateInner {
    pub persist: PersistInstance,
}

impl ApiState {
    pub fn new(persist: PersistInstance) -> Self {
        ApiState(Arc::new(ApiStateInner { persist }))
    }
}

impl Deref for ApiState {
    type Target = ApiStateInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
