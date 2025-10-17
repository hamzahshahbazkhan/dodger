use macroquad::prelude::*;

struct FallingCircle {
    x: f32,
    y: f32,
    radius: f32,
    speed: f32,
}

impl FallingCircle {
    fn new() -> Self {
        let radius = 40.0;
        Self {
            radius,
            x: rand::gen_range(radius, screen_width() - radius),
            y: -50.0,
            speed: 200.0,
        }
    }

    fn update(&mut self, dt: f32) {
        self.y += self.speed * dt;
    }

    fn draw(&self) {
        draw_circle(self.x, self.y, self.radius, RED);
    }

    fn is_off_screen(&self) -> bool {
        self.y - self.radius > screen_height()
    }

    fn collides_with_player(&self, player: &Player) -> bool {
        let closest_x = self.x.clamp(player.x, player.x + player.size);
        let closest_y = self.y.clamp(player.y, player.y + player.size);

        let dx = self.x - closest_x;
        let dy = self.y - closest_y;

        dx * dx + dy * dy < self.radius * self.radius
    }
}

struct Player {
    x: f32,
    y: f32,
    size: f32,
    vel: f32,
    accl: f32,
    max_speed: f32,
}

impl Player {
    fn new() -> Self {
        Self {
            x: (screen_width() - 40.0) / 2.0,
            y: screen_height() - 40.0,
            size: 40.0,
            vel: 0.0,
            accl: 500.0,
            max_speed: 800.0,
        }
    }

    fn update(&mut self, dt: f32) {
        self.y = screen_height() - 40.0;
        if is_key_down(KeyCode::Left) {
            if self.vel > 0.0 {
                self.vel = 0.0;
            }
            self.vel -= self.accl * dt;
            // self.x -= self.speed * dt;
        } else if is_key_down(KeyCode::Right) {
            if self.vel < 0.0 {
                self.vel = 0.0;
            }
            self.vel += self.accl * dt;
            // self.x += self.speed * dt;
        }

        if !is_key_down(KeyCode::Left) && !is_key_down(KeyCode::Right) {
            self.vel = 0.0;
        }
        self.vel = self.vel.clamp(-self.max_speed, self.max_speed);
        self.x += self.vel * dt;
        self.x = self.x.clamp(0.0, screen_width() - self.size)
    }

    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.size, self.size, GREEN);
    }
}

struct Game {
    player: Player,
    circles: Vec<FallingCircle>,
    score: u32,
    game_over: bool,
    spawn_timer: f32,
    spawn_interval: f32,
}

impl Game {
    fn new() -> Self {
        Self {
            player: Player::new(),
            circles: vec![],
            score: 0,
            game_over: false,
            spawn_timer: 0.0,
            spawn_interval: 1.0,
        }
    }

    fn update(&mut self, dt: f32) {
        self.player.update(dt);
        self.check_collision(dt);
        self.update_score();
        self.spawn_timer += dt;
        if self.spawn_timer >= self.spawn_interval {
            self.spawn_circle();
            self.spawn_timer = 0.0;
        }
    }

    fn draw(&self) {
        self.player.draw();
        for circle in self.circles.iter() {
            circle.draw();
        }
        draw_text(&self.score.to_string(), 10.0, 30.0, 40.0, GREEN);
    }

    fn spawn_circle(&mut self) {
        let mut circle = FallingCircle::new();
        circle.speed += self.score as f32 * 10.0;
        circle.radius += self.score as f32 * 2.0;

        self.circles.push(circle);
        self.spawn_interval = rand::gen_range(0.8, 1.2); // this is added because the circles were
                                                         // getting clustered due to how rand
                                                         // function is implemented and thus making
                                                         // it predictable
    }

    fn check_collision(&mut self, dt: f32) {
        for circle in self.circles.iter_mut() {
            circle.update(dt);
        }
        if self
            .circles
            .iter()
            .any(|ball| ball.collides_with_player(&self.player))
        {
            self.game_over = true;
        }
    }

    fn update_score(&mut self) {
        let before = self.circles.len();
        self.circles.retain(|c| !c.is_off_screen());
        let after = self.circles.len();

        let increment = (before - after) as u32;
        self.score += increment;
    }

    fn reset(&mut self) {
        self.circles.clear();
        self.score = 0;
        self.game_over = false;
        self.spawn_timer = 0.0;
        self.player.x = (screen_width() - 40.0) / 2.0;
    }

    fn restart(&mut self) {
        if self.game_over && is_key_down(KeyCode::R) {
            self.reset();
            self.game_over = false;
        }
    }
}

#[macroquad::main("Dodger")]
async fn main() {
    let mut game = Game::new();
    loop {
        let dt = get_frame_time();
        clear_background(BLACK);

        if !game.game_over {
            game.update(dt);
            game.draw();
        } else {
            let score_text = game.score.to_string();
            let text_dimensions =
                measure_text(&format!("GAME OVER: {}", score_text), None, 40, 1.0);
            draw_text(
                &format!("GAME OVER: {}", score_text),
                screen_width() / 2.0 - text_dimensions.width / 2.0,
                screen_height() / 2.0 - text_dimensions.height / 2.0,
                40.0,
                GREEN,
            );

            draw_text("Press R to restart", 10.0, 30.0, 20.0, GREEN);
            game.restart();
        }

        next_frame().await
    }
}
