use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use actix_web::web::Payload;
use futures::StreamExt;
use uuid::Uuid;
use crate::vertices::vertex::Vertex;

//Configure Vertex Handlers
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_vertex);
    cfg.service(get_vertex);
    cfg.service(update_vertex);
    cfg.service(delete_vertex);
}

#[post("/vertex")]
pub async fn create_vertex(mut payload: Payload) -> impl Responder {
    // Concatenate all chunks in the payload
    let body = payload.next().await;

    let vertex: Result<Vertex, serde_json::Error> = match body {
        Some(Ok(bytes)) => {
            // If there are bytes, attempt to deserialize them into a Vertex
            serde_json::from_slice::<Vertex>(&bytes)
        },
        Some(Err(err)) => {
            // If there was a problem reading the bytes, return an error
            return HttpResponse::InternalServerError().body(format!("Payload error - Invalid JSON data for new Vertex: {:?}", err));
        },
        None => {
            // If there are no bytes, create a default vertex
            let default_vertex = Vertex::default();
            return HttpResponse::Ok().json(default_vertex);
        },
    };

    match vertex {
        Ok(vertex) => HttpResponse::Ok().json(vertex),
        Err(_) => HttpResponse::BadRequest().body("Invalid Vertex data as JSON"),
    }
}

#[get("/vertex/{id}")]
pub async fn get_vertex(id: Result<web::Path<Uuid>, actix_web::Error>) -> impl Responder {
    match id{
        Ok(id) =>{
            //Look up the vertex in the database and return the vertex
            //TODO: implement the actual database query
            HttpResponse::Ok().body(format!("Vertex ID: {}",id))        
        },
        Err(_) => {
            HttpResponse::BadRequest().body("Invalid UUID")
        }
    }
    
}

#[put("/vertex/{id}")]
pub async fn update_vertex(
    id: Result<web::Path<Uuid>, actix_web::Error>,
    new_info: Result<web::Json<Vertex>, actix_web::Error>
) -> impl Responder {
    match id{
        Ok(id) =>{

            match new_info{
                Ok(new_info) => {
                    // Use the id to update the corresponding vertex in the database with the new data
                    // TODO: implement the actual database update
                    HttpResponse::Ok().body(format!("Updated Vertex {} with id {} name {} and type{}",
                    id, new_info.id, new_info.name, new_info.vertex_type))
                },
                Err(_) => {
                    HttpResponse::BadRequest().body(format!("Invalid Vertex data as JSON for vertex id {}",id))
                }
            }  
        },
        Err(_) => {
            HttpResponse::BadRequest().body("Invalid UUID")
        }
    }
     
}

#[delete("/vertex/{id}")]
pub async fn delete_vertex(id: Result<web::Path<Uuid>, actix_web::Error>) -> impl Responder {
    match id{
        Ok(id)=>{
            //Look up the vertex in the database and return the vertex
            //TODO: implement the actual database query
            HttpResponse::Ok().body(format!("Deleted Vertex ID: {}",id))
        },
        Err(_) =>{
            HttpResponse::BadRequest().body("Invalid UUID")
        }
    }   
}