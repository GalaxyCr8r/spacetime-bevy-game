use bevy::prelude::Component;
use spacetimedb_sdk::identity::Identity;

#[derive(Component)]
pub struct Owner {
    pub id: Identity
}