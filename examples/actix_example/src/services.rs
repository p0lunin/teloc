use actix_web::http::Method;
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

    pub async fn get(&self) -> MutexGuard<'_, String> {
        self.data.lock().await
    }
}

// Service that handles requests.
pub struct ActixService<'a> {
    store: Arc<Repository>,
    method: &'a Method,
}

// #[inject] macro allow to use `ActixService` in `ServiceProvider`
#[teloc::inject]
impl<'a> ActixService<'a> {
    pub fn new(store: Arc<Repository>, method: &'a Method) -> Self {
        Self { store, method }
    }
}

impl ActixService<'_> {
    pub async fn change_and_get_previous(&self, new_data: String) -> String {
        let previous = self.store.get().await.clone();
        self.store.change(new_data).await;
        format!(
            "Request Method: {}\nPrevious request body: {}\n",
            self.method, previous
        )
    }
}
