use futures_util::lock::{Mutex, MutexGuard};
use std::sync::Arc;

// Repository stores previous request.
pub struct Repository {
    data: Mutex<String>,
}

// #[inject] macro allow to use `Repository` in `ServiceProvider`
#[teloc::inject]
impl Repository {
    pub fn new() -> Self {
        Repository {
            data: Mutex::new(String::new()),
        }
    }
}

impl Repository {
    pub async fn change(&self, new_data: String) {
        *self.data.lock().await = new_data;
    }

    pub async fn get<'a: 'b, 'b>(&'a self) -> MutexGuard<'b, String> {
        self.data.lock().await
    }
}

// Service that handles requests.
pub struct ActixService {
    store: Arc<Repository>,
}

// #[inject] macro allow to use `ActixService` in `ServiceProvider`
#[teloc::inject]
impl ActixService {
    pub fn new(store: Arc<Repository>) -> Self {
        ActixService { store }
    }
}

impl ActixService {
    pub async fn change_and_get_previous(&self, new_data: String) -> String {
        let previous = self.store.get().await.clone();
        self.store.change(new_data).await;
        previous
    }
}
