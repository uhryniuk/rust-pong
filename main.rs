use ggez::*;
use ggez::graphics::Rect;
use ggez::graphics::Color;
use ggez::graphics::*;
use ggez::input::keyboard::KeyCode::*;
use rand::*;

// FINISHED OFF AT DRAWING THE SCOREBOARD


// Built in vector, uses built in math libraries
type Vector = ggez::mint::Vector2<f32>;
const SCREEN_WIDTH: f32 = 600.0;
const SCREEN_HEIGHT: f32 = 600.0;

const PADDLE_SPEED: f32 = 5.0;
const X_OFFSET: f32 = 20.; // distance from each paddle to their respective walls
const PADDLE_WIDTH: f32 = 12.;
const PADDLE_HEIGHT: f32 = 75.;

const BALL_RADIUS: f32 = 10.;


// Main state of the game: holds paddles
struct MainState {
    l_paddle: Rect,
    r_paddle: Rect,
    ball: Ball,
    l_score: u16,
    r_score: u16,
} 

// Paddle Object
struct Paddle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

// Ball Object
struct Ball {
    rect: Rect,
    vel: Vector,
}

impl Ball {
    fn new() -> Self {
        use rand::{thread_rng, Rng};
   
        let mut rng = thread_rng(); // initialize random number generator
        let mut x_vel = rng.gen_range(std::ops::Range{start: 1., end: 3.}); // generate random float from 3 to 5
        let mut y_vel = rng.gen_range(std::ops::Range{start: 1., end: 3.});
   
        // rng.gen::<bool> generates either true or false with a 50% chance of each
        if rng.gen::<bool>() {
            x_vel *= -1.0;
        }
        if rng.gen::<bool>() {
            y_vel *= -1.0;
        }
   
        Ball {
            rect: Rect::new(
                SCREEN_WIDTH / 2.0 - BALL_RADIUS / 2.0,
                SCREEN_HEIGHT / 2.0 - BALL_RADIUS / 2.0,
                BALL_RADIUS,
                BALL_RADIUS,
            ),
            vel: Vector { x: x_vel, y: y_vel },
        }
    }
}

impl event::EventHandler for MainState {

    fn update(&mut self, ctx: &mut Context) -> ggez::GameResult {

        if input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::W){
            self.l_paddle.y -= 5.0;
        }

        if input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::S){
            self.l_paddle.y += 5.0;
        }

        if input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::Up){
            self.r_paddle.y -= 5.0;
        }

        if input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::Down){
            self.r_paddle.y += 5.0;
        }

        // Could use self.ball.rect.y += self.ball.vel.y
        self.ball.rect.translate(self.ball.vel);

        if (self.ball.vel.x < 0.0 && self.ball.rect.overlaps(&self.l_paddle))
            || (self.ball.vel.x > 0.0 && self.ball.rect.overlaps(&self.r_paddle))
        {
            self.ball.vel.x *= -1.0;
        }
        
        if (self.ball.vel.y < 0.0 && self.ball.rect.top() < 0.0)
            || (self.ball.vel.y > 0.0 && self.ball.rect.bottom() > SCREEN_HEIGHT) {
                self.ball.vel.y *= -1.0;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> ggez::GameResult {
        // Clearing the screen before drawing to it, either init or again.
        ggez::graphics::clear(ctx, Color::new(0.0, 0.0, 0.0, 1.0));

        // Creating the ball and attaching the mesh
        let ball_mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), self.ball.rect, Color::new(1.0, 1.0, 1.0, 1.0))
        .expect("Error on the ball, man");

        draw(ctx, &ball_mesh, DrawParam::default())
        .expect("Error drawing the ball to screen");

        // Creating the left paddle and attaching the mesh
        let l_paddle_mesh = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            self.l_paddle,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )
        .expect("error creating ball mesh");
        draw(ctx, &l_paddle_mesh, DrawParam::default())
            .expect("error drawing ball mesh");

        // Creating the right paddle and attaching the mesh.
        let r_paddle_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            self.r_paddle,
            Color::new(1.0, 0.5, 1.0, 1.0),
        )
        .expect("error creating ball mesh");
        draw(ctx, &r_paddle_mesh, DrawParam::default())
            .expect("error drawing ball mesh");


        present(ctx).expect("error presenting");
        Ok(())
    }
}

pub fn main() -> ggez::GameResult{
    // Create a mutable referenc to a context and events loop
    let (ctx, event_loop) = ggez::ContextBuilder::new("Pong", "Dilly")
    .window_mode(conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
    .build().unwrap();

    // Make a mutable reference to MainState
    let main_state = MainState {
        l_paddle: Rect::new(20.0, 300.0, 20.0, 50.0),
        r_paddle: Rect::new(580.0, 300.0, 20.0, 50.0),
        ball: Ball::new(),
        l_score: 0,
        r_score: 0,
    };

    // Start the game
    ggez::event::run(ctx, event_loop, main_state)
}












