use crate::{view::assets::{drawable::{Origin, Dimensions}, rectangle, color}, model::pong::PlayArea as PlayArea};
use crate::view::gameview as GV;
use crate::controller::pong_controller;

pub mod controller;
pub mod view;
pub mod model;

use fermium::{
    prelude::*,
};

use ndarray::arr1;
fn main() {
    pong_controller::run();
}
