use std::{ops::Range, collections::HashMap, sync::{Arc, Mutex}};
use game_objects::Position;

use crate::model::pong::game_objects::objects::ObjectType;

use self::game_objects::objects::GameObject;


pub mod vectors;
pub mod game_objects;
pub const AREA_WIDTH: usize = 90;
pub const AREA_HEIGHT: usize = 50;

pub struct Bounds {
    w: usize,
    h: usize,
    w_range: Range<usize>,
    h_range: Range<usize>
}

impl Bounds {
    pub fn new(w: usize, h: usize) -> Self {
        Self {w, h, w_range: 0..w, h_range: 0..h}
    }
}

impl Bounds {
    fn contains_x(&self, x: usize) -> bool {
        self.w_range.contains(&x)
    }
    fn contains_y(&self, y: usize) -> bool {
        self.h_range.contains(&y)
    }
    fn contains(&self, pos: &Position) -> bool {
        self.contains_x(pos.get_x_pos() as usize) && self.contains_y(pos.get_y_pos() as usize)
    }
}

pub struct PlayArea {
    dims : Bounds,
    pub game_objects: Vec<GameObject>
}

impl PlayArea {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            dims: Bounds::new(width, height),
            game_objects: Vec::new()
        }
    }

    pub fn from_bounds(dims: Bounds) -> Self {
        Self {
            dims,
            game_objects: Vec::new()
        }
    }
}

impl Default for PlayArea {
    fn default() -> Self {
        Self::new(AREA_WIDTH, AREA_HEIGHT)
    }
}

impl PlayArea {
    pub fn object_within_bounds(&self, obj: &GameObject) -> bool {
        self.dims.contains(&obj.pos)
    }
}


impl PlayArea {
    const MAX_SPEED: f64 = 5.0;
    const SPEED_INCREMENT: f64 = 0.2;
    pub fn get_width(&self) -> usize {
        self.dims.w
    }

    pub fn get_height(&self) -> usize {
        self.dims.h
    }

    pub fn dims_as_tuple(&self) -> (usize, usize) {
        (self.get_width(), self.get_height())
    }

    pub fn add_game_object(&mut self, obj: GameObject) {
        if self.object_within_bounds(&obj) {
            self.game_objects.push(obj);
        } else {
            panic!("Failed to add object to PlayArea: Object is out of bounds!")
        }
    }

    fn resolve_speed_increase(obj: &mut GameObject) {
        if obj.vec.get_magnitude() > Self::MAX_SPEED {
            obj.vec.set_magnitude(Self::MAX_SPEED);
        } else {
            obj.vec.set_magnitude(obj.vec.get_magnitude() + Self::SPEED_INCREMENT)
        }
    }

    fn resolve_movement(obj: &mut GameObject) {
        let next_pos = obj.next_pos();
        obj.update_pos(next_pos);
    }

    fn check_if_adjacent(obj: &GameObject, other: &GameObject) -> bool {
        obj.intersecting(other)
    } 

    fn resolve_interact(obj: &mut GameObject, other: &GameObject) {
        if obj.object_type == ObjectType::BALL {
            if Self::check_if_adjacent(obj, other) { 
                println!("{} interacting with {}: ", obj.get_id(), other.get_id());
                obj.interact_with(other);
                Self::resolve_speed_increase(obj);
            }
        }  
    }

    pub fn resolve_object_behaviors(&mut self) {
        
        let number_objects = self.game_objects.len();
        for i in 0..number_objects {
            for j in 0..number_objects {
                if i != j {
                    let other = self.game_objects[j];
                    Self::resolve_interact(&mut self.game_objects[i], &other);
                }
            }
            Self::resolve_movement(&mut self.game_objects[i])
            
            // resolve interaction
            
            
            // resolve movement
            
        }
    }
}




