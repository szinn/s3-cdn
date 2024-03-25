use std::sync::Arc;

use async_trait::async_trait;

use super::{document_service::DocumentService, store_service::StoreService};

pub struct Server {
    store: Arc<dyn StoreService>,
}

impl Server {
    pub fn new(store: Arc<dyn StoreService>) -> Self {
        Self { store }
    }
}

#[async_trait]
impl DocumentService for Server {}
