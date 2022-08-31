use std::ops::Range;
use crate::{model::pong::PlayArea, view::gameview::GameView};

pub fn run() {
    unsafe {
        default().run();
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

pub unsafe fn default() -> Controller {
    Controller { 
        play_area: PlayArea::default(), 
        game_view: GameView::default() 
    }
}
pub struct Controller{
    play_area: PlayArea,
    game_view: GameView,
}

impl Controller {

    pub unsafe fn run(&mut self) {
        self.game_view.init();
        self.game_view.open_window();

        loop {
            self.game_view.render();
        }

        self.game_view.close_window();
        self.game_view.quit();
    } 
}