use crate::Point3D;

/// An axis aligned 3D box represented by its minimum and maximum coordinates.
#[repr(C)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(
    feature = "serde",
    serde(bound(serialize = "T: Serialize", deserialize = "T: Deserialize<'de>"))
)]
pub struct Box3D<T> {
    pub min: Point3D<T>,
    pub max: Point3D<T>,
}