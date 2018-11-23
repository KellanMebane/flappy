extern crate find_folder;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate rand;

//use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, GlyphCache, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use piston_window::*;
use rand::prelude::*;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
    x: f64,
    y: f64,
    wall_x: f64,
    wall_size: f64,
    gravity: f64,
    velocity: f64,
    over: bool,
    points: usize,
}

impl App {
    fn render(&mut self, args: &RenderArgs, text_glyphs: &mut GlyphCache) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let wall_u: [f64; 4] = [0.0, 0.0, 50.0, self.wall_size];
        let wall_l: [f64; 4] = [0.0, 0.0, 50.0, 325.0 - self.wall_size];

        //let rotation = self.rotation;
        let x = self.x;
        let y = self.y;
        let wall_x = self.wall_x;
        let size = self.wall_size;
        let over = self.over;
        let score = format!("Score: {}", self.points);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            rectangle(RED, square, c.transform.trans(x, y), gl);
            rectangle(BLUE, wall_u, c.transform.trans(wall_x, 0.0), gl);
            rectangle(BLUE, wall_l, c.transform.trans(wall_x, size + 175.0), gl);

            if over {
                rectangle(RED, [0.0, 0.0, 500.0, 500.0], c.transform, gl);
            }

            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 42)
                .draw(
                    &score,
                    text_glyphs,
                    &c.draw_state,
                    c.transform.trans(200.0, 200.0),
                    gl,
                ).unwrap();
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
        self.y = self.y - (self.velocity * args.dt);
        self.velocity += self.gravity / 1.0;
        //println!("velocity {}", self.velocity);
        self.wall_x -= 250.0 * args.dt;
        if self.wall_x < -50.0 {
            self.wall_x = 500.0;
            if !self.over {
                self.points += 1;
            }

            let mut rng = rand::thread_rng();
            let num: f64 = rng.gen_range(50.0, 300.0);
            self.wall_size = num;
        }

        if self.y > 450.0 {
            self.y = 450.0;
            self.velocity = 0.0;
            //self.gravity = 0.0;
        }

        if !self.over {
            self.over = self.collision(
                self.x,
                self.x + 50.0,
                self.y,
                self.y + 50.0,
                self.wall_x,
                self.wall_x + 50.0,
                self.wall_size + 175.0,
                500.0,
            );

            if self.over == true {
                return;
            }

            self.over = self.collision(
                self.x,
                self.x + 50.0,
                self.y,
                self.y + 50.0,
                self.wall_x,
                self.wall_x + 50.0,
                0.0,
                self.wall_size,
            );

            if self.over == true {
                return;
            }

            if self.y < 0.0 || self.y + 50.0 >= 500.0 {
                self.over = true;
            }
        }
    }

    fn arc(&mut self) {
        self.velocity = 500.0;
        //self.gravity = -9.8;
    }

    fn restart(&mut self) {
        self.x = 20.0;
        self.y = 200.0;
        self.gravity = -9.8;
        self.velocity = 0.0;
        self.wall_x = 500.0;
        let mut rng = rand::thread_rng();
        let num: f64 = rng.gen_range(50.0, 300.0);
        self.wall_size = num;
        self.over = false;
        self.points = 0;
    }

    fn collision(
        &self,
        lx: f64,
        lxu: f64,
        ly: f64,
        lyu: f64,
        rx: f64,
        rxu: f64,
        ry: f64,
        ryu: f64,
    ) -> bool {
        if lx < rxu && lxu > rx && ly < ryu && lyu > ry {
            // collision detected!
            return true;
        }
        false
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: PistonWindow = WindowSettings::new("spinning-square", [500, 500])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    //let mut gl = GlGraphics::new(opengl);
    //let mut events = Events::new(EventSettings::new());

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
        x: 25.0,
        y: 200.0,
        gravity: -9.8,
        velocity: 0.0,
        wall_x: 500.0,
        wall_size: 200.0,
        over: false,
        points: 0,
    };

    let mut glyphs =
        opengl_graphics::GlyphCache::new("assets/FiraSans-Regular.ttf", (), TextureSettings::new())
            .unwrap();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r, &mut glyphs);
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::W {
                app.arc();
            }

            if key == Key::Space {
                app.restart();
            }
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
