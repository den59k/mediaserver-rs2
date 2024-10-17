use std::{collections::{HashMap}, sync::Arc};

use futures_util::lock::Mutex;

use crate::mediaserver_room::MediaServerRoom;


// Struct store rooms
pub struct MediaServer {

  rooms: Mutex<HashMap<String, Arc<MediaServerRoom>>>
  
}

impl MediaServer {
  pub fn new() -> Self {
    MediaServer {
      rooms: Mutex::new(HashMap::new())
    }
  }
  
  pub async fn get_room(&self, room_id: String) -> Arc<MediaServerRoom> {
    let mut rooms = self.rooms.lock().await;
    let room = rooms
      .entry(room_id)
      .or_insert(Arc::new(MediaServerRoom::new()));
    return room.clone();
  }
}