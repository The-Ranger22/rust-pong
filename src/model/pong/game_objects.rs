use std::{fmt::Display, collections::HashMap, hash::Hash, ops};

use num::{Num, PrimInt, FromPrimitive, ToPrimitive};

use self::{behaviors::{ObjectBehavior, ObjectInteractBehavior, ObjectMovementBehavior, ObjectMovementBehaviors, ObjectInteractBehaviors}, objects::{GameObject, ObjectType, ObjectId}};
use super::{PlayArea, vectors::EuclideanVector};

pub mod traits;
pub mod objects;
pub mod behaviors;

#[derive(Clone, Copy, Debug)]
pub struct Position {
    x_pos: f64,
    y_pos: f64
}

impl Position {
    pub fn new(x_pos: impl ToPrimitive, y_pos: impl ToPrimitive) -> Self {
        Self { x_pos: x_pos.to_f64().unwrap(), y_pos: y_pos.to_f64().unwrap()}
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x_pos, self.y_pos)
    }
}

impl Position {
    pub fn get_x_pos(&self) -> f64 {self.x_pos}
    pub fn get_y_pos(&self) -> f64 {self.y_pos}

    pub fn inc_x_pos(&mut self, change: f64) {
        self.x_pos += change;
    }

    pub fn inc_y_pos(&mut self, change: f64) {
        self.y_pos += change
    }

    pub fn as_tuple(&self) -> (f64, f64) {
        (self.x_pos, self.y_pos)
    }

    pub fn as_discrete_tuple(&self) -> (usize, usize) {
        (self.x_pos as usize, self.y_pos as usize)
    }
}



#[derive(Clone, Copy, Debug)]
pub struct ObjectDimensions {
    width: f64,
    height: f64
}

impl ObjectDimensions {
    pub fn new(width: usize, height: usize) -> Self {
        Self {width: width as f64, height: height as f64}
    }

    pub fn get_width(&self) -> f64 {
        self.width 
    }

    pub fn get_height(&self) -> f64 {
        self.height
    }

}
#[derive(Debug)]
pub struct GameObjectFactory {
    instance_map: HashMap<ObjectType, u32>
}



impl GameObjectFactory {
    fn prepare_instance_map() -> HashMap<ObjectType, u32> {
        let mut instance_map: HashMap<ObjectType, u32> = HashMap::new();
        instance_map.insert(ObjectType::BALL, 0);
        instance_map.insert(ObjectType::PADDLE, 0);
        instance_map.insert(ObjectType::WALL, 0);
        instance_map.insert(ObjectType::GOAL, 0);
        instance_map.insert(ObjectType::CUSTOM, 0);
        instance_map
    }

    pub fn new() -> Self {
        Self {instance_map: Self::prepare_instance_map()}
    }
}

impl GameObjectFactory {
    fn fetch_vector(obj_type: ObjectType) -> EuclideanVector {
        match obj_type {
            ObjectType::BALL    => EuclideanVector::new(1.0, 45.0),
            ObjectType::PADDLE  => EuclideanVector::new(1.0, 0.0),
            ObjectType::WALL    => EuclideanVector::new(1.0, 45.0),
            ObjectType::GOAL    => EuclideanVector::new(0.0, 0.0),
            ObjectType::CUSTOM  => EuclideanVector::new(0.0, 0.0),
        }
    }    
    fn fetch_behavior(obj_type: ObjectType) -> ObjectBehavior {
        match obj_type {
            ObjectType::BALL    => ObjectBehavior::new( 
                ObjectInteractBehavior::create(ObjectInteractBehaviors::LOSSLESS_COLLISION),
                ObjectMovementBehavior::create(ObjectMovementBehaviors::MOVING)
            ),
            ObjectType::PADDLE  => ObjectBehavior::new( 
                ObjectInteractBehavior::create(ObjectInteractBehaviors::LOSSLESS_COLLISION),
                ObjectMovementBehavior::create(ObjectMovementBehaviors::STATIONARY)
            ),
            ObjectType::WALL    => ObjectBehavior::new( 
                ObjectInteractBehavior::create(ObjectInteractBehaviors::NOTHING),
                ObjectMovementBehavior::create(ObjectMovementBehaviors::STATIONARY)
            ),
            ObjectType::GOAL    => ObjectBehavior::new( 
                ObjectInteractBehavior::create(ObjectInteractBehaviors::NOTHING),
                ObjectMovementBehavior::create(ObjectMovementBehaviors::STATIONARY)
            ),
            ObjectType::CUSTOM  => ObjectBehavior::new( 
                ObjectInteractBehavior::create(ObjectInteractBehaviors::NOTHING),
                ObjectMovementBehavior::create(ObjectMovementBehaviors::STATIONARY)
            ),
        }
    }

    

    fn get_instance_count_and_increment(&mut self, obj_type: &ObjectType) -> u32 {
        self.instance_map.insert(*obj_type, *self.instance_map.get(obj_type).unwrap() + 1).unwrap()
    }

    fn fetch_object_id(&mut self, obj_type: &ObjectType) -> ObjectId {
        ObjectId::ID(*obj_type, self.get_instance_count_and_increment(&obj_type))
    }

    pub fn create(&mut self, obj_type: ObjectType, x: usize, y: usize, w: usize, h: usize) -> GameObject {
        let id = self.fetch_object_id(&obj_type);
        let pos = Position::new(x, y);
        let dims = ObjectDimensions::new(w, h);
        GameObject::new(
            id,
            obj_type, 
            pos, 
            dims, 
            Self::fetch_vector(obj_type), 
            Self::fetch_behavior(obj_type)
        )
    }
}



