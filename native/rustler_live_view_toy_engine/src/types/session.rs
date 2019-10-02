use bytes::Bytes;
use rand::prelude::*;
use serde_json::{Value, Map, Number};
use std::str;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::net::SocketAddr;

use super::avatars::Avatars;
use super::channel::Tx;
use super::config::Config;

/// Data shared between all connected clients.
pub struct Session {
    pub config: Config,
    pub avatars: Arc<Mutex<Avatars>>,
    pub entities: HashMap<Bytes, Tx>,
    pub master_key: Bytes
}

impl Session {
    pub fn new(config: Config, avatars: Arc<Mutex<Avatars>>) -> Self {
        let master_key = config.master_key.clone();

        Session {
            avatars: avatars,
            config: config,
            entities: HashMap::new(),
            master_key: Bytes::from(master_key)
        }
    }

    pub fn add_entity(&mut self, _addr: SocketAddr, entity_key: &Bytes, tx: Tx) -> bool {
        &self.entities.insert(entity_key.clone(), tx);

        if &self.master_key == &entity_key {
            true
        } else {
            &self.add_avatar(entity_key.clone());

            false
        }
    }

    pub fn remove_entity(&mut self, entity_key: &Bytes) {
        &self.entities.remove(&entity_key.clone());
        &self.avatars.lock().unwrap().remove(&entity_key.clone());

        let mut op = Map::new();

        op.insert("_op".to_string(), Value::String("remove_avatar".to_string()));
        op.insert("key".to_string(), Value::String(str::from_utf8(&entity_key).unwrap().to_string()));

        &self.push_update(serde_json::to_string(&op).unwrap());
    }

    pub fn add_avatar(&mut self, entity_key: Bytes) {
        let mut avatar = Map::new();
        let r = rand::thread_rng().gen_range(0, 255);
        let g = rand::thread_rng().gen_range(0, 255);
        let b = rand::thread_rng().gen_range(0, 255);
        let x = rand::thread_rng().gen_range(0, 100);
        let y = rand::thread_rng().gen_range(0, 100);

        avatar.insert("_op".to_string(), Value::String("add_avatar".to_string()));
        avatar.insert("key".to_string(), Value::String(str::from_utf8(&entity_key).unwrap().to_string()));
        avatar.insert("speed".to_string(), Value::Number(Number::from(10)));
        avatar.insert("r".to_string(), Value::Number(Number::from(r)));
        avatar.insert("g".to_string(), Value::Number(Number::from(g)));
        avatar.insert("b".to_string(), Value::Number(Number::from(b)));
        avatar.insert("x".to_string(), Value::Number(Number::from(x)));
        avatar.insert("y".to_string(), Value::Number(Number::from(y)));
        avatar.insert("x_velocity".to_string(), Value::Number(Number::from(0)));
        avatar.insert("y_velocity".to_string(), Value::Number(Number::from(0)));

        &self.push_update(serde_json::to_string(&avatar).unwrap());
        &self.avatars.lock().unwrap().insert(entity_key.clone(), avatar.clone());
    }

    pub fn push_update(&mut self, serialized: String) {
        for (id, tx) in &self.entities {
            if &self.master_key == id {
                let response = Bytes::from(serialized.clone());

                tx.unbounded_send(response).unwrap();
            }
        }
    }

    pub fn handle_message(&mut self, entity_key: Bytes, message: String) {
        let mut op = Map::new();

        op.insert("_op".to_string(), Value::String("update_avatar".to_string()));
        op.insert("key".to_string(), Value::String(str::from_utf8(&entity_key).unwrap().to_string()));
        op.insert("value".to_string(), Value::String(message.to_string()));

        &self.push_update(serde_json::to_string(&op).unwrap());
    }
}
