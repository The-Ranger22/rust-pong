use std::{thread::{JoinHandle, self}, sync::{Arc, Mutex, atomic::{AtomicBool, Ordering, AtomicI32}}};

use crate::{model::pong::{PlayArea, game_objects::{objects::{GameObject, ObjectType}, ObjectDimensions, GameObjectFactory, Position}, vectors::EuclideanVector}, view::{gameview::{GameView}, assets::{Drawable, rectangle::Rectangle, drawable::{Point, Dimensions}, color::Colors, line::Line}}};

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

fn paddle_interact_behavior(paddle: &mut GameObject, other: &GameObject) {
    match other.object_type {
        ObjectType::WALL => paddle.kill_velocity(),
        _ => ()
    }
}

fn add_paddles(play_area: &mut PlayArea, factory: &mut GameObjectFactory, paddle_dims: (usize, usize), wall_thickness: usize, gap: usize) {
    let mut left_paddle = factory.create(ObjectType::PADDLE, wall_thickness + gap, wall_thickness + gap, paddle_dims.0, paddle_dims.1);
    let right_paddle = factory.create(ObjectType::PADDLE, play_area.get_width() - (wall_thickness + paddle_dims.0 + gap), wall_thickness + gap, paddle_dims.0, paddle_dims.1);
    
    left_paddle.behavior.set_interact_behavior(paddle_interact_behavior);

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
    add_goals(&mut play_area, &mut factory, goal_width, wall_thickness);
    // adding the ball
    add_ball(&mut play_area, &mut factory, ball_width);
    play_area
}

unsafe fn init_gameview(width: usize, height: usize) -> GameView {
    let mut gameview = GameView::sdl2(width as i32, height as i32);
    gameview.init();
    gameview
}

pub unsafe fn default(width: usize, height: usize) -> Controller {
    Controller { 
        play_area: init_playarea(width, height),
        keep_playing: Arc::new(AtomicBool::new(true)),
        threads: Vec::new(),
        renderer_started: Arc::new(AtomicBool::new(false)),
        keyboard_input: Arc::new(AtomicI32::new(-1)),
        ready_to_render: Arc::new(AtomicBool::new(false)),
        objects_to_render: Arc::new(Mutex::new(Vec::new())),
        plyr_momentum: 0.0,
        comp_momentum: 0.0,
        plyr_score: 0,
        comp_score: 0
    }
}

pub struct Controller{
    play_area: PlayArea,
    keep_playing: Arc<AtomicBool>,
    threads: Vec<JoinHandle<()>>,
    renderer_started: Arc<AtomicBool>,
    keyboard_input: Arc<AtomicI32>,
    ready_to_render: Arc<AtomicBool>,
    objects_to_render: Arc<Mutex<Vec<GameObject>>>,
    plyr_momentum: f64,
    comp_momentum: f64,
    plyr_score: u8,
    comp_score: u8
}

impl Controller {
    fn wait_on_all_threads(&mut self) {
        while let Some(thread_handle) = self.threads.pop() {
            thread_handle.join().unwrap()
        }
    }
}

impl Controller {
    unsafe fn render(&mut self) {
        let (width, height) = self.play_area.dims_as_tuple();
        
        let objects_to_render = Arc::clone(&self.objects_to_render);
        let ready_to_render = Arc::clone(&self.ready_to_render);

        let keep_playing = Arc::clone(&self.keep_playing);
        let renderer_started = Arc::clone(&self.renderer_started);
        let keyboard_input = Arc::clone(&self.keyboard_input);
        self.threads.push(thread::spawn(move || {
            //println!("Starting render thread");
            let mut game_view = init_gameview(width, height);
            renderer_started.store(true, Ordering::Release);
            while keep_playing.load(Ordering::Acquire) {
                while !ready_to_render.load(Ordering::Acquire) {}
                let mut id = 0;
                while let Some(mut obj) = objects_to_render.lock().unwrap().pop() {
                    game_view.add_drawable_object(convert_game_object_to_drawing(id, &mut obj));
                    id += 1;
                }
                //println!("Rendering!");
                game_view.render();
                if let Some(keypress) = game_view.keyboard_input() {
                    match keypress {
                        _ => {keyboard_input.store(keypress, Ordering::Release)}
                    }
                }
                ready_to_render.store(false, Ordering::Release);
            }
            game_view.close_window();
            game_view.quit();
        }));
    }

    fn resolve_model(&mut self) {
        let mut i = 0;
        while self.ready_to_render.load(Ordering::Acquire) {
            i += 1;
        }
        for object in self.play_area.game_objects.iter() {
            self.objects_to_render.lock().unwrap().push(object.clone())
        }
        self.ready_to_render.store(true, Ordering::Release);
        self.play_area.resolve_object_behaviors()
    }
}

impl Controller {
    const MOMENTUM_UPPER: f64 = 2.5;
    const MOMENTUM_LOWER: f64 = -2.5;
    const INCREMENT: f64 = 0.5;
    const RESTING: f64 = 0.0;

    fn move_up(paddle: &mut GameObject, momentum: &mut f64) {
        if *momentum > Self::MOMENTUM_LOWER {
            *momentum -= Self::INCREMENT;
        }
        Self::resolve_move(paddle, momentum)
    }

    fn move_down(paddle: &mut GameObject, momentum: &mut f64) {
        if *momentum < Self::MOMENTUM_UPPER {
            *momentum += Self::INCREMENT;
        }
        Self::resolve_move(paddle, momentum);
    }

    fn decay(momentum: &mut f64) {
        if *momentum > Self::RESTING {
            *momentum -= 0.1;
        } else if *momentum < Self::RESTING {
            *momentum += 0.1;
        }
    }

    fn momentum_decay(&mut self) {
        Self::decay(&mut self.plyr_momentum);
        Self::resolve_move(&mut self.play_area.game_objects[4], &mut self.plyr_momentum)
    }

    fn resolve_move(paddle: &mut GameObject, momentum: &mut f64) {
        paddle.pos.inc_y_pos(*momentum)
    }
}

impl Controller {
    unsafe fn handle_input(&mut self) {
        match self.keyboard_input.load(Ordering::Acquire) {
            27 => {self.keep_playing.store(false, Ordering::Release)},
            119 => Self::move_up(&mut self.play_area.game_objects[4], &mut self.plyr_momentum),
            115 => Self::move_down(&mut self.play_area.game_objects[4], &mut self.plyr_momentum),
            _ => self.momentum_decay()
        }
    }
}

impl Controller {
    fn resolve_computer_turn(&mut self) {
        let (ball_y_origin, ball_y_extent) = (
            self.play_area.game_objects[8].pos.get_y_pos(), 
            self.play_area.game_objects[8].y_extent()
        );

        let ball_y_center = ball_y_origin + (ball_y_extent - ball_y_origin).abs();

        let paddle = &mut self.play_area.game_objects[5];
        let paddle_y_center = paddle.pos.get_y_pos() + (paddle.y_extent() - paddle.pos.get_y_pos()).abs();

        if paddle_y_center > ball_y_center {
            Self::move_up(paddle, &mut self.comp_momentum)
        } else if paddle_y_center < ball_y_center {
            Self::move_down(paddle, &mut self.comp_momentum)
        }



    }
}

impl Controller {
    const WIN_SCORE: u8 = 3;

    fn reset(&mut self) {
        println!("{} | {}", self.plyr_score, self.comp_score);
        self.plyr_momentum = 0.0;
        self.comp_momentum = 0.0;
        let w_h = self.play_area.dims_as_tuple();
        let ball = &mut self.play_area.game_objects[8];
        let ball_width = ball.dim.get_width();
        ball.pos = Position::new(w_h.0/2 - (ball_width/2.0) as usize, w_h.1/2 - (ball_width/2.0) as usize);
        ball.vec = EuclideanVector::new(1.0, if rand::random() {-135.0} else {45.0})
    }

    fn check_if_score(&mut self) {
        let left_score_zone = self.play_area.game_objects[6];
        let right_score_zone = self.play_area.game_objects[7];
        let ball = &mut self.play_area.game_objects[8];
        if ball.intersecting(&left_score_zone) {
            self.comp_score += 1;
            self.reset();
        } else if ball.intersecting(&right_score_zone) {
            self.plyr_score += 1;
            self.reset();
        }
    }

    fn check_win_condition(&mut self) {
        if self.plyr_score == Self::WIN_SCORE || self.comp_score == Self::WIN_SCORE {
            self.keep_playing.store(false, Ordering::Release);
        } 
    }
}

impl Controller {
    unsafe fn run(&mut self) {
        self.render();
        while self.keep_playing.load(Ordering::Acquire) {
            self.handle_input();
            self.resolve_computer_turn();
            self.resolve_model();
            self.check_if_score();
            self.check_win_condition();
        }
        println!("Waiting on threads...");
        self.wait_on_all_threads();
    } 
}