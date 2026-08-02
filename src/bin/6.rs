use std::f32::consts::PI;

use macroquad::{prelude::*, rand::gen_range};

const WINDOW_W: f32 = 800.0;
const WINDOW_H: f32 = 600.0;
const PADDLE_W: f32 = 12.0;
const PADDLE_H: f32 = 80.0;
const MIN_PADDLE_H: f32 = 40.0;
const BALL_SIZE: f32 = 12.0;
const MAX_MULTIPLIER: f32 = 3.0;
const BALL_SPEED_MULTIPLIER: f32 = 2.5;
const PADDLE_OFFSET: f32 = 20.0;
const PADDLE_SPEED: f32 = 1500.0; // pixels per second
const WIN_SCORE: u32 = 5;
const AI_DIFFICULTY: u8 = 0;
const AI_UPDATE_RANGE: f32 = 13.0_f32;
const MAX_THETA: f32 = PI / 3.0;
const COUNT_DOWN_TIME: f32 = 3.0_f32;
const POWERUP_SIZE: f32 = 150.0;
const SHIELD_OFFSET: f32 = 10.0;
struct Paddle<'a> {
    rect: Rect,
    texture: &'a Texture2D,
    ai: Option<u8>,
    vel_multiplier: f32,
}

impl<'a> Paddle<'a> {
    fn new(x: f32, texture: &'a Texture2D, ai: Option<u8>) -> Self {
        Self {
            rect: Rect::new(x, WINDOW_H / 2.0 - PADDLE_H / 2.0, PADDLE_W, PADDLE_H),
            texture,
            ai,
            vel_multiplier: 1.0,
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

    fn update(
        &mut self,
        dt: f32,
        going_up_key: KeyCode,
        going_down_key: KeyCode,
        balls: &Vec<Ball>,
    ) {
        match self.ai {
            Some(difficulty) => {
                let vel_multiplier = self.vel_multiplier.abs();
                let mut idx_max = 0;
                let mut max_x = balls[0].rect.x;
                for i in 0..balls.len() {
                    if balls[i].vel.x > 0.0 && balls[i].rect.x > max_x {
                        idx_max = i;
                        max_x = balls[i].rect.x;
                    }
                }
                let ball = &balls[idx_max];
                if ball.vel.x > 0.0 {
                    let t = (WINDOW_W - ball.rect.x) / ball.vel.x * difficulty as f32 / 2.0;
                    let y = ball.rect.y + BALL_SIZE / 2.0 + t * ball.vel.y;
                    let diff = (self.rect.y) + self.rect.h / 2.0_f32 - y;
                    if (diff) > AI_UPDATE_RANGE * (3.0 - difficulty as f32) {
                        self.rect.y -= PADDLE_SPEED * vel_multiplier * dt;
                    } else if diff < -AI_UPDATE_RANGE * (3.0 - difficulty as f32) {
                        self.rect.y += PADDLE_SPEED * vel_multiplier * dt;
                    }
                    self.rect.y = clamp(self.rect.y, 0.0, WINDOW_H - self.rect.h);
                }
            }
            None => {
                if is_key_down(going_down_key) {
                    self.rect.y += PADDLE_SPEED * self.vel_multiplier * dt;
                }

                if is_key_down(going_up_key) {
                    self.rect.y -= PADDLE_SPEED * self.vel_multiplier * dt;
                }

                self.rect.y = clamp(self.rect.y, 0.0, WINDOW_H - self.rect.h);
            }
        }
    }

    fn shrink(&mut self, amount: f32) {
        self.rect.h = MIN_PADDLE_H.max(self.rect.h - amount);
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
            vel: Vec2::new(BALL_SPEED_MULTIPLIER * 300.0, BALL_SPEED_MULTIPLIER * 220.0),
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
            let mut y_diff = self.rect.y + BALL_SIZE / 2.0 - (left.rect.y + left.rect.h / 2.0);
            y_diff *= 2.0 / left.rect.h;
            if y_diff < -1.0 {
                y_diff = -1.0;
            } else if y_diff > 1.0 {
                y_diff = 1.0;
            }
            let vel_length = self.vel.length();
            self.vel = Vec2 {
                x: (f32::cos(MAX_THETA * y_diff)),
                y: (f32::sin(MAX_THETA * y_diff)),
            } * vel_length;
            self.rect.x = left.rect.x + left.rect.w; // push ball out
            self.vel.x = self.vel.x.abs();
        }

        if self.rect.overlaps(&right.rect) {
            let mut y_diff = self.rect.y + BALL_SIZE / 2.0 - (right.rect.y + right.rect.h / 2.0);
            y_diff *= 2.0 / right.rect.h;
            if y_diff < -1.0 {
                y_diff = -1.0;
            } else if y_diff > 1.0 {
                y_diff = 1.0;
            }
            let vel_length = self.vel.length();
            self.vel = Vec2 {
                x: (f32::cos(MAX_THETA * y_diff)),
                y: (f32::sin(MAX_THETA * y_diff)),
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

impl Score {
    fn draw(&self) {
        let text = format!("{}   {}", self.left, self.right);
        let dims = measure_text(&text, None, 48, 1.0);
        draw_text(&text, WINDOW_W / 2.0 - dims.width / 2.0, 48.0, 48.0, WHITE);
    }

    fn update(&mut self, balls: &mut Vec<Ball>) -> Option<Player> {
        for ball in balls {
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
                    + (MAX_MULTIPLIER - 1.0_f32) as f32
                        * (self.right as f32 + self.left as f32) as f32
                        / (2.0_f32 * WIN_SCORE as f32 - 2.0_f32) as f32;
            }

            if left_exit || right_exit {
                return Some(p);
            } else {
                return None;
            }
        }
        None
    }
}

enum GameState {
    Playing,
    GameOver,
    Stopwatch(f32),
}

#[derive(Clone, Copy, Debug)]
enum Player {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
enum PowerUpEffect {
    BigPaddle,
    EnemySmallPaddle,
    SlowBall,
    FastBall,
    EnemyFreeze,
    EnemyReverseControls,
    Shield,
    MultiBall,
    None,
}

struct PowerUp<'txt> {
    effect: PowerUpEffect,
    rect: Rect,
    texture: &'txt Texture2D,
    time: f32,
    claimed: bool,
}

impl<'txt> PowerUp<'txt> {
    fn new(texture: &'txt Texture2D) -> Self {
        Self {
            effect: PowerUpEffect::None,
            rect: Rect {
                x: WINDOW_W / 2.0 - POWERUP_SIZE / 2.0,
                y: 0.0,
                w: POWERUP_SIZE,
                h: POWERUP_SIZE,
            },
            texture,
            time: 0.0,
            claimed: false,
        }
    }
    fn rand_gen(&mut self) -> f32 {
        let idx = macroquad::rand::gen_range(1, 9);
        let t: f32;
        match idx {
            1 => {
                self.effect = PowerUpEffect::BigPaddle;
                t = 5.0;
            }
            2 => {
                self.effect = PowerUpEffect::EnemySmallPaddle;
                t = 5.0;
            }
            3 => {
                self.effect = PowerUpEffect::SlowBall;
                t = 5.0;
            }
            4 => {
                self.effect = PowerUpEffect::FastBall;
                t = 4.0;
            }
            5 => {
                self.effect = PowerUpEffect::EnemyFreeze;
                t = 3.0;
            }
            6 => {
                self.effect = PowerUpEffect::EnemyReverseControls;
                t = 4.0;
            }
            7 => {
                self.effect = PowerUpEffect::Shield;
                t = f32::INFINITY;
            }
            _ => {
                self.effect = PowerUpEffect::MultiBall;
                t = 6.0;
            }
        }
        self.rect.y = gen_range(0.0, WINDOW_H - POWERUP_SIZE);
        self.time = t;
        t
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
    fn check_apply(
        &mut self,
        balls: &mut Vec<Ball>,
        left: &mut Paddle,
        right: &mut Paddle,
        stack: &mut Vec<f32>,
    ) -> bool {
        let ball = &mut balls[0];
        let texture = ball.texture;
        if self.rect.overlaps(&ball.rect) {
            self.claimed = true;
            match self.effect {
                PowerUpEffect::BigPaddle => {
                    if ball.vel.x < 0.0 {
                        stack.push(right.rect.h);
                        stack.push(1.0);
                        right.rect.h += right.rect.h;
                    } else {
                        stack.push(left.rect.h);
                        stack.push(-1.0);
                        left.rect.h += left.rect.h;
                    }
                    return true;
                }
                PowerUpEffect::EnemySmallPaddle => {
                    if ball.vel.x < 0.0 {
                        stack.push(left.rect.h);
                        stack.push(-1.0);
                        left.rect.h /= 2.0;
                    } else {
                        stack.push(right.rect.h);
                        stack.push(1.0);
                        right.rect.h /= 2.0;
                    }
                    return true;
                }
                PowerUpEffect::SlowBall => {
                    ball.vel /= 2.0;
                    return true;
                }
                PowerUpEffect::FastBall => {
                    ball.vel *= 2.0;
                    return true;
                }
                PowerUpEffect::EnemyFreeze => {
                    if ball.vel.x < 0.0 {
                        stack.push(left.vel_multiplier);
                        stack.push(-1.0);
                        left.vel_multiplier = 0.0;
                    } else {
                        stack.push(right.vel_multiplier);
                        stack.push(1.0);
                        right.vel_multiplier = 0.0;
                    }
                    return true;
                }
                PowerUpEffect::EnemyReverseControls => {
                    if ball.vel.x < 0.0 {
                        stack.push(left.vel_multiplier);
                        stack.push(-1.0);
                        left.vel_multiplier *= -1.0;
                    } else {
                        stack.push(right.vel_multiplier);
                        stack.push(1.0);
                        right.vel_multiplier *= -1.0;
                    }
                    return true;
                }
                PowerUpEffect::Shield => {
                    stack.push(ball.vel.x);
                    return true;
                }
                PowerUpEffect::MultiBall => {
                    balls.push(Ball::new(texture));
                }
                PowerUpEffect::None => {}
            }
        }
        false
    }

    fn restore(
        &mut self,
        balls: &mut Vec<Ball>,
        left: &mut Paddle,
        right: &mut Paddle,
        stack: &mut Vec<f32>,
    ) {
        self.claimed = false;
        match self.effect {
            PowerUpEffect::BigPaddle => {
                if stack.pop().unwrap() > 0.0 {
                    right.rect.h = stack.pop().unwrap();
                } else {
                    left.rect.h = stack.pop().unwrap();
                }
            }
            PowerUpEffect::EnemySmallPaddle => {
                if stack.pop().unwrap() > 0.0 {
                    right.rect.h = stack.pop().unwrap();
                } else {
                    left.rect.h = stack.pop().unwrap();
                }
            }
            PowerUpEffect::SlowBall => {
                balls[0].vel *= 2.0;
            }
            PowerUpEffect::FastBall => {
                balls[0].vel /= 2.0;
            }
            PowerUpEffect::EnemyFreeze => {
                if stack.pop().unwrap() > 0.0 {
                    right.vel_multiplier = stack.pop().unwrap();
                } else {
                    left.vel_multiplier = stack.pop().unwrap();
                }
            }
            PowerUpEffect::EnemyReverseControls => {
                if stack.pop().unwrap() > 0.0 {
                    right.vel_multiplier = stack.pop().unwrap();
                } else {
                    left.vel_multiplier = stack.pop().unwrap();
                }
            }
            PowerUpEffect::Shield => {
                stack.pop();
            }
            PowerUpEffect::MultiBall => {
                balls.pop();
            }
            PowerUpEffect::None => {}
        }
    }
}

fn draw_balls(balls: &Vec<Ball>) {
    for ball in balls {
        ball.draw();
    }
}

fn update_balls(balls: &mut Vec<Ball>, dt: f32) {
    for ball in balls {
        ball.update(dt);
    }
}

fn balls_check_paddles(balls: &mut Vec<Ball>, left: &Paddle, right: &Paddle) {
    for ball in balls {
        ball.check_paddles(left, right);
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
    let pu_texture = load_texture("assets/background.png").await.unwrap();
    let mut balls = Vec::new();
    balls.push(Ball::new(&ball_texture));
    // let ball = &mut balls[0];

    let mut left = Paddle::new(PADDLE_OFFSET, &paddle_texture, None);
    let mut right = Paddle::new(
        WINDOW_W - PADDLE_W - PADDLE_OFFSET,
        &paddle_texture,
        Some(AI_DIFFICULTY),
    );
    let mut pu_active_flag: Option<PowerUp> = None;
    let mut pu_timer = 8.0_f32;
    let mut stack: Vec<f32> = Vec::new();
    loop {
        let dt = get_frame_time();

        match &mut game_state {
            GameState::Playing => {
                clear_background(BLACK);
                draw_centre_line();

                match &mut pu_active_flag {
                    Some(pu) => {
                        if !(pu.claimed) {
                            let subloop = balls
                                .iter()
                                .map(|r| r.vel_multiplier)
                                .fold(1.0_f32, f32::max)
                                + 1.0;
                            let substep = dt / subloop;
                            for _ in 1..(subloop as u8) {
                                pu_timer -= substep;
                                if pu_timer < 0.0 {
                                    pu.rand_gen();
                                    pu_timer = 8.0;
                                }
                                left.update(substep, KeyCode::W, KeyCode::S, &balls);
                                right.update(substep, KeyCode::Up, KeyCode::Down, &balls);
                                update_balls(&mut balls, substep);
                                balls_check_paddles(&mut balls, &left, &right);
                                if pu.check_apply(&mut balls, &mut left, &mut right, &mut stack) {
                                    pu_timer = 8.0;
                                }
                            }
                            pu.draw();
                        } else {
                            let subloop = balls
                                .iter()
                                .map(|r| r.vel_multiplier)
                                .fold(1.0_f32, f32::max)
                                + 1.0;
                            let substep = dt / subloop;
                            for _ in 1..(subloop as u8) {
                                left.update(substep, KeyCode::W, KeyCode::S, &balls);
                                right.update(substep, KeyCode::Up, KeyCode::Down, &balls);
                                update_balls(&mut balls, substep);
                                balls_check_paddles(&mut balls, &left, &right);
                            }
                            match pu.effect {
                                PowerUpEffect::Shield => {
                                    if stack.len() > 0 {
                                        if stack[0] < 0.0 {
                                            // Left player shield
                                            draw_line(
                                                PADDLE_W + PADDLE_OFFSET + SHIELD_OFFSET,
                                                0.0,
                                                PADDLE_W + PADDLE_OFFSET + SHIELD_OFFSET,
                                                WINDOW_H,
                                                2.0,
                                                SKYBLUE,
                                            );
                                        } else {
                                            draw_line(
                                                WINDOW_W - PADDLE_OFFSET - PADDLE_W - SHIELD_OFFSET,
                                                0.0,
                                                WINDOW_W - PADDLE_OFFSET - PADDLE_W - SHIELD_OFFSET,
                                                WINDOW_H,
                                                2.0,
                                                SKYBLUE,
                                            );
                                        }
                                    }
                                }
                                _ => {}
                            }
                            pu_timer -= dt;
                            if pu_timer < 8.0 - pu.time {
                                pu.restore(&mut balls, &mut left, &mut right, &mut stack);
                                pu_active_flag = None;
                            }
                        }
                    }
                    None => {
                        pu_timer -= dt;
                        if pu_timer < 0.0 {
                            let mut pu = PowerUp::new(&pu_texture);
                            pu.rand_gen();
                            pu_timer = 8.0;
                            pu_active_flag = Some(pu);
                        }
                        let subloop = balls
                            .iter()
                            .map(|r| r.vel_multiplier)
                            .fold(1.0_f32, f32::max)
                            + 1.0;
                        let substep = dt / subloop;
                        for _ in 1..(subloop as u8) {
                            left.update(substep, KeyCode::W, KeyCode::S, &balls);
                            right.update(substep, KeyCode::Up, KeyCode::Down, &balls);
                            update_balls(&mut balls, substep);
                            balls_check_paddles(&mut balls, &left, &right);
                        }
                    }
                }

                match score.update(&mut balls) {
                    Some(p) => {
                        if let Some(pu) = &mut pu_active_flag {
                            if pu.claimed {
                                pu.restore(&mut balls, &mut left, &mut right, &mut stack);
                            }
                        }
                        pu_active_flag = None;
                        pu_timer = 8.0;

                        balls[0].reset(p);
                        match p {
                            Player::Left => {
                                left.shrink((PADDLE_H - MIN_PADDLE_H) / 4.0);
                            }
                            Player::Right => {
                                right.shrink((PADDLE_H - MIN_PADDLE_H) / 4.0);
                            }
                        }
                        if score.left >= WIN_SCORE {
                            winner = "Left player wins!";
                            game_state = GameState::GameOver;
                        } else if score.right >= WIN_SCORE {
                            winner = "Right player wins!";
                            game_state = GameState::GameOver;
                        } else {
                            game_state = GameState::Stopwatch(COUNT_DOWN_TIME);
                        }
                    }
                    None => {}
                }

                left.draw();
                right.draw();
                draw_balls(&balls);
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
                    balls = Vec::new();
                    balls.push(Ball::new(&ball_texture));
                    left = Paddle::new(PADDLE_OFFSET, &paddle_texture, None);
                    right = Paddle::new(
                        WINDOW_W - PADDLE_OFFSET - PADDLE_W,
                        &paddle_texture,
                        Some(AI_DIFFICULTY),
                    );
                    game_state = GameState::Stopwatch(COUNT_DOWN_TIME);
                }
            }
            GameState::Stopwatch(time) => {
                clear_background(BLACK);
                draw_centre_line();
                left.update(dt, KeyCode::W, KeyCode::S, &balls);
                right.update(dt, KeyCode::Up, KeyCode::Down, &balls);
                left.draw();
                right.draw();
                *time -= dt;
                if *time > 0.0 {
                    let sec = f32::ceil(*time);
                    let text = format!("{sec}");
                    let dims = measure_text(&text, None, 240, 1.0);
                    draw_text(
                        &text,
                        WINDOW_W / 2.0 - dims.width / 2.0,
                        WINDOW_H / 2.0 - dims.height / 2.0,
                        240.0,
                        WHITE,
                    );
                } else {
                    game_state = GameState::Playing;
                }
            }
        }

        next_frame().await;
    }
}
