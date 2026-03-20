//! Structures and traits relating to objects (things that can be rendered).

use crate::geometry::Shape;
use std::ops::Index;

mod pbr_material;
pub use pbr_material::{PbrMaterial, PbrMaterialIdx};

/// Trait representing a renderable object.
pub trait Object<S> {
    /// Returns the shape of the object.
    fn shape<'a>(&'a self, scene: &'a S) -> &'a dyn Shape<S>;

    /// Returns the material of the object.
    fn material<'a>(&'a self, scene: &'a S) -> &'a PbrMaterial;
}

/// Structure that wraps a shape and material index.
///
/// This is likely the structure you want to use for representing objects in a scene.
pub struct WrappedShape<T> {
    shape: T,
    material: PbrMaterialIdx,
}

impl<T> WrappedShape<T> {
    /// Creates a new object from a given shape and material index.
    pub fn new(shape: T, material: PbrMaterialIdx) -> Self {
        Self { shape, material }
    }
}

impl<S, T> Object<S> for WrappedShape<T>
where
    T: Shape<S>,
    S: Index<PbrMaterialIdx, Output = PbrMaterial>,
{
    fn shape<'a>(&'a self, _scene: &'a S) -> &'a dyn Shape<S> {
        &self.shape
    }

    fn material<'a>(&'a self, scene: &'a S) -> &'a PbrMaterial {
        &scene[self.material]
    }
}
