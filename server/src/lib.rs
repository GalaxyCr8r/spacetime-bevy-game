use spacetimedb::{spacetimedb, Identity, SpacetimeType, spacetimedb_lib::primary_key};

#[spacetimedb(table)]
pub struct User {
    #[primarykey]
    pub user_id: Identity,
    pub connected: bool
}

#[derive(SpacetimeType, Clone)]
pub struct StdbVector2 {
    pub x: f32,
    pub y: f32,
}

#[spacetimedb(table)]
#[derive(Clone)]
pub struct Object {
    #[primarykey]
    #[autoinc]
    pub object_id: u64,
    pub position: StdbVector2,
    pub velocity: StdbVector2,
}

#[spacetimedb(table)]
#[derive(Clone)]
pub struct Player {
    #[primarykey]
    pub object_id: u64,

    #[unique]
    pub user_id: Identity,
}

#[spacetimedb(init)]
pub fn init() {
    // Called when the module is initially published
}
