use crate::type_alias::*;
use futures::{StreamExt, *};
use requests::Requests;
use tokio::{
    sync::{mpsc, oneshot, Mutex}, //
    time::timeout,
};
use tokio_tungstenite::connect_async;

use crate::{base_client::subscriptions::Subscriptions, prelude::*};
// "wss://api.exchange.cryptomkt.com/api/3/ws/public"

mod requests;
mod subscriptions;
struct WebsocketClient {
    reader: Mutex<Reader>,
    writer: Mutex<Writer>,
}
struct InnerClient {
    private_key: hmac::Key,
    public_key: String,
    url: String,
    requests: Requests,
    subscriptions: Subscriptions,
    ws: WebsocketClient,
    is_auth: Mutex<bool>,
}
#[derive(Clone)]
pub(crate) struct BaseClient(Arc<InnerClient>);

impl BaseClient {
    pub async fn new(priv_k: &str, pub_k: &str, url: &str) -> Result<Self> {
        let private_key = hmac::Key::new(hmac::HMAC_SHA256, priv_k.as_bytes());
        log::debug!("Connecting to cryptomkt's websocket API.");
        let (ws, _) = connect_async(url).await?;

        let (writer, reader) = ws.split();
        let writer = Mutex::new(writer);
        let reader = Mutex::new(reader);

        let client = Self(Arc::new(InnerClient {
            private_key,
            public_key: pub_k.into(),
            url: url.into(),
            requests: Requests::default(),
            subscriptions: Subscriptions::default(),
            ws: WebsocketClient { reader, writer },
            is_auth: Mutex::default(),
        }));
        client.clone().run_forever().await;
        Ok(client)
    }
    pub async fn is_auth(&self) -> bool {
        *self.0.is_auth.lock().await
    }
    pub async fn emit(&self, data: &impl Serialize) -> Result {
        let mut writer = self.0.ws.writer.lock().await;
        let json = serde_json::to_string(data).unwrap();
        writer.send(json.into()).await?;
        Ok(())
    }

    pub async fn authenticate(&self) -> Result<()> {
        let request = Request::login(&self.0.private_key, &self.0.public_key);
        let res = self.request(&request).await?;
        match res {
            | Response::Error(error) => Err(Error::from(error)),
            | _ => {
                let mut is_auth = self.0.is_auth.lock().await;
                *is_auth = res.success();
                Ok(())
            }
        }
    }

    pub async fn request(&self, req: &Request<'_>) -> Result<Response> {
        let (tx, rx) = oneshot::channel();
        let id = req.id();
        self.0.requests.insert(id, tx).await;
        self.emit(req).await?;
        if let Ok(res) = timeout(crate::TIMEMOUT, rx).await {
            match res? {
                | Response::Error(error) => Err(Error::from(error)),
                | res => Ok(res),
            }
        } else {
            Err(Error::Timeout)
        }
    }

    async fn register_subs<F>(&self, method: &str, f: F)
    where
        F: subscriptions::Closure,
    {
        self.0.subscriptions.insert(method, Box::new(f)).await;
    }

    pub async fn subscribe<T>(
        &self,
        method: &str,
        tx: mpsc::Sender<T>,
        f: fn(Response) -> Result<T, Response>,
    ) where
        T: Send + Sync + 'static,
    {
        self.register_subs(method, move |res| {
            let tx = tx.clone();
            Box::pin(async move {
                let t = f(res)?;
                (tx).send(t).await.ok();
                Ok(())
            }) as subscriptions::ClosureOutput
        })
        .await;
    }
    pub async fn subscribe_vec<T>(
        &self,
        method: &'static str,
        tx: Sender<T>,
        f: fn(Response) -> Result<Vec<T>, Response>,
    ) where
        T: Send + Sync + 'static,
    {
        self.register_subs(method, move |res| {
            let tx = tx.clone();

            Box::pin(async move {
                let objects = f(res)?;
                for obj in objects {
                    tx.send(obj).await.ok();
                }
                Ok(())
            }) as subscriptions::ClosureOutput
        })
        .await;
    }

    pub async fn unsubscribe(&self, method: &'static str) {
        self.0.subscriptions.remove(method).await;
    }

    pub async fn recv(&self) -> Result {
        let mut reader = self.0.ws.reader.lock().await;
        let msg = reader.next().await.ok_or(Error::Closed)??;
        self.send(msg).await
    }

    pub async fn send(&self, msg: Message) -> Result {
        let msg = msg.to_string();
        if msg.is_empty() {
            return Ok(());
        }
        let res: Response = serde_json::from_str(&msg)?;

        if let Err(res) = self.0.requests.intercept(res).await {
            if let Err(res) = self.0.subscriptions.intercept(res).await {
                error!(
                    "unhandeled message: {}",
                    serde_json::to_string(&res).unwrap()
                );
            }
        }
        Ok(())
    }

    async fn reconnect(&self) -> Result {
        let (ws, _) = tokio_tungstenite::connect_async(&self.0.url).await?;
        let (writer, reader) = ws.split();
        let mut w = self.0.ws.writer.lock().await;
        let mut r = self.0.ws.reader.lock().await;
        *r = reader;
        *w = writer;
        Ok(())
    }
    async fn run_forever(self) {
        spawn(async move {
            loop {
                match self.recv().await {
                    | Ok(()) => (),
                    | Err(error) => {
                        error!("cryptmkt error: {}", error);
                        info!("reconnecting to cryptomkt.");
                        if let Err(err) = self.reconnect().await {
                            error!("reconnection error: {}", err);
                        }
                        if self.is_auth().await {
                            self.authenticate()
                                .await
                                .map_err(|err| {
                                    error!("authentication error: {}", err);
                                })
                                .ok();
                        }
                    }
                }
            }
        });
    }
}
