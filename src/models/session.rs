use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct User {
  pub username: String,
  pub password: String
}

#[derive(Resource, PartialEq, Eq)]
pub enum Type {
    Online,
    Offline,
}