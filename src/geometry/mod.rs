//! Structures and traits relating to geometry such as rays and triangles.

mod shape;
pub use shape::Shape;

mod ray;
pub use ray::{Ray, RayIntersection};

mod vertex;
pub use vertex::{Vertex, VertexIdx};

mod triangle;
pub use triangle::{Triangle, TriangleIdx};
