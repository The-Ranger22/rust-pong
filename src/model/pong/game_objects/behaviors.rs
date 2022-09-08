use super::{objects::GameObject, Position};

#[derive(Clone, Copy)]
pub struct ObjectBehavior {
    interact_behavior: ObjectInteractBehavior,
    movement_behavior: ObjectMovementBehavior
}

impl ObjectBehavior {
    pub fn new(interact_behavior: ObjectInteractBehavior, movement_behavior: ObjectMovementBehavior) -> Self {
        Self {interact_behavior, movement_behavior}
    }
}

impl ObjectBehavior {
    pub fn interact(&self, the_changed: &mut GameObject, the_changer: &GameObject){
        self.interact_behavior.call(the_changed, the_changer)
    } 

    pub fn movement(&self, game_object: &GameObject) -> Position {
        self.movement_behavior.call(game_object)
    }
}

pub enum ObjectInteractBehaviors {
    NOTHING,
    LOSSLESS_COLLISION
}

impl ObjectInteractBehaviors {
    pub fn nothing(the_changed: &mut GameObject, the_changer: &GameObject) {}
    // collision behavior
    pub fn lossless_collision(the_changed: &mut GameObject, the_changer: &GameObject) {
        the_changed.vec.collide_with(the_changer.vec)
    }
    
}
#[derive(Clone, Copy)]
pub struct ObjectInteractBehavior {
    behavior: fn(&mut GameObject, &GameObject)
}

impl ObjectInteractBehavior {
    pub fn create(interact_behavior: ObjectInteractBehaviors) -> ObjectInteractBehavior {
        ObjectInteractBehavior {
            behavior: match interact_behavior {
                    ObjectInteractBehaviors::NOTHING => ObjectInteractBehaviors::nothing,
                    ObjectInteractBehaviors::LOSSLESS_COLLISION => ObjectInteractBehaviors::lossless_collision,
            }
        }
    }

    fn call(&self, the_changed: &mut GameObject, the_changer: &GameObject) {
        (self.behavior)(the_changed, the_changer)
    }
}

pub enum ObjectMovementBehaviors {
    STATIONARY,
    MOVING
}

impl ObjectMovementBehaviors {
    pub fn stationary(game_object: &GameObject) -> Position {game_object.pos}

    pub fn moving(game_object: &GameObject) -> Position {
        // update object position based off of vector
        let x_change = game_object.vec.x_component();
        let y_change = game_object.vec.y_component();

        let mut new_pos = game_object.pos;
        new_pos.inc_x_pos(x_change);
        new_pos.inc_y_pos(y_change);
        
        new_pos
    }
}

#[derive(Clone, Copy)]
pub struct ObjectMovementBehavior {
    behavior: fn(&GameObject) -> Position
}

impl ObjectMovementBehavior {
    pub fn create(movement_behavior: ObjectMovementBehaviors) -> ObjectMovementBehavior {
        match movement_behavior {
            ObjectMovementBehaviors::STATIONARY => ObjectMovementBehavior::new(ObjectMovementBehaviors::stationary),
            ObjectMovementBehaviors::MOVING     => ObjectMovementBehavior::new(ObjectMovementBehaviors::moving),
        }
    }

    pub fn new(f: fn(&GameObject) -> Position) -> Self {
        Self {behavior: f}
    }
}

impl ObjectMovementBehavior {
    fn call(&self, game_object: &GameObject) -> Position {
        (self.behavior)(game_object)
    }
}
