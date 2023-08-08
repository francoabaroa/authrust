use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

#[derive(Clone)]
pub struct UserSession {
    pub username: String,
    pub email: String,
}

pub struct SessionStore(RwLock<HashMap<String, UserSession>>);

impl SessionStore {
    pub fn new() -> Self {
        SessionStore(RwLock::new(HashMap::new()))
    }

    pub fn create_session(&self, user: UserSession) -> String {
        let session_id = Uuid::new_v4().to_string();
        let mut sessions = self.0.write().unwrap();
        sessions.insert(session_id.clone(), user);
        session_id
    }

    pub fn get_user(&self, session_id: &str) -> Option<UserSession> {
        let sessions = self.0.read().unwrap();
        sessions.get(session_id).cloned()
    }

    pub fn remove_session(&self, session_id: &str) {
        let mut sessions = self.0.write().unwrap();
        sessions.remove(session_id);
    }
}
