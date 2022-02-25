use tokio::sync::{oneshot::Sender, Mutex};

use crate::prelude::*;
#[derive(Default)]
pub struct Requests(Mutex<HashMap<i64, (i64, Sender<Response>)>>);

impl Requests {
    // It is used when making a request to register a response.
    pub async fn insert(&self, id: i64, tx: Sender<Response>) {
        let time = chrono::Utc::now().timestamp();
        let mut hashmap = self.0.lock().await;
        hashmap.insert(id, (time, tx));
        if hashmap.len() != 0 {
            drop(hashmap);
            self.clear().await;
        }
    }

    // deletes all entries that are older that 20 seconds.
    async fn clear(&self) {
        let now = chrono::Utc::now().timestamp();
        let mut hashmap = self.0.lock().await;
        hashmap.retain(|_, (time, _)| now < 20 + *time);
    }

    // given a API response, it will try to send it to an awaiting task if there is one.
    // if no tasks are waiting it will return the response so it can be streamed.
    pub async fn intercept(&self, response: Response) -> Result<(), Response> {
        let id = response.id();
        if let Some(id) = id {
            self.send_response(id, response).await
        } else {
            Err(response)
        }
    }

    async fn send_response(&self, id: i64, response: Response) -> Result<(), Response> {
        let mut hashmap = self.0.lock().await;
        if let Some((_, (_, tx))) = hashmap.remove_entry(&id) {
            tx.send(response)?;
            Ok(())
        } else {
            Err(response)
        }
    }
}
