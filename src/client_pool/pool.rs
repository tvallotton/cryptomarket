use super::*;

pub struct Pool {
    private_key: String,
    public_key: String,
    clients: Vec<PublicClient>,
    subs: HashMap<(String, &'static str), usize>,
}

impl Pool {
    pub async fn new(private_key: &str, public_key: &str) -> Result<Arc<RwLock<Self>>> {
        Ok(Arc::new(RwLock::new(Self {
            public_key: public_key.into(),
            private_key: private_key.into(),
            subs: Default::default(),
            clients: vec![PublicClient::new(private_key, public_key).await?],
        })))
    }

    fn len(&self) -> usize {
        self.clients.len()
    }

    pub async fn new_client(&mut self) -> Result<&PublicClient> {
        let new_client = PublicClient::new(&self.private_key, &self.public_key).await?;
        self.clients.push(new_client);
        Ok(&self.clients[self.len() - 1])
    }

    pub fn subscribe_last(&mut self, symbols: &[&str], channel: &'static str) {
        for sym in symbols {
            self.subs.insert(((*sym).into(), channel), self.len() - 1);
        }
    }
}
