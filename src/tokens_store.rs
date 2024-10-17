use std::{collections::HashMap, sync::Arc, time::Instant};
use rand::{distributions::Alphanumeric, Rng};

use tokio::{sync::Mutex, task::JoinHandle};

fn generate_token(length: usize) -> String {
  let token: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(length)
    .map(char::from)
    .collect();
  token
}

pub struct TokenStoreEntry<T> {
  dispose_handle: JoinHandle<()>,
  used_at: Option<Instant>,
  payload: Arc<T>
}

// Struct which stored user tokens and user info
pub struct TokensStore<T> {
  tokens: Arc<Mutex<HashMap<String, TokenStoreEntry<T>>>>
}

impl<T: Send + Sync + 'static> TokensStore<T> {
  pub fn new() -> Self {
    TokensStore {
      tokens: Arc::new(Mutex::new(HashMap::new()))
    }
  }

  pub async fn register_token(&self, payload: T) -> String {
    let token = generate_token(40);

    self.tokens.lock().await.insert(token.clone(), TokenStoreEntry { 
      payload: Arc::new(payload), 
      used_at: None,
      dispose_handle: self.create_dispose_task(&token)
    });
    token
  }

  pub async fn use_token(&self, token: &String) -> Option<Arc<T>> {
    let mut map = self.tokens.lock().await;
    let val = map.get_mut(token)?;
    val.used_at = Some(Instant::now());
    val.dispose_handle.abort();
    Some(val.payload.clone())
  }

  pub async fn release_token(&self, token: &String) {
    let mut map = self.tokens.lock().await;
    if let Some(val) = map.get_mut(token) {
      val.used_at = None;

      val.dispose_handle = self.create_dispose_task(&token);
    } 
  }
  
  fn create_dispose_task(&self, token: &String) -> JoinHandle<()> {
    let tokens_clone = self.tokens.clone();
    let token_clone = token.clone();
    let dispose_handle = tokio::spawn(async move {
      tokio::time::sleep(std::time::Duration::from_secs(5)).await;
      let mut tokens = tokens_clone.lock().await;
      let _ = tokens.remove(&token_clone);
      println!("Token aborted. Tokens active: {}", tokens.len());
    });
    return dispose_handle
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn register_token() {
    let tokens_store: TokensStore<String> = TokensStore::new();

    let _ = tokens_store.register_token("test-value".to_string()).await;

  }

}