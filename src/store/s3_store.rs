use async_trait::async_trait;

use crate::core::store_service::StoreService;

pub struct S3Store {}

impl S3Store {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl StoreService for S3Store {}
