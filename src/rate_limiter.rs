use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};

struct RateLimiter {
    usage_counter: Arc<HashMap<String, (Mutex<i32>, DateTime<Utc>)>>
}