use super::{Direction, EntityID};

pub struct RenderInfo<'a> {
    pub short: &'a str,
    pub long: &'a str
}
pub trait HasRender {
    fn get(&self) -> RenderInfo;
}
pub struct StorageChestInfo<'a> {
    pub contains: Vec<&'a str>,
    pub capacity: usize,
}
pub trait HasStorageChest {
    fn get(&self) -> StorageChestInfo;
}
pub struct StorageFacetInfo<'a> {
    pub contains: Vec<(&'a str, Direction)>
}
pub trait HasStorageFacet {
    fn get(&self) -> StorageFacetInfo;
}
pub struct StorageCharacterInfo;

pub trait HasStorageCharacter {
    fn get(&self) -> StorageCharacterInfo;
}
pub struct PortalInfo<'a> {
    pub to: &'a str, 
}
pub trait HasPortal {
    fn get(&self) -> PortalInfo;
}
pub struct MovableInfo {
    pub movable: bool,
}
pub trait HasMovable {
    fn get(&self) -> MovableInfo;
}
pub struct SpawnerInfo;
pub trait HasSpawner {
    fn get(&self) -> SpawnerInfo;
}

