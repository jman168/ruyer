//! Structures relating to geometry such as rays and triangles.

mod triangle;
pub use triangle::Triangle;

mod ray;
pub use ray::{Ray, RayIntersection};

mod vertex;
pub use vertex::Vertex;
