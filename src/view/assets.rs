use fermium::renderer::{SDL_Renderer, SDL_SetRenderDrawColor};

use self::drawable::{HasOrigin, HasColor, HasDimensions};

pub trait Drawable:  HasOrigin + HasColor + HasDimensions{
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

pub mod drawable {
    use super::color::RGBColor;
    // structs that all drawable objects require
    #[derive(Clone, Copy)]
    pub struct Origin {
        pub x: i32,
        pub y: i32
    }

    impl Origin {
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
    // Traits
    pub trait HasOrigin {
        fn fetch_origin(&self) -> Origin;
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
    #[derive(Clone, Copy)]
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
                Colors::BLUE    => build_rgb_tuple(0, 255, 0),
                Colors::GREEN   => build_rgb_tuple(0, 0, 255),
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
    use super::{Drawable, drawable::Dimensions, drawable::{Origin, HasColor, HasDimensions, HasOrigin}, color::RGBColor, color};
    
    // default paddle dimensions
    pub const DEF_WIDTH : i32 = 40;
    pub const DEF_HEIGHT: i32 = 40;
    pub const DEF_DIMENSIONS: Dimensions = Dimensions{w: DEF_WIDTH, h: DEF_HEIGHT};
    pub const DEF_COLOR: RGBColor = RGBColor {r: 255, g: 255, b: 255, a: 255};

    pub fn new(origin: Origin, dims: Dimensions, color: RGBColor) -> Rectangle {
        Rectangle {
            origin,
            dims,
            color
        }
    }

    pub fn default(origin: Origin) -> Rectangle {
        Rectangle {
            origin,
            dims: DEF_DIMENSIONS,
            color: DEF_COLOR
        }
    }

    pub struct Rectangle {
            origin: Origin,
            dims: Dimensions,
            color: RGBColor
    }
    
    impl Drawable for Rectangle {
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

    impl HasOrigin for Rectangle {
        fn fetch_origin(&self) -> Origin {
            self.origin
        }
    }
}

    

