use anyhow::Result;
use super::components;
use super::EntityManager;
use super::map;
use super::EntityID;
use super::maptraits::*;


pub struct GameState {
    pub game_name: String,
    pub entities: EntityManager,
    pub render: components::Render,
    pub location: components::Location,
    pub portal: components::Portal,
    pub player: components::Player,
    pub storage: components::Storage,
    pub movable: components::Movable,
}

impl GameState {

    pub fn new() -> GameState {
        return GameState {
            game_name: "".to_owned(), 
            entities: EntityManager::new(),
            render: components::Render::new(),
            location: components::Location::new(),
            portal: components::Portal::new(),
            player: components::Player::new(),
            storage: components::Storage::new(),
            movable: components::Movable::new(),
        }
    }
    
    fn add_comp_render<T: HasRender>(&mut self, id: EntityID, x: &T) -> Result<()> {
        let info = x.get();
        self.render.add(id, info.short, info.long);
        return Ok(());
    }

    fn add_comp_movable<T: HasMovable>(&mut self, id: EntityID, x: &T) -> Result<()> {
        let info = x.get();
        self.movable.add(id, info.movable);
        return Ok(());
    }

    fn add_comp_storage_chest<T: HasStorageChest>(&mut self, 
                                                       id: EntityID,
                                                       x: &T) -> Result<()> {
        let info = x.get();
        self.storage.add_chest(id, &info.capacity);
        for n in info.contains.iter() {
            self.storage.try_into_chest(&id, &self.entities.id(n)?)?;
        }
        return Ok(());
    }

    fn add_comp_storage_facet<T: HasStorageFacet>(&mut self,
                                                       id: EntityID,
                                                       x: &T) -> Result<()> {
        self.storage.add_facet(&id);
        let info = x.get();
        for (n, rs) in info.contains.iter() {
            let dir = rs.clone(); 
            self.storage.into_facet(&id, &dir, &self.entities.id(n)?)?;
        }
        return Ok(());
    }
    fn add_comp_storage_character<T: HasStorageCharacter>(&mut self,
                                                               id: EntityID,
                                                               _: &T) -> Result<()> {
        self.storage.add_character(id);
        return Ok(());
    }

    fn add_comp_portal<T: HasPortal>(&mut self, id: EntityID, x: &T) -> Result<()> {
        let info = x.get();
        let  to = &self.entities.id(&info.to)?;
        self.portal.add(id, *to);
        return Ok(());
    }

    fn add_comp_spawner<T: HasSpawner>(&mut self, id: EntityID, x: &T) -> Result<()> {
        let info = x.get(); 
        return Ok(());
    }


    fn add_room(&mut self, r: &map::Room) -> Result<()> {
        let id = self.entities.id(&r.name)?;
        self.add_comp_render(id, r)?;
        self.add_comp_storage_chest(id, r)?;
        self.add_comp_storage_facet(id, r)?;
        self.add_comp_storage_character(id, r)?;
        return Ok(());
    }
    
    fn add_container(&mut self, c: &map::Container) -> Result<()> {
        let id = self.entities.id(&c.name)?;
        self.add_comp_render(id, c)?;
        self.add_comp_storage_chest(id, c)?;
        self.add_comp_movable(id, c)?;
        return Ok(());
    }
    
    fn add_object(&mut self, o: &map::Object) -> Result<()> {
        let id = self.entities.id(&o.name)?;
        self.add_comp_render(id, o)?;
        self.add_comp_movable(id, o)?;
        return Ok(());
    }

    fn add_portal(&mut self, p: &map::Portal) -> Result<()> {
        let id = self.entities.id(&p.name)?;
        self.add_comp_render(id, p)?;
        self.add_comp_portal(id, p)?;
        return Ok(());
    }
    
    fn add_spawner(&mut self, s: &map::Spawner) -> Result<()> {
        let id = self.entities.id(&s.name)?;
        self.add_comp_render(id, s)?;
        self.add_comp_spawner(id, s)?;
        return Ok(());
    }
    
    pub fn load(&mut self, m: &map::Map) -> Result<()> {
        
        self.game_name = m.name.clone();

        // step 1: give everything ids
        for x in &m.rooms { self.entities.add(&x.name); }
        for x in &m.containers { self.entities.add(&x.name); }
        for x in &m.objects { self.entities.add(&x.name); }
        for x in &m.portals { self. entities.add(&x.name); }
        for x in &m.spawners { self.entities.add(&x.name); }

        // step 2: now add the entities (which may have references to each other)
        for x in &m.rooms { self.add_room(x)?; }
        for x in &m.containers { self.add_container(x); }
        for x in &m.objects { self.add_object(x); }
        for x in &m.portals { self.add_portal(x); }
        for x in &m.spawners { self.add_spawner(x); }

        return Ok(());
    }
}
