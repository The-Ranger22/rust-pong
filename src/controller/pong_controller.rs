use std::{thread::{JoinHandle}, sync::{Arc, Mutex}};

use crate::{model::pong::{PlayArea, game_objects::{objects::{GameObject, ObjectType}, ObjectDimensions, Position, behaviors::{ObjectBehavior, ObjectInteractBehaviors, ObjectInteractBehavior, ObjectMovementBehavior, ObjectMovementBehaviors}, GameObjectFactory}, vectors::EuclideanVector}, view::{gameview::GameView, assets::{Drawable, rectangle::Rectangle, drawable::{Point, Dimensions}, color::Colors, line::Line}}};

pub fn run(width: usize, height: usize) {
    unsafe {
        default(width, height).run();
    }
}

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
        Point::from_tuple(convert_usize_tuple_to_i32_tuple(obj.pos.as_discrete_tuple())),
        object_dimensions_to_dimensions(&obj.dim),
        color,
        priority
    ))
}

fn create_vector_drawing(id: usize, obj: &GameObject) -> Box<dyn Drawable> {
    Box::new(Line::new(
        id,
        vec![
            Point::from_tuple(convert_usize_tuple_to_i32_tuple(obj.pos.as_discrete_tuple())),
            Point::from_tuple(convert_usize_tuple_to_i32_tuple(obj.next_pos().as_discrete_tuple())),
        ],
        Colors::BLUE.as_rgb(),
        0
    ))
}

fn add_walls(play_area: &mut PlayArea, factory: &mut GameObjectFactory, wall_thickness: usize) {
    let wall_specs = [
        // (x, y, w, h, angle)
        (0, 0, play_area.get_width(), wall_thickness, 90.0),
        (0, play_area.get_height() - wall_thickness, play_area.get_width(), wall_thickness, 270.0),
        (0, 0, wall_thickness, play_area.get_height(), 180.0),
        (play_area.get_width() - wall_thickness, 0, wall_thickness, play_area.get_height(), 0.0)
    ];

    for (x, y, w, h, angle) in wall_specs {
        let mut wall = factory.create(ObjectType::WALL, x, y, w, h);
        wall.vec.set_angle(angle);
        play_area.add_game_object(wall)
    }
}

fn add_paddles(play_area: &mut PlayArea, factory: &mut GameObjectFactory, paddle_dims: (usize, usize), wall_thickness: usize, gap: usize) {
    let left_paddle = factory.create(ObjectType::PADDLE, wall_thickness + gap, wall_thickness + gap, paddle_dims.0, paddle_dims.1);
    let right_paddle = factory.create(ObjectType::PADDLE, play_area.get_width() - (wall_thickness + paddle_dims.0 + gap), wall_thickness + gap, paddle_dims.0, paddle_dims.1);
    play_area.add_game_object(left_paddle);
    play_area.add_game_object(right_paddle);
}

fn add_ball(play_area: &mut PlayArea, factory: &mut GameObjectFactory, ball_width: usize) {
    let mut ball = factory.create(ObjectType::BALL, play_area.get_width()/2 - ball_width/2, play_area.get_height()/2 - ball_width/2, ball_width, ball_width);
    play_area.add_game_object(ball);
}

fn add_goals(play_area: &mut PlayArea, factory: &mut GameObjectFactory, goal_width: usize, wall_thickness: usize) {
    let left_goal = factory.create(
        ObjectType::GOAL, 
        wall_thickness, 
        wall_thickness, 
        goal_width, 
        play_area.get_height() - 2*wall_thickness
    );

    let right_goal = factory.create(
        ObjectType::GOAL, 
        play_area.get_width() - (wall_thickness + goal_width), 
        wall_thickness, 
        goal_width, 
        play_area.get_height() - 2*wall_thickness
    );

    play_area.add_game_object(left_goal);
    play_area.add_game_object(right_goal);
}



fn init_playarea(width: usize, height: usize) -> PlayArea {
    let mut factory = GameObjectFactory::new();
    let mut play_area =  PlayArea::new(width, height);
    let wall_thickness: usize = 10;
    let ball_width = 20;
    let paddle_dims: (usize, usize) = (10, 50);
    let goal_width: usize = 30;
    let pixel_gap_between_paddle_and_wall = 30;
    let area_dims = (width, height);
    // walls
    add_walls(&mut play_area, &mut factory, wall_thickness);
    // adding the paddles
    add_paddles(&mut play_area, &mut factory, paddle_dims, wall_thickness, pixel_gap_between_paddle_and_wall);
    // score zones
    //add_goals(&mut play_area, &mut factory, goal_width, wall_thickness);
    // adding the ball
    add_ball(&mut play_area, &mut factory, ball_width);
    play_area
}


unsafe fn init_gameview(width: usize, height: usize) -> GameView {
    let mut gameview = GameView::sdl2(width as i32, height as i32);
    gameview.init();
    gameview.open_window();
    gameview
}


pub unsafe fn default(width: usize, height: usize) -> Controller {
    Controller { 
        play_area: init_playarea(width, height), 
        game_view: init_gameview(width, height),
        keep_playing: true
    }
}
pub struct Controller{
    play_area: PlayArea,
    game_view: GameView,
    keep_playing: bool
}



impl Controller {
    fn add_game_object_drawings_to_renderer(&mut self) {
        for i in 0..self.play_area.game_objects.len() {
            self.game_view.add_drawable_object(
                convert_game_object_to_drawing(i, &self.play_area.game_objects[i])
            );
        }
    }
}

impl Controller {
    unsafe fn render(&mut self) {
        self.add_game_object_drawings_to_renderer();
        self.game_view.render();
        // let gv = Arc::clone(&self.game_view);
        // self.add_handle(thread::spawn(move || {
        //     gv.lock().unwrap().render()
        // }));
    }

    fn resolve(&mut self) {
        self.play_area.resolve_object_behaviors();
    }
}

impl Controller {
    unsafe fn handle_input(&mut self) {
        if let Some(keycode) = self.game_view.keyboard_input() {
            match keycode {
                27 => {self.keep_playing = false},
                _ => ()
            }
        }
    }
}

impl Controller {
    unsafe fn run(&mut self) {
        while self.keep_playing {
            // TODO: switch to an interleave method -> update & render object before moving to the next 
            self.add_game_object_drawings_to_renderer();
            self.render();
            self.resolve();
            self.handle_input();
        }
        self.game_view.close_window();
        self.game_view.quit();
    } 
}