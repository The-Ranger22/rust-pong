use std::fmt::Display;
use std::hash::{Hash};
use std::ops::Range;

use crate::{ model::pong::{vectors::EuclideanVector}};

use super::{Position, behaviors::ObjectBehavior, ObjectDimensions};

#[derive(Clone, Copy, Debug, Hash)]
pub enum ObjectType {
    BALL,
    PADDLE,
    WALL,
    GOAL,
    CUSTOM
}

impl PartialEq for ObjectType {
    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(&other.to_string())
    }
}

impl Eq for ObjectType {}

impl Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectType::BALL    => write!(f, "BALL"     ),
            ObjectType::PADDLE  => write!(f, "PADDLE"   ),
            ObjectType::WALL    => write!(f, "WALL"     ),
            ObjectType::GOAL    => write!(f, "GOAL"     ),
            ObjectType::CUSTOM  => write!(f, "CUSTOM"   ),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ObjectId {
    ID(ObjectType, u32)
}

impl Display for ObjectId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectId::ID(s, i) => write!(f, "{s}_{i}"),
        }
    }
}

/* 
    Traits:
        Has an object type
        Has a position
        has a vector
        has dimensions
        has a behavior
    Behavior:
        Checks if touching another game object
        Updates its position based on its vector
        Iteracts with other game objects

        
 */

 // TODO: Switch behavior into a Vector of behaviors so that I can add and remove behaviors to an object on the fly
#[derive(Clone, Copy)]
 pub struct GameObject{
    id: ObjectId,
    pub object_type: ObjectType,
    pub pos: Position,
    last_pos: Position,
    pub dim: ObjectDimensions,
    pub vec: EuclideanVector,
    pub behavior: ObjectBehavior
}

impl GameObject {
    pub fn new(id: ObjectId, object_type: ObjectType, pos: Position, dim: ObjectDimensions, vec: EuclideanVector, behavior: ObjectBehavior) -> Self {
        Self {id, object_type, pos, last_pos: pos, dim, vec, behavior}
    }
}

impl PartialEq for GameObject {
    fn eq(&self, other: &Self) -> bool {
        self.id.to_string().eq(&other.id.to_string())
    }
}

impl Eq for GameObject {}

impl GameObject {
    pub fn x_extent(&self) -> f64 {
        self.pos.get_x_pos() + self.dim.get_width()
    }

    pub fn y_extent(&self) -> f64 {
        self.pos.get_y_pos() + self.dim.get_height()
    }

    pub fn midpoint(&self) -> Position {
        Position::new(self.pos.x_pos + self.x_extent()/2.0, self.pos.y_pos + self.y_extent()/2.0)
    }

    pub fn dim_boundaries(&self) -> (Range<f64>, Range<f64>) {
        (self.pos.get_x_pos()..self.x_extent(), self.pos.get_y_pos()..self.y_extent())
    }
}

impl GameObject {
    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    pub fn kill_velocity(&mut self) {
        self.vec.set_angle(0.0);
        self.vec.set_magnitude(0.0);
    }
    

    pub fn next_pos(&self) -> Position {
        let behavior = self.behavior;
        behavior.movement(self)
    }

    pub fn update_pos(&mut self, new_pos: Position) {
        self.last_pos = self.pos;
        self.pos = new_pos
    }

    pub fn fetch_last_position(&self) -> Position {
        self.last_pos
    }

    pub fn interact_with(&mut self, other: &Self) {
        let behavior = self.behavior;
        behavior.interact(self, other)
    }

    /* 
        Objects have space that they occupy that are defined by their position and dimensions (width, height)
        Four points can be extracted [Format used -> [POINT-DESIG]: (x-coord, y-coord)]:
            TOP-LEFT    : (self.pos.x_pos                   , self.pos.y_pos                  )
            TOP-RIGHT   : (self.pos.x_pos + self.dim.width  , self.pos.y_pos                  )
            BOTTOM-LEFT : (self.pos.x_pos                   , self.pos.y_pos + self.dim.height)
            BOTTOM-RIGHT: (self.pos.x_pos + self.dim.width  , self.pos.y_pos + self.dim.height)
        Four types of adjacencies: 
            Above
            Below
            Left of
            Right of
    */

    pub fn intersecting(&self, other: &Self) -> bool {
        let (other_x_range, other_y_range) = other.dim_boundaries();
        (other_x_range.contains(&self.pos.get_x_pos()) || other_x_range.contains(&self.x_extent()))
        && 
        (other_y_range.contains(&self.pos.get_y_pos()) || other_y_range.contains(&self.y_extent()))
    }
    
    

    
}




