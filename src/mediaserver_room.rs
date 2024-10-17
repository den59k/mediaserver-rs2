use std::sync::Arc;

use tokio::sync::{mpsc::{self, Receiver, Sender}, RwLock};

pub struct MediaServerUser {
  notify: Arc<Sender<String>>
}

pub struct MediaServerRoom {
  users: RwLock<Vec<MediaServerUser>>
}

impl MediaServerRoom {

  pub fn new() -> Self {
    MediaServerRoom {
      users: RwLock::new(vec![])
    }
  }

  pub async fn add_user(&self) -> Receiver<String> {
    let (sender, receiver) = mpsc::channel::<String>(20);
    self.users.write().await.push(MediaServerUser { notify: Arc::new(sender) });
    return receiver;
  }

  pub async fn send_text_message(&self, text: String) {
    let users = self.users.read().await;
    for user in users.iter() {
      let sender = user.notify.clone();
      let text = text.clone();
      tokio::spawn(async move { 
        let _ = sender.send(text).await;
      });
    }
  }
}