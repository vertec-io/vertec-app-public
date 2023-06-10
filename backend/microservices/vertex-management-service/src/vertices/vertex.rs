use serde::{Deserialize, Serialize};
use uuid::Uuid;

//Vertex struct
#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct Vertex {
    pub id: Uuid,
    pub name: String,
    pub vertex_type: String,
    pub configuration: VertexConfiguration,
    pub relationships: Vec<Uuid>,
}

impl Default for Vertex {
    fn default() -> Self {
        Vertex { id: Uuid::new_v4(), //Generate a random UUID
                 name: String::from("Default Vertex"),
                 vertex_type: String::from("Default Type"), 
                 configuration: VertexConfiguration::default(), 
                 relationships: Vec::new() 
                }
    }
}

//Vertex configuration struct
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VertexConfiguration {
     // Add the specific parameters that make up a vertex's configuration
     name: String
}

impl Default for VertexConfiguration {
    fn default() -> Self {
        VertexConfiguration { name: String::from("New Vertex Configuration") }
    }
}