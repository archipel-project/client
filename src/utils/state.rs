use bevy::prelude::Resource;

use crate::models::session::{self, User};

#[derive(Resource)]
pub struct State {
    pub session_type: session::Type,
    pub user: User
}

impl State {
  pub fn is_offline(&self) -> bool {
    matches!(self.session_type, session::Type::Offline)
  }

  pub fn is_online(&self) -> bool {
    matches!(self.session_type, session::Type::Online)
  }
  
  pub fn toggle_session(&mut self) {
    self.session_type = match self.session_type {
      session::Type::Offline => session::Type::Online,
      session::Type::Online => session::Type::Offline
    };
  }
}

impl Default for State {
    fn default() -> Self {
      Self {
        session_type: session::Type::Online,
        user: User::default()
      }
    }
  }
 