use axum::{
    extract::{Path, State},
    routing::{post, delete},
    Json, Router,
};

use crate::{
    model::{ModelController, Ticket, TicketForCreate},
    Error,
};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
    .route("/tickets", post(create_ticket).get(list_ticket))
    .route("/tickets/:id", delete(delete_ticket))
    .with_state(mc)
}
async fn create_ticket(
    mc: State<ModelController>,
    Json(ticket_fc): Json<TicketForCreate>,
) -> Result<Json<Ticket>, Error> {
    println!("--> {:<12} - create_ticket", "HANDLER");
    let ticket = mc.create_ticket(ticket_fc).await?;
    Ok(Json(ticket))
}

async fn list_ticket(mc: State<ModelController>) -> Result<Json<Vec<Ticket>>, Error> {
    println!("--> {:<12} - list_ticket", "HANDLER");
    let ticket = mc.list_tickets().await?;
    Ok(Json(ticket))
}

async fn delete_ticket(
    mc: State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>, Error> {
    println!("--> {:<12} - delete_ticket", "HANDLER");
    let ticket = mc.delete_ticket(id).await?;
    Ok(Json(ticket))
}
