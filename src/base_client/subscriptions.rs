use super::Response;
use crate::prelude::Result;
use log::error;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use tokio::sync::Mutex;

pub(crate) type ClosureOutput =
    Pin<Box<dyn Future<Output = Result<(), Response>> + Send + Sync + 'static>>;
pub(crate) trait Closure: Fn(Response) -> ClosureOutput + Send + Sync + 'static {}
impl<F: Fn(Response) -> ClosureOutput + Send + Sync + 'static> Closure for F {}

#[derive(Default)]
pub struct Subscriptions(Mutex<HashMap<String, Box<dyn Closure>>>);

impl Subscriptions {
    pub(crate) async fn insert<F: Closure>(&self, method: &str, f: F) {
        self.0.lock().await.insert(method.into(), Box::new(f));
    }

    pub async fn intercept(&self, response: Response) -> Result<(), Response> {
        let method = response.method();
        if let Some(method) = method {
            self.send(method, response).await
        } else {
            Err(response)
        }
    }

    async fn send(&self, method: String, response: Response) -> Result<(), Response> {
        let hashmap = self.0.lock().await;
        if let Some(f) = hashmap.get(&*method) {
            match f(response).await {
                | Ok(_) => Ok(()),
                | Err(res) => {
                    error!("Error sending response: {:?}", res);
                    Err(res)
                }
            }
        } else {
            Err(response)
        }
    }

    pub async fn remove(&self, method: &'static str) {
        self.0.lock().await.remove(method);
    }
}
