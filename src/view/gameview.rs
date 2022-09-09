
use fermium::{
    prelude::*,
    *
};
use priority_queue::PriorityQueue;
use crate::view::assets::Drawable;

use self::gfx_environments::{IsGraphicsEnvironment, IsWindow, IsRenderer, sdl2::{SDLWindow, SDLGraphicsEnvironment, SDLRenderer}, Environments, invalid::InvalidWindow};

use super::assets::color::Colors;


pub struct GameView {
    env: Box<dyn IsGraphicsEnvironment>,
    window: Box<dyn IsWindow>,
    renderer: Box<dyn IsRenderer>,
    drawings: PriorityQueue<Box<dyn Drawable>, i32>,
    started: bool,
    gfx_env: Environments,
    width: i32,
    height: i32
}

// Constructors
impl GameView {
    pub unsafe fn new(gfx_env: Environments, width: i32, height: i32) -> Self {
        let env = Environments::INVALID.build_env();
        let window = Environments::INVALID.build_window(width, height);
        let renderer = Environments::INVALID.build_renderer(&window);
        Self {env, window, renderer, drawings: PriorityQueue::new(), started: false, gfx_env, width, height}
    }

    pub unsafe fn default() -> Self {
        Self::new(
            Environments::SDL, 
            gfx_environments::sdl2::DEFAULT_WINDOW_WIDTH, 
            gfx_environments::sdl2::DEFAULT_WINDOW_HEIGHT
        )
    }

    pub unsafe fn sdl2(width: i32, height: i32) -> Self {
        Self::new(Environments::SDL, width, height)
    }
}


// Private 
impl GameView {
    unsafe fn clear_render(&self) {
        self.renderer.clear();
    }
    unsafe fn write_render(&self) {
        self.renderer.present();
    }
    unsafe fn render_all_drawings(&mut self) {
        while let Some((drawing, priority)) = self.drawings.pop() {
            self.renderer.draw_drawable(drawing);
        }
    }
    unsafe fn render_bg_color(&self) {
        self.renderer.set_render_color(Colors::BLACK.as_rgb())
    }


}

// public operations
impl GameView {
    pub unsafe fn init(&mut self) {
        self.env = self.gfx_env.build_env();
        self.env.init();
        self.started = true;
    }

    pub unsafe fn open_window(&mut self) {
        if !self.started {
            self.init();
        }
        self.window = self.gfx_env.build_window(self.width, self.height);
        self.renderer = self.gfx_env.build_renderer(&self.window)
    }

    pub unsafe fn get_screen_dimensions(&self) -> (i32, i32) {
        self.window.fetch_dimensions()
    }

    pub unsafe fn keyboard_input(&mut self) -> Option<i32> {
        self.env.handle_keyboard_input()
    }

    pub unsafe fn render(&mut self) {
        self.clear_render();
        self.render_all_drawings();
        self.render_bg_color();
        self.write_render();
        self.env.delay();
    }

    pub fn add_drawable_object(&mut self, drawing: Box<dyn Drawable>) {
        let priority = drawing.fetch_z_index();
        self.drawings.push(drawing, priority);
    }

    pub unsafe fn close_window(&mut self) {
        self.window.close();
        self.window = Box::new(InvalidWindow::default())
    }

    pub unsafe fn quit(&self) {
        self.env.quit();
    }
}


mod gfx_environments {
    use std::any::Any;

    use crate::view::assets::{color::RGBColor, Drawable};

    use self::{invalid::{InvalidGraphicsEnvironment, InvalidWindow, InvalidRenderer}, sdl2::{SDLGraphicsEnvironment, SDLWindow, SDLRenderer}};
    
    pub enum Environments {
        INVALID,
        SDL
    }

    impl Environments {
        pub fn build_env(&self) -> Box<dyn IsGraphicsEnvironment> {
            match self {
                Environments::INVALID => Box::new(InvalidGraphicsEnvironment::default()),
                Environments::SDL => Box::new(SDLGraphicsEnvironment::default()),
            }
        }

        pub unsafe fn build_window(&self, width: i32, height: i32) -> Box<dyn IsWindow> {
            match self {
                Environments::INVALID => Box::new(InvalidWindow::default()),
                Environments::SDL => Box::new(SDLWindow::from_dims(width, height)),
            }
        }

        pub unsafe fn build_renderer(&self, window: &Box<dyn IsWindow>) -> Box<dyn IsRenderer> {
            match self {
                Environments::INVALID => Box::new(InvalidRenderer::default()),
                Environments::SDL => {
                    Box::new(SDLRenderer::default(
                        match window.as_any().downcast_ref::<SDLWindow>() {
                            Some(win) => win.expose_window(),
                            None => panic!("Type mismatch! Expected SDLWindow!"),
                        }
                    ))
                },
            }
        }
    }

    pub trait IsGraphicsEnvironment {
        unsafe fn init(&self);
        unsafe fn quit(&self);
        unsafe fn delay(&self);
        unsafe fn handle_keyboard_input(&mut self) -> Option<i32>;
    }

    pub trait IsWindow {
        unsafe fn close(&mut self);
        unsafe fn fetch_dimensions(&self) -> (i32, i32);
        fn as_any(&self) -> &dyn Any;
    }
    
    pub trait IsRenderer {
        unsafe fn clear(&self);
        unsafe fn present(&self);
        unsafe fn set_render_color(&self, rgb: RGBColor);
        unsafe fn draw_drawable(&self, drawing: Box<dyn Drawable>);
    }

    pub mod invalid {
            use std::any::Any;

            use crate::view::gameview::gfx_environments::{IsGraphicsEnvironment, IsWindow, IsRenderer};
            use crate::view::assets::{color::RGBColor, Drawable};

            pub struct InvalidGraphicsEnvironment;

            impl IsGraphicsEnvironment for InvalidGraphicsEnvironment {
                unsafe fn init(&self) {
                    panic!("The graphics environment is invalid!");
                }

                unsafe fn quit(&self) {
                    panic!("The graphics environment is invalid!");
                }

                unsafe fn delay(&self) {
                    panic!("The graphics environment is invalid!");
                }

                unsafe fn handle_keyboard_input(&mut self) -> Option<i32> {
                    panic!("The graphics environment is invalid!");
                }
            }

            impl Default for InvalidGraphicsEnvironment {
                fn default() -> Self {
                    Self {  }
                }
            }

            pub struct InvalidWindow;

            impl IsWindow for InvalidWindow {
                unsafe fn close(&mut self) {
                    panic!("Invalid Window!");
                }

                unsafe fn fetch_dimensions(&self) -> (i32, i32) {
                    panic!("Invalid Window!");
                }

                fn as_any(&self) -> &dyn Any {
                    self
                }
            }

            impl Default for InvalidWindow {
                fn default() -> Self {
                    Self
                }
            }
            
            pub struct InvalidRenderer;

            impl IsRenderer for InvalidRenderer {
                unsafe fn clear(&self) {
                    panic!("Invalid renderer!")
                }

                unsafe fn present(&self) {
                    panic!("Invalid renderer!")
                }

                unsafe fn set_render_color(&self, rgb: RGBColor) {
                    panic!("Invalid renderer!")
                }

                unsafe fn draw_drawable(&self, drawing: Box<dyn Drawable>) {
                    panic!("Invalid renderer!")
                }
            }

            impl Default for InvalidRenderer {
                fn default() -> Self {
                    Self {  }
                }
            }
        }
    
    pub mod sdl2 {
        
        use std::any::Any;

        use fermium::{SDL_Init, SDL_INIT_EVERYTHING, SDL_Quit, timer::SDL_Delay, video::{SDL_Window, SDL_GetWindowSize, SDL_DestroyWindow, SDL_CreateWindow, SDL_WINDOWPOS_CENTERED}, renderer::{SDL_Renderer, SDL_CreateRenderer, SDL_RenderClear, SDL_RenderPresent, SDL_SetRenderDrawColor}, prelude::{SDL_Event, SDL_WaitEventTimeout, SDL_KEYDOWN}};
        use crate::view::gameview::{gfx_environments::IsGraphicsEnvironment};

        use self::options::{WindowOptions, RendererOptions};
        use super::{IsWindow, IsRenderer};

        pub const DEFAULT_WINDOW_WIDTH: i32 = 800;
        pub const DEFAULT_WINDOW_HEIGHT: i32 = 600;
        pub const DEFAULT_WINDOW_X_POS: i32 = SDL_WINDOWPOS_CENTERED;
        pub const DEFAULT_WINDOW_Y_POS: i32 = SDL_WINDOWPOS_CENTERED;

        pub mod options {
            use fermium::video::{SDL_WINDOW_OPENGL, SDL_WINDOWPOS_CENTERED, SDL_WINDOW_ALLOW_HIGHDPI};

            pub struct WindowOptions {
                pub title: String,
                pub x: i32,
                pub y: i32,
                pub w: i32,
                pub h: i32,
                pub flags: u32
            }
            
            impl WindowOptions {
                fn new(title: String, x: i32, y: i32, w: i32, h: i32, flags: u32) -> Self {
                    Self {
                        title,
                        x,
                        y,
                        w,
                        h,
                        flags
                    }
                }
            }

            impl Default for WindowOptions {
                fn default() -> Self {
                    Self::new(
                        String::from("Default"),
                        super::DEFAULT_WINDOW_X_POS,
                        super::DEFAULT_WINDOW_Y_POS,
                        super::DEFAULT_WINDOW_WIDTH,
                        super::DEFAULT_WINDOW_HEIGHT,
                        (SDL_WINDOW_OPENGL | SDL_WINDOW_ALLOW_HIGHDPI).0,
                    )
                }
            }
            pub struct RendererOptions {
                pub index: i32,
                pub flags: u32
            }

            impl RendererOptions {
                pub fn new(index: i32, flags: u32) -> Self {
                    Self {index, flags}
                }
            }
            
            impl Default for RendererOptions {
                fn default() -> Self {
                    Self::new(-1, 1)
                }
            }
        }
        
        pub struct SDLGraphicsEnvironment {
            ms_delay: u32,
            event: SDL_Event,
            timeout: i32
        }

        impl SDLGraphicsEnvironment {
            const SDL_EVENT_SUCCESS: i32 = 1;
            const SDL_EVENT_FAILURE: i32 = 0;
        }

        

        impl IsGraphicsEnvironment for SDLGraphicsEnvironment {
            unsafe fn init(&self) {
                assert_eq!(SDL_Init(SDL_INIT_EVERYTHING), 0)
            }
            
            unsafe fn quit(&self) {
                SDL_Quit()
            }

            unsafe fn delay(&self) {
                SDL_Delay(self.ms_delay)
            }

            unsafe fn handle_keyboard_input(&mut self) -> Option<i32> {
                if SDL_WaitEventTimeout(&mut self.event, self.timeout) ==  Self::SDL_EVENT_SUCCESS {
                    match self.event.type_ {
                        SDL_KEYDOWN => {
                            Option::Some(self.event.key.keysym.sym.0)
                        },
                        _ => Option::None
                    }
                } else {
                    Option::None
                }
            }
        }

        impl SDLGraphicsEnvironment {
            pub const DEFAULT_REFRESH_RATE: u32 = 10;
        }

        impl SDLGraphicsEnvironment {
            pub fn new(ms_delay: u32) -> Self {
                Self{ms_delay, event: SDL_Event::default(), timeout: 1}
            }
        }

        impl Default for SDLGraphicsEnvironment {
            fn default() -> Self {Self::new(Self::DEFAULT_REFRESH_RATE)}
        }

        



        pub struct SDLWindow {
            window: *mut SDL_Window
        }

        impl IsWindow for SDLWindow {
            unsafe fn close(&mut self) {
                SDL_DestroyWindow(self.window);
            }

            unsafe fn fetch_dimensions(&self) -> (i32, i32) {
                let mut w: i32 = 0;
                let mut h: i32 = 0;
                SDL_GetWindowSize(self.window, &mut w, &mut h);
                (w, h)
            }

            fn as_any(&self) -> &dyn Any {
                self
            }
        }

        impl SDLWindow {
            pub unsafe fn new(options: WindowOptions) -> Self {
                let window = SDL_CreateWindow(
                    options.title.as_bytes().as_ptr().cast(),
                    options.x,
                    options.y,
                    options.w,
                    options.h,
                    options.flags
                );
                assert!(!window.is_null());
                Self {window}
            }

            pub fn expose_window(&self) -> *mut SDL_Window {
                self.window
            }

        }

        impl SDLWindow {
            pub unsafe fn default() -> Self {
                Self::new(WindowOptions::default())
            }

            pub unsafe fn from_dims(width: i32, height: i32) -> Self {
                let mut options = WindowOptions::default();
                options.w = width;
                options.h = height;
                Self::new(options)
            }
        }

        pub struct SDLRenderer {
            renderer: *mut SDL_Renderer
        }

        impl IsRenderer for SDLRenderer {
            unsafe fn clear(&self) {
                SDL_RenderClear(self.renderer);
            }

            unsafe fn present(&self) {
                SDL_RenderPresent(self.renderer);
            }

            unsafe fn set_render_color(&self, rgb: crate::view::assets::color::RGBColor) {
                let (r, g, b, a) = rgb.as_tuple();
                SDL_SetRenderDrawColor(self.renderer, r, g, b, a);
            }

            unsafe fn draw_drawable(&self, drawing: Box<dyn crate::view::assets::Drawable>) {
                drawing.render_with(self.renderer);
            }
        }

        impl SDLRenderer {
            pub unsafe fn new(window: *mut SDL_Window, options: RendererOptions) -> Self {
                let renderer = SDL_CreateRenderer(window, options.index, options.flags);
                assert!(!renderer.is_null());
                Self {
                    renderer
                }
            }

            pub unsafe fn default(window: *mut SDL_Window) -> Self {
                Self::new(window, RendererOptions::default())
            }
        }

        
    }
}
