use std::ops::Range;
use crate::{model::pong::{PlayArea, game_objects::{objects::{GameObject, ObjectType}, ObjectDimensions, Position, behaviors::{ObjectBehavior, ObjectInteractBehaviors, ObjectInteractBehavior, ObjectMovementBehavior, ObjectMovementBehaviors}, GameObjectFactory}, vectors::EuclideanVector}, view::{gameview::GameView, assets::{Drawable, rectangle::Rectangle, drawable::{Point, Dimensions}, color::Colors, line::Line}}};

pub fn run(width: usize, height: usize) {
    unsafe {
        default(width, height).run();
    }
}

fn map_into_range(value: usize, src_range: &Range<usize>, target_range: &Range<usize>) -> u32 {
    if !src_range.contains(&value) {
        panic!();
    }
    (
        (value as f64/src_range.len() as f64)*(target_range.len() as f64) + target_range.start as f64
    ) as u32
}


pub unsafe fn default(width: usize, height: usize) -> Controller {
    Controller { 
        play_area: PlayArea::new(width, height), 
        game_view: GameView::default() 
    }
}
pub struct Controller{
    play_area: PlayArea,
    game_view: GameView,
}

impl Controller {

    fn object_dimensions_to_dimensions(dims: &ObjectDimensions) -> Dimensions {
        Dimensions::new(dims.get_width() as i32, dims.get_height() as i32)
    }

    fn convert_usize_tuple_to_i32_tuple(tup: (usize, usize)) -> (i32, i32) {
        (tup.0 as i32, tup.1 as i32)
    }

    fn convert_game_object_to_drawing(id: usize, obj: &GameObject) -> Box<dyn Drawable> {
        let (color, priority) = match obj.object_type {
            ObjectType::BALL    => (Colors::YELLOW.as_rgb()     , 1),
            ObjectType::PADDLE  => (Colors::WHITE.as_rgb()      , 1),
            ObjectType::WALL    => (Colors::GREY.as_rgb()       , 3),
            ObjectType::GOAL    => (Colors::BLACK.as_rgb(), 2),
            ObjectType::CUSTOM  => (Colors::RED.as_rgb()        , 1),
        };
        Box::new(Rectangle::new(
            id,
            Point::from_tuple(Self::convert_usize_tuple_to_i32_tuple(obj.pos.as_discrete_tuple())),
            Self::object_dimensions_to_dimensions(&obj.dim),
            color,
            priority
        ))
    }

    fn create_vector_drawing(id: usize, obj: &GameObject) -> Box<dyn Drawable> {
        Box::new(Line::new(
            id,
            vec![
                Point::from_tuple(Self::convert_usize_tuple_to_i32_tuple(obj.pos.as_discrete_tuple())),
                Point::from_tuple(Self::convert_usize_tuple_to_i32_tuple(obj.next_pos().as_discrete_tuple())),
            ],
            Colors::BLUE.as_rgb(),
            0
        ))
    }

    fn add_game_object_drawings_to_renderer(&mut self) {
        for i in 0..self.play_area.game_objects.len() {
            self.game_view.add_drawable_object(
                Self::convert_game_object_to_drawing(i, &self.play_area.game_objects[i])
            );
        }
    }
}

impl Controller {
    unsafe fn run(&mut self) {
        let mut factory = GameObjectFactory::new();
        
        self.game_view.init();
        self.game_view.open_window();

        let wall_thickness: usize = 10;
        let ball_width = 20;
        let paddle_dims: (usize, usize) = (10, 50);
        let goal_width: usize = 30;
        let area_dims = self.game_view.get_screen_dimensions();
        let area_dims = (area_dims.0 as usize, area_dims.1 as usize);

        // walls

        let mut north_wall = factory.create(ObjectType::WALL, 0, 0, area_dims.0 , wall_thickness);
        let mut south_wall = factory.create(ObjectType::WALL, 0, area_dims.1 - wall_thickness, area_dims.0 , wall_thickness);
        let mut west_wall = factory.create(ObjectType::WALL, 0, 0, wall_thickness, area_dims.1);
        let mut east_wall = factory.create(ObjectType::WALL, area_dims.0 - wall_thickness, 0, wall_thickness, area_dims.1);

        north_wall.vec.set_angle(90.0);
        south_wall.vec.set_angle(270.0);
        east_wall.vec.set_angle(180.0);
        west_wall.vec.set_angle(0.0);
        
        self.play_area.add_game_object(north_wall);
        self.play_area.add_game_object(south_wall);
        self.play_area.add_game_object(east_wall);
        self.play_area.add_game_object(west_wall);
        

        // adding the paddles

        let left_paddle = factory.create(ObjectType::PADDLE, wall_thickness + 30, wall_thickness + 30, paddle_dims.0, paddle_dims.1);
        let right_paddle = factory.create(ObjectType::PADDLE, area_dims.0 - (wall_thickness + paddle_dims.0 + 30), wall_thickness + 30, paddle_dims.0, paddle_dims.1);

        self.play_area.add_game_object(left_paddle);
        self.play_area.add_game_object(right_paddle);

        // score zones

        let left_goal = factory.create(ObjectType::GOAL, wall_thickness, wall_thickness, goal_width, area_dims.1 - 2*wall_thickness);
        let right_goal = factory.create(ObjectType::GOAL, area_dims.0 - (wall_thickness + goal_width), wall_thickness, goal_width, area_dims.1 - 2*wall_thickness);


        // self.play_area.add_game_object(left_goal);
        // self.play_area.add_game_object(right_goal);
        // adding the ball
        self.play_area.add_game_object(factory.create(ObjectType::BALL, area_dims.0/2 - ball_width/2, area_dims.1/2 - ball_width/2, ball_width, ball_width));
        let mut i: usize = 0;

        loop {
            // TODO: switch to an interleave method -> update & render object before moving to the next 
            self.add_game_object_drawings_to_renderer();
            self.game_view.render();
            self.play_area.resolve_object_behaviors();
            if i >= 10000 {
                break;
            }
            i += 1;
        }
        self.game_view.close_window();
        self.game_view.quit();
    } 
}