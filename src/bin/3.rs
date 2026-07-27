use std::f32::consts::PI;

use macroquad::prelude::*;

const WINDOW_W: f32 = 800.0;
const WINDOW_H: f32 = 600.0;
const PADDLE_W: f32 = 12.0;
const PADDLE_H: f32 = 80.0;
const BALL_SIZE: f32 = 12.0;
const MAX_MULTIPLIER: f32 = 3.0;
const PADDLE_OFFSET: f32 = 20.0;
const PADDLE_SPEED: f32 = 1500.0; // pixels per second
const WIN_SCORE: u32 = 5;
const AI_DIFFICULTY: u8 = 1;
const AI_UPDATE_RANGE: f32 = 10.0_f32;

struct Paddle<'a> {
    rect: Rect,
    texture: &'a Texture2D,
    ai: Option<u8>,
}

impl<'a> Paddle<'a> {
    fn new(x: f32, texture: &'a Texture2D, ai: Option<u8>) -> Self {
        Self {
            rect: Rect::new(x, WINDOW_H / 2.0 - PADDLE_H / 2.0, PADDLE_W, PADDLE_H),
            texture,
            ai,
        }
    }

    fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.rect.x,
            self.rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.rect.w, self.rect.h)),
                ..Default::default()
            },
        );
    }

    fn update(&mut self, dt: f32, going_up_key: KeyCode, going_down_key: KeyCode, ball: &Ball) {
        match self.ai {
            Some(difficulty) => match difficulty {
                0 => {
                    if self.rect.y + PADDLE_H / 2.0_f32 - ball.rect.y > AI_UPDATE_RANGE {
                        self.rect.y -= PADDLE_SPEED * dt;
                    } else if self.rect.y + PADDLE_H / 2.0_f32 - ball.rect.y < -AI_UPDATE_RANGE {
                        self.rect.y += PADDLE_SPEED * dt;
                    }
                    self.rect.y = clamp(self.rect.y, 0.0, WINDOW_H - PADDLE_H);
                }
                1 => {
                    let t = (WINDOW_W - ball.rect.x) / ball.vel.x;
                    let y = ball.rect.y + t * ball.vel.y;
                    if self.rect.y + PADDLE_H / 2.0_f32 - y > 0.0_f32 {
                        self.rect.y -= PADDLE_SPEED * dt;
                    } else if self.rect.y + PADDLE_H / 2.0_f32 - y < 0.0_f32 {
                        self.rect.y += PADDLE_SPEED * dt;
                    }
                    self.rect.y = clamp(self.rect.y, 0.0, WINDOW_H - PADDLE_H);
                }

                _ => {
                    if self.rect.y + PADDLE_H / 2.0_f32 - ball.rect.y > AI_UPDATE_RANGE {
                        self.rect.y -= PADDLE_SPEED * dt;
                    } else if self.rect.y + PADDLE_H / 2.0_f32 - ball.rect.y < -AI_UPDATE_RANGE {
                        self.rect.y += PADDLE_SPEED * dt;
                    }
                    self.rect.y = clamp(self.rect.y, 0.0, WINDOW_H - PADDLE_H);
                }
            },
            None => {
                if is_key_down(going_down_key) {
                    self.rect.y += PADDLE_SPEED * dt;
                }

                if is_key_down(going_up_key) {
                    self.rect.y -= PADDLE_SPEED * dt;
                }

                self.rect.y = clamp(self.rect.y, 0.0, WINDOW_H - PADDLE_H);
            }
        }
    }
}

struct Ball<'b> {
    rect: Rect,
    vel: Vec2,
    texture: &'b Texture2D,
    // Q1
    vel_multiplier: f32,
}

impl<'b> Ball<'b> {
    fn new(texture: &'b Texture2D) -> Self {
        Self {
            rect: Rect::new(
                WINDOW_W / 2.0 - BALL_SIZE / 2.0,
                WINDOW_H / 2.0 - BALL_SIZE / 2.0,
                BALL_SIZE,
                BALL_SIZE,
            ),
            vel: Vec2::new(2.0 * 300.0, 2.0 * 220.0),
            texture,
            vel_multiplier: 1_f32,
        }
    }

    fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            self.rect.x,
            self.rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.rect.w, self.rect.h)),
                ..Default::default()
            },
        );
    }

    fn update(&mut self, dt: f32) {
        self.rect.x += self.vel.x * (dt * self.vel_multiplier);
        self.rect.y += self.vel.y * (dt * self.vel_multiplier);

        // bounce off top wall
        if self.rect.y < 0.0 {
            self.rect.y = 0.0;
            self.vel.y = self.vel.y.abs();
        }
        // bounce off bottom wall
        if self.rect.y + self.rect.h > WINDOW_H {
            self.rect.y = WINDOW_H - self.rect.h;
            self.vel.y = -self.vel.y.abs();
        }
    }

    fn check_paddles(&mut self, left: &Paddle, right: &Paddle) {
        if self.rect.overlaps(&left.rect) {
            let mut y_diff = self.rect.y + BALL_SIZE - (left.rect.y + PADDLE_H / 2.0);
            y_diff *= 2.0 / PADDLE_H;
            if y_diff < -1.0 {
                y_diff = -1.0;
            } else if y_diff > 1.0 {
                y_diff = 1.0;
            }
            let vel_length = self.vel.length();
            let coef: f32 = PI;
            self.vel = Vec2 {
                x: (f32::cos(coef * y_diff)),
                y: (f32::sin(coef * y_diff)),
            } * vel_length;
            self.rect.x = left.rect.x + left.rect.w; // push ball out
            self.vel.x = self.vel.x.abs();
        }

        if self.rect.overlaps(&right.rect) {
            let mut y_diff = self.rect.y + BALL_SIZE - (right.rect.y + PADDLE_H / 2.0);
            y_diff *= 2.0 / PADDLE_H;
            if y_diff < -1.0 {
                y_diff = -1.0;
            } else if y_diff > 1.0 {
                y_diff = 1.0;
            }
            let vel_length = self.vel.length();
            let coef: f32 = f32::to_radians(100.0);

            self.vel = Vec2 {
                x: (f32::cos(coef * y_diff)),
                y: (f32::sin(coef * y_diff)),
            } * vel_length;
            self.vel.x = -self.vel.x.abs();
            self.rect.x = right.rect.x - self.rect.w; // push ball out
        }
    }

    fn reset(&mut self, p: Player) {
        self.rect.x = WINDOW_W / 2.0 - BALL_SIZE / 2.0;
        self.rect.y = WINDOW_H / 2.0 - BALL_SIZE / 2.0;
        match p {
            Player::Left => {
                self.vel.x = -self.vel.x.abs();
            }
            Player::Right => {
                self.vel.x = self.vel.x.abs();
            }
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Pong".to_owned(),
        ..Conf::default()
    }
}

fn draw_centre_line() {
    let mut y = 10.0;
    while y < WINDOW_H {
        draw_line(WINDOW_W / 2.0, y, WINDOW_W / 2.0, y + 15.0, 2.0, DARKGRAY);
        y += 25.0;
    }
}

struct Score {
    left: u32,
    right: u32,
}

impl Default for Score {
    fn default() -> Self {
        Self { left: 0, right: 0 }
    }
}

enum GameState {
    Playing,
    GameOver,
}
enum Player {
    Left,
    Right,
}
impl Score {
    fn draw(&self) {
        let text = format!("{}   {}", self.left, self.right);
        let dims = measure_text(&text, None, 48, 1.0);
        draw_text(&text, WINDOW_W / 2.0 - dims.width / 2.0, 48.0, 48.0, WHITE);
    }

    fn update(&mut self, ball: &mut Ball) -> Option<Player> {
        let left_exit = ball.rect.x + ball.rect.w < 0.0;
        let right_exit = ball.rect.x > WINDOW_W;
        let mut p = Player::Left;
        if left_exit {
            self.right += 1;
            p = Player::Right;
        }

        if right_exit {
            p = Player::Left;
            self.left += 1;
        }

        //update vel_mult
        if right_exit || left_exit {
            ball.vel_multiplier = 1.0_f32
                + (MAX_MULTIPLIER - 1.0_f32) as f32 * (self.right as f32 + self.left as f32) as f32
                    / (2.0_f32 * WIN_SCORE as f32 - 2.0_f32) as f32;
        }

        if left_exit || right_exit {
            Some(p)
        } else {
            None
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    /* Run the game loop, stepping the simulation once per frame. */
    let mut score = Score::default();
    let mut game_state = GameState::Playing;
    let mut winner = "";
    let ball_texture = load_texture("assets/ball.png").await.unwrap();
    let paddle_texture = load_texture("assets/paddle.png").await.unwrap();
    let mut ball = Ball::new(&ball_texture);
    let mut left = Paddle::new(PADDLE_OFFSET, &paddle_texture, None);
    let mut right = Paddle::new(
        WINDOW_W - PADDLE_W - PADDLE_OFFSET,
        &paddle_texture,
        Some(AI_DIFFICULTY),
    );
    loop {
        let dt = get_frame_time();

        match game_state {
            GameState::Playing => {
                clear_background(BLACK);
                draw_centre_line();

                let subloop = (ball.vel_multiplier) + 1.0;
                let substep = dt / subloop;
                for _ in 1..(subloop as u8) {
                    left.update(substep, KeyCode::W, KeyCode::S, &ball);
                    right.update(substep, KeyCode::Up, KeyCode::Down, &ball);
                    ball.update(substep);
                    ball.check_paddles(&left, &right);
                }

                match score.update(&mut ball) {
                    Some(p) => {
                        ball.reset(p);
                        if score.left >= WIN_SCORE {
                            winner = "Left player wins!";
                            game_state = GameState::GameOver;
                        } else if score.right >= WIN_SCORE {
                            winner = "Right player wins!";
                            game_state = GameState::GameOver;
                        }
                    }
                    None => {}
                }

                left.draw();
                right.draw();
                ball.draw();
                score.draw();
            }
            GameState::GameOver => {
                let dims = measure_text(winner, None, 48, 1.0);
                draw_text(
                    winner,
                    WINDOW_W / 2.0 - dims.width / 2.0,
                    WINDOW_H / 2.0,
                    48.0,
                    WHITE,
                );

                let hint = "Press R to restart";
                let hdims = measure_text(hint, None, 24, 1.0);
                draw_text(
                    hint,
                    WINDOW_W / 2.0 - hdims.width / 2.0,
                    WINDOW_H / 2.0 + 40.0,
                    24.0,
                    GRAY,
                );

                if is_key_pressed(KeyCode::R) {
                    score = Score::default();
                    ball = Ball::new(&ball_texture);
                    left = Paddle::new(PADDLE_OFFSET, &paddle_texture, None);
                    right = Paddle::new(
                        WINDOW_W - PADDLE_OFFSET - PADDLE_W,
                        &paddle_texture,
                        Some(AI_DIFFICULTY),
                    );
                    game_state = GameState::Playing;
                }
            }
        }

        next_frame().await;
    }
}
