use ggez;
use ggez::graphics::{Color, Drawable, Rect, Text};
use ggez::input::keyboard::KeyCode;
use ggez::mint::{Point2, Vector2};
use ggez::{event, graphics, Context, GameResult};
use rand::{thread_rng, Rng};

const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;
const RACKET_PADDING: f32 = 40.0;

const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;

const PLAYER_SPEED: f32 = 500.0;
const BALL_SPEED: f32 = 400.0;

const MIDDLE_LINE_WIDTH: f32 = 2.0;

fn clamp(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}

fn move_racket(pos: &mut Point2<f32>, keycode: KeyCode, y_dir: f32, ctx: &mut Context) {
    let dt = ctx.time.delta().as_secs_f32();
    let screen_h = ctx.gfx.drawable_size().1;

    if ctx.keyboard.is_key_pressed(keycode) {
        pos.y += y_dir * PLAYER_SPEED * dt;
    }
    clamp(&mut pos.y, 0.0, screen_h);
}

fn randomize_vec(vec: &mut Vector2<f32>, x: f32, y: f32) {
    let mut rng = thread_rng();
    vec.x = match rng.gen_bool(0.5) {
        true => x,
        false => -x,
    };
    vec.y = match rng.gen_bool(0.5) {
        true => y,
        false => -y,
    };
}

struct MainState {
    player_1_pos: Point2<f32>,
    player_2_pos: Point2<f32>,
    ball_pos: Point2<f32>,
    ball_vel: Vector2<f32>,
    player_1_score: i32,
    player_2_score: i32,
}

impl MainState {
    pub fn new(ctx: &Context) -> GameResult<MainState> {
        let (screen_w, screen_h) = ctx.gfx.drawable_size();
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);

        let mut ball_vel = Vector2 { x: 0.0, y: 0.0 };
        randomize_vec(&mut ball_vel, BALL_SPEED, BALL_SPEED);

        let main_state = MainState {
            player_1_pos: Point2 {
                x: RACKET_WIDTH_HALF + RACKET_PADDING,
                y: screen_h_half,
            },
            player_2_pos: Point2 {
                x: screen_w - RACKET_WIDTH_HALF - RACKET_PADDING,
                y: screen_h_half,
            },
            ball_pos: Point2 {
                x: screen_w_half,
                y: screen_h_half,
            },
            ball_vel,
            player_1_score: 0,
            player_2_score: 0,
        };
        Ok(main_state)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ctx.time.delta().as_secs_f32();
        let (screen_w, screen_h) = ctx.gfx.drawable_size();

        move_racket(&mut self.player_1_pos, KeyCode::W, -1.0, ctx);
        move_racket(&mut self.player_1_pos, KeyCode::S, 1.0, ctx);
        move_racket(&mut self.player_2_pos, KeyCode::I, -1.0, ctx);
        move_racket(&mut self.player_2_pos, KeyCode::K, 1.0, ctx);

        self.ball_pos.x += self.ball_vel.x * dt;
        self.ball_pos.y += self.ball_vel.y * dt;

        if self.ball_pos.x < 0.0 {
            self.ball_pos.x = screen_w * 0.5;
            self.ball_pos.y = screen_h * 0.5;
            randomize_vec(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
            self.player_2_score += 1;
        }

        if self.ball_pos.x > screen_w {
            self.ball_pos.x = screen_w * 0.5;
            self.ball_pos.y = screen_h * 0.5;
            randomize_vec(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
            self.player_1_score += 1;
        }

        // ball bounce
        if self.ball_pos.y < BALL_SIZE_HALF {
            self.ball_pos.y = BALL_SIZE_HALF;
            self.ball_vel.y = self.ball_vel.y.abs();
        } else if self.ball_pos.y > screen_h - BALL_SIZE_HALF {
            self.ball_pos.y = screen_h - BALL_SIZE_HALF;
            self.ball_vel.y = -self.ball_vel.y.abs();
        }

        let intersects_player_1 = self.ball_pos.x - BALL_SIZE_HALF
            < self.player_1_pos.x + RACKET_WIDTH_HALF
            && self.ball_pos.x + BALL_SIZE_HALF > self.player_1_pos.x - RACKET_WIDTH_HALF
            && self.ball_pos.y - BALL_SIZE_HALF < self.player_1_pos.y + RACKET_HEIGHT_HALF
            && self.ball_pos.y + BALL_SIZE_HALF > self.player_1_pos.y - RACKET_HEIGHT_HALF;

        if intersects_player_1 {
            self.ball_vel.x = self.ball_vel.x.abs();
        }

        let intersects_player_2 = self.ball_pos.x - BALL_SIZE_HALF
            < self.player_2_pos.x + RACKET_WIDTH_HALF
            && self.ball_pos.x + BALL_SIZE_HALF > self.player_2_pos.x - RACKET_WIDTH_HALF
            && self.ball_pos.y - BALL_SIZE_HALF < self.player_2_pos.y + RACKET_HEIGHT_HALF
            && self.ball_pos.y + BALL_SIZE_HALF > self.player_2_pos.y - RACKET_HEIGHT_HALF;

        if intersects_player_2 {
            self.ball_vel.x = -self.ball_vel.x.abs();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        let (screen_w, screen_h) = ctx.gfx.drawable_size();
        let racket_rect = Rect::new(
            -RACKET_WIDTH_HALF,
            -RACKET_HEIGHT_HALF,
            RACKET_WIDTH,
            RACKET_HEIGHT,
        );
        let racket_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            racket_rect,
            Color::WHITE,
        )?;

        let ball_rect = Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);
        let ball_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            ball_rect,
            Color::WHITE,
        )?;

        let score_text = Text::new(format!(
            "{}             {}",
            self.player_1_score, self.player_2_score
        ));
        let text_rect = score_text.dimensions(ctx).unwrap();

        let middle_rect = Rect::new(-MIDDLE_LINE_WIDTH * 0.5, 0.0, MIDDLE_LINE_WIDTH, screen_h);
        let middle_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            middle_rect,
            Color::WHITE,
        )?;

        // drawing

        let draw_param = graphics::DrawParam::default().dest(Vector2 {
            x: screen_w * 0.5,
            y: 0.0,
        });
        canvas.draw(&middle_mesh, draw_param);

        let draw_param_1 = graphics::DrawParam::default().dest(self.player_1_pos);
        canvas.draw(&racket_mesh, draw_param_1);

        let draw_param_2 = graphics::DrawParam::default().dest(self.player_2_pos);
        canvas.draw(&racket_mesh, draw_param_2);

        let draw_param_3 = graphics::DrawParam::default().dest(self.ball_pos);
        canvas.draw(&ball_mesh, draw_param_3);

        let mut score_pos = Vector2 {
            x: screen_w * 0.5,
            y: screen_h * 0.1,
        };
        score_pos.x -= text_rect.w * 0.5;

        let draw_param_4 = graphics::DrawParam::default().dest(score_pos);
        canvas.draw(&score_text, draw_param_4);

        canvas.finish(ctx)?;

        Ok(())
    }
}
fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Pong", "Jesus");
    let (ctx, event_loop) = cb.build()?;
    ctx.gfx.set_window_title("Pong");
    let state = MainState::new(&ctx)?;

    event::run(ctx, event_loop, state)
}
