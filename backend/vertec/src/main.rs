//MODULES

// IMPORTS
//  We're using actix-web framework to build our high performance server
//  We're using standard library environment variables to read in config vars (address, port, etc.)
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;

//HANDLE the root endpoint '/'
//  Responder --> Can return any type that implements the 'Responder' trait
//  Here, the response is an HttpResponse, which has a Responder trait with the following:
//      Status: status code of 200 OK (See HTTP Protocol)
//      Body:   'Welcome to the Vertec!'
async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h1>Welcome to the Vertec!</h1>")
}

//ENTRY point of the application.
//  #[actix_web::main] attribute macro simplifies setup and starting of the Actix-Web Server
//  main() is an async function that returns a std::io::Result, which could succeed (Ok(T)) or error (Err(E))
//      Creates a  new HttpServer instance using the new() method
//      We pass a closure to the new() method to configure the server
//      In this case, we create an App instance using the new() method, and define a single route ('/')
//      The route() method takes two arguments:
//          path: '/' --> the string defining the route path
//          route: Route --> A Route type which points to a function that gets called on the defined http method
//                       --> Note, this method can be chained for multiple https methods on the same route (GET,POST,PUT,etc.)
//          
//      The server is bound to the address from environment veriables, or default, using the bind method. Since we return a 
//      Result object, we use the ? operator to propagate any errors up the call stack if an error occurs in the bind process
//      
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //Get the server address from environment variables or use a default value
    let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());

    //Get the server port from the environment variables or use a default value
    let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "8000".to_string());

    let bind_address = format!("{}:{}", server_address,server_port);
    
    //Define the api routes to their handlers
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            // .configure(vertices::handlers::init_routes)
    })
    .bind(bind_address)?
    .run()
    .await
}
