use log::info;
use std::time::Duration;
use spacetimedb::{rand::Rng, Identity, SpacetimeType, ReducerContext, ScheduleAt, Table, table, Timestamp};

#[table(name = stdb_client, public)]
pub struct StdbClient {
    #[primary_key]
    pub client_id: Identity,
    pub connected: bool,
}

// This allows us to store 2D points in tables.
#[derive(SpacetimeType, Clone, Default, Debug)]
pub struct StdbVector2 {
    pub x: f32,
    pub y: f32,
}

#[table(name = stdb_object, public)]
#[derive(Clone, Default)]
pub struct StdbObject {
    #[primary_key]
    #[auto_inc]
    pub object_id: u64,
    pub name: String,

    pub position: StdbVector2,
}

#[table(name = stdb_player, public)]
pub struct StdbPlayer {
    #[primary_key]
    pub object_id: u64,

    #[unique]
    pub client_id: Identity,
}

#[spacetimedb::reducer(init)]
pub fn init(_ctx: &ReducerContext) {
    // Called when the module is initially published
}

// Called when the client connects, we update the logged_in state to true
#[spacetimedb::reducer(client_connected)]
pub fn identity_connected(ctx: &ReducerContext) {
    // called when the client connects, we update the logged_in state to true
    update_client_login_state(ctx, true);
}

// Called when the client disconnects, we update the logged_in state to false
#[spacetimedb::reducer(client_disconnected)]
pub fn identity_disconnected(ctx: &ReducerContext) {
    // Called when the client disconnects, we update the logged_in state to false
    update_client_login_state(ctx, false);
}

// This helper function gets the PlayerComponent, sets the logged
// in variable and updates the PlayerComponent table row.
pub fn update_client_login_state(ctx: &ReducerContext, connected: bool) {
    if let Some(stdb_client) = ctx.db.stdb_client().client_id().find(ctx.sender) {
        // If this is a returning client, i.e. we already have a `StdbClient` with this `Identity`,
        // set `online: true`, but leave the rest unchanged.
        log::info!("StdbClient {:?} connection changed to: {}", ctx.sender.to_abbreviated_hex(), connected);
        ctx.db.stdb_client().client_id().update(StdbClient { connected: connected, ..stdb_client });
    } else {
        // If this is a new client, create a `StdbClient` row for the `Identity`,
        // which is online, but hasn't set a name.      
        ctx.db.stdb_client().try_insert(StdbClient {
            client_id: ctx.sender,
            connected: connected,
        })
        .expect("Failed to create a unique Client");
        info!("Created Client");
    }
}

// This reducer is called when the user logs in for the first time and
// enters a username
#[spacetimedb::reducer]
pub fn create_player(ctx: &ReducerContext) -> Result<(), String> {
    // Get the Identity of the client who called this reducer
    let client_id = ctx.sender;

    // Make sure we don't already have a player with this identity
    //if StdbPlayer::filter_by_client_id(&client_id).is_some() {
    if ctx.db.stdb_player().client_id().find(client_id).is_some() {
        log::info!("Player already exists");
        return Err("Player already exists".to_string());
    }

    // Create a new entity for this player and get a unique `entity_id`.
    let object_id = ctx.db.stdb_object().try_insert(StdbObject::default()) // StdbObject::insert(StdbObject::default())
        .expect("Failed to create a unique Player.")
        .object_id;

    // The PlayerComponent uses the same entity_id and stores the identity of
    // the owner, username, and whether or not they are logged in.
    // StdbPlayer::insert(StdbPlayer {
    //     object_id,
    //     client_id,
    // })
    ctx.db.stdb_player().try_insert(StdbPlayer {
        object_id: object_id,
        client_id: client_id,
    })
    .expect("Failed to insert Player.");

    log::info!("Player created: {}", object_id);

    Ok(())
}

pub fn remove_player(ctx: &ReducerContext) -> Result<(), String> {
    let client_id = ctx.sender;

    if !ctx.db.stdb_player().client_id().find(client_id).is_some() {
        log::info!("Player doesn't exist");
        return Err("Player doesn't exist".to_string());
    }

    if let Some(player) = ctx.db.stdb_player().client_id().find(client_id) {
        ctx.db.stdb_player().delete(player);
        log::info!("Removed Player: {:?}", client_id);
    }

    Ok(())
}

#[spacetimedb::reducer]
pub fn update_player_pos(ctx: &ReducerContext, position: StdbVector2) -> Result<(), String> {
    if let Some(player) = ctx.db.stdb_player().client_id().find(ctx.sender) {
        if let Some(mut object) = ctx.db.stdb_object().object_id().find(player.object_id) { //StdbObject::filter_by_object_id(&player.object_id) {
            object.position = position;
            //StdbObject::update_by_object_id(&player.object_id, object);
            ctx.db.stdb_object().object_id().update( object );
            return Ok(());
        }
    }

    return Err("Player not found".to_string());
}
