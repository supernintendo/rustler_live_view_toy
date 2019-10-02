use bytes::Bytes;
use std::collections::HashMap;
use serde_json::{Value, Map};

pub type Avatars = HashMap<Bytes, Map<String, Value>>;
