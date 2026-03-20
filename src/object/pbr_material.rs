use glam::Vec3;
#[cfg(test)]
use std::ops::Index;

/// Structure for storing the physical material properties of a rendered object.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PbrMaterial {
    color: Vec3,
    emission: Vec3,
}

impl PbrMaterial {
    /// Creates a new [`PbrMaterial`] from its given material properties.
    pub fn new(color: Vec3, emission: Vec3) -> Self {
        Self { color, emission }
    }

    /// Returns the color of the material.
    pub fn color(&self) -> &Vec3 {
        &self.color
    }

    /// Returns the emission of the material.
    pub fn emission(&self) -> &Vec3 {
        &self.emission
    }
}

/// Structure used for accessing a [`PbrMaterial`] by its index in some container.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PbrMaterialIdx(pub u32);

#[cfg(test)]
impl Index<PbrMaterialIdx> for Vec<PbrMaterial> {
    type Output = PbrMaterial;

    fn index(&self, index: PbrMaterialIdx) -> &Self::Output {
        &self[index.0 as usize]
    }
}
