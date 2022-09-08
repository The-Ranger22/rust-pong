use fermium::renderer::{SDL_Renderer, SDL_SetRenderDrawColor};

use self::drawable::{HasPoint, HasColor, HasZIndex};

use std::hash::Hash;


pub trait Drawable:  HasPoint + HasColor + HasZIndex {
    fn fetch_id(&self) -> usize;

    unsafe fn draw(&self, renderer: *mut SDL_Renderer) -> bool;

    unsafe fn render_color(&self, renderer: *mut SDL_Renderer) {
        let color = self.fetch_color();
        SDL_SetRenderDrawColor(renderer, color.r, color.g, color.b, color.a);  
    }

    unsafe fn render_with(&self, renderer: *mut SDL_Renderer) -> bool {
        self.render_color(renderer);
        self.draw(renderer)
    }
}

impl PartialEq for dyn Drawable {
    fn eq(&self, other: &Self) -> bool {
        self.fetch_z_index() == other.fetch_z_index()
    }
}

impl Eq for dyn Drawable {
    
}

impl Hash for dyn Drawable {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.fetch_id().hash(state);
        self.fetch_color().hash(state);
        self.fetch_origin().hash(state);
        self.fetch_z_index().hash(state);
    }
}

pub mod drawable {
    use super::color::RGBColor;
    // structs that all drawable objects require
    #[derive(Clone, Copy, Hash)]
    pub struct Point {
        pub x: i32,
        pub y: i32
    }

    impl Point {
        pub fn new(x: i32, y: i32) -> Self {
            Self {x, y}
        }

        pub fn from_tuple(x_y: (i32, i32)) -> Self {
            Self {x: x_y.0, y: x_y.1}
        }

        pub fn update(&mut self, x: i32, y: i32) {
            self.x = x;
            self.y = y;
        }
    }

    #[derive(Clone, Copy)]
    pub struct Dimensions {
        pub w: i32, 
        pub h: i32
    }

    impl Dimensions {
        pub fn new(w: i32, h: i32) -> Self {
            Self {w, h}
        }
    }
    // Traits
    pub trait HasPoint {
        fn fetch_origin(&self) -> Point;
    }

    pub trait HasZIndex {
        fn fetch_z_index(&self) -> i32;
    }
    
    pub trait HasDimensions {
        fn fetch_dimensions(&self) -> Dimensions;
    }

    pub trait HasColor {
        fn fetch_color(&self) -> RGBColor;
    }
}

pub mod color {
    const DEFAULT_ALPHA: u8 = 255;

    fn build_rgb_tuple(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
        (r, g, b)
    }

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> RGBColor {
        RGBColor {r, g, b, a}
    }
    #[derive(Clone, Copy, Hash)]
    pub struct RGBColor {
        pub r: u8,
        pub g: u8,
        pub b: u8,
        pub a: u8
    }

    impl RGBColor {
        pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
            Self {r, g, b, a}
        }

        pub fn as_tuple(&self) -> (u8, u8, u8, u8) {
            (self.r, self.g, self.b, self.a)
        }
    }

    pub enum Colors {
        RED,
        BLUE,
        GREEN,
        MAGENTA,
        BLACK,
        WHITE,
        YELLOW,
        GREY,
        CYAN
    }
    impl Colors {
        pub fn as_rgb(&self) -> RGBColor {
            self.as_rgba(DEFAULT_ALPHA)
        }

        pub fn as_rgba(&self, a: u8) -> RGBColor {
            let (r, g, b) = match self {
                Colors::RED     => build_rgb_tuple(255, 0, 0),
                Colors::GREEN   => build_rgb_tuple(0, 255, 0),
                Colors::BLUE    => build_rgb_tuple(0, 0, 255),
                Colors::MAGENTA => build_rgb_tuple(255, 0, 255),
                Colors::BLACK   => build_rgb_tuple(0, 0, 0),
                Colors::WHITE   => build_rgb_tuple(255, 255, 255),
                Colors::YELLOW  => build_rgb_tuple(255, 255, 0),
                Colors::GREY    => build_rgb_tuple(128,128,128),
                Colors::CYAN    => build_rgb_tuple(0,255,255),
            };
            RGBColor::new(r, g, b, a)
        }
    }
}

pub mod rectangle {
    use fermium::{renderer::SDL_RenderFillRect, rect::SDL_Rect};
    use super::{Drawable, drawable::Dimensions, drawable::{Point, HasColor, HasDimensions, HasPoint, HasZIndex}, color::RGBColor, color::{Colors}};
    
    // default paddle dimensions
    pub const DEF_WIDTH : i32 = 40;
    pub const DEF_HEIGHT: i32 = 40;
    pub const DEF_DIMENSIONS: Dimensions = Dimensions{w: DEF_WIDTH, h: DEF_HEIGHT};
    pub const DEF_COLOR: RGBColor = RGBColor {r: 255, g: 255, b: 255, a: 255};

    pub struct Rectangle {
        id: usize,
        origin: Point,
        dims: Dimensions,
        color: RGBColor,
        z_index: i32
    }
    
    impl Rectangle {
        pub fn new(id: usize, origin: Point, dims: Dimensions, color: RGBColor, z_index: i32) -> Rectangle {
            Rectangle {
                id,
                origin,
                dims,
                color,
                z_index
            }
        }
    }

    impl Default for Rectangle {
        fn default() -> Self {
            Self {id: 0, origin: Point::new(0, 0), dims: Dimensions::new(0, 0), color: Colors::WHITE.as_rgb(), z_index: 0 }
        }
    }

    impl Drawable for Rectangle {
        fn fetch_id(&self) -> usize {
            self.id
        }

        unsafe fn draw(&self, renderer: *mut fermium::renderer::SDL_Renderer) -> bool {
            SDL_RenderFillRect(
                renderer,
                &mut SDL_Rect{
                    x: self.origin.x,
                    y: self.origin.y,
                    w: self.dims.w,
                    h: self.dims.h
                }
            ) == 0
        }
    }

    impl HasColor for Rectangle {
        fn fetch_color(&self) -> RGBColor {
            self.color
        }
    }

    impl HasDimensions for Rectangle {
        fn fetch_dimensions(&self) -> Dimensions {
            self.dims
        }
    }

    impl HasPoint for Rectangle {
        fn fetch_origin(&self) -> Point {
            self.origin
        }
    }

    impl HasZIndex for Rectangle {
        fn fetch_z_index(&self) -> i32 {
            self.z_index
        }
    }
}

pub mod line {
    use fermium::{renderer::{SDL_RenderDrawLine}};

    use super::{Drawable, drawable::{HasColor, HasPoint, Point, HasZIndex}, color::RGBColor};

    pub struct Line {
        id: usize,
        points: Vec<Point>,
        color: RGBColor,
        z_index: i32
    }

    impl Line {
        pub fn new(id: usize, points: Vec<Point>, color: RGBColor, z_index: i32) -> Self {
            Self {id, points, color, z_index}
        }
    }

    impl Drawable for Line {
        fn fetch_id(&self) -> usize {
            self.id
        }
        unsafe fn draw(&self, renderer: *mut fermium::renderer::SDL_Renderer) -> bool {

            for i in 1..self.points.len() {
                let p1 = self.points[i - 1];
                let p2 = self.points[i];
                if SDL_RenderDrawLine(renderer, p1.x, p1.y, p2.x, p2.y) != 0 {
                    return false
                }
            }
            true
        }
    }

    impl HasColor for Line {
        fn fetch_color(&self) -> RGBColor {
            self.color
        }
    }

    impl HasPoint for Line {
        fn fetch_origin(&self) -> Point {
            self.points[0]
        }
    }

    impl HasZIndex for Line {
        fn fetch_z_index(&self) -> i32 {
            self.z_index
        }
    }
}

