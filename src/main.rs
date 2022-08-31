use crate::controller::pong_controller;

pub mod controller;
pub mod view;
pub mod model;


fn main() {
    pong_controller::run();
}
