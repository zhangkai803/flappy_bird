
use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DUATION: f32 = 75.0;

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new() -> Self {
        Player { x: 5, y: 25, velocity: 0.0 }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(0, self.y, YELLOW, BLACK, to_cp437('@'))
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity as i32;
        // self.x += 1;

        if self.y < 0 {
            self.y = 0
        }
    }

    fn flap(&mut self) {
        self.velocity = -2.0;
    }
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}

impl Obstacle {
    fn new() -> Self {
        Obstacle {
            x: SCREEN_WIDTH,
            gap_y: 20,
            size: 10,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        for i in 0..self.gap_y {
            ctx.print(self.x, i, "|")
        }
        for i in self.gap_y + self.size..SCREEN_HEIGHT {
            ctx.print(self.x, i, "|")
        }
    }

    fn mov(&mut self) {
        self.x -= 1;
        if self.x == 0 {
            self.x = SCREEN_WIDTH;
        }
    }
}

enum GameMode {
    Menu,
    Play,
    End,
}

struct State {
    mode: GameMode,
    frame_time: f32,
    player: Player,
    obstacle: Obstacle,
}

impl State {
    fn new() -> Self {
        State{
            mode: GameMode::Menu,
            frame_time: 0.0,
            player: Player::new(),
            obstacle: Obstacle::new(),
        }
    }

    fn menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Bird!");
        ctx.print_centered(8, "(P) Play");
        ctx.print_centered(9, "(Q) Quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => {
                    self.restart();
                },
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {},
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);

        self.frame_time += ctx.frame_time_ms;

        if self.frame_time > FRAME_DUATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        ctx.print(0, 0, "Space to flap");
        ctx.print(0, 1, &format!("player: {} {}", self.player.x, self.player.y));
        ctx.print(0, 2, &format!("obstacle: {} {}", self.obstacle.x, self.obstacle.gap_y));

        if self.obstacle.x == self.player.x && self.player.y < self.obstacle.gap_y {
            self.mode = GameMode::End;
        }

        if self.obstacle.x == self.player.x && self.player.y > (self.obstacle.gap_y + self.obstacle.size) {
            self.mode = GameMode::End;
        }

        if self.player.y > SCREEN_HEIGHT {
            self.mode = GameMode::End;
        }

        self.player.render(ctx);
        self.obstacle.render(ctx);
        self.obstacle.mov();

    }

    fn restart(&mut self) {
        self.player = Player::new();
        self.mode = GameMode::Play;
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new();
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You Dead!");
        ctx.print_centered(8, "(P) Play");
        ctx.print_centered(9, "(Q) Quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => {
                    self.restart();
                },
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {},
            }
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello!");
        match self.mode {
            GameMode::Menu => {
                self.menu(ctx);
            },
            GameMode::Play => {
                self.play(ctx);
            },
            GameMode::End => {
                self.dead(ctx);
            }
        }
    }
}

fn main() -> BError{
    let ctx = BTermBuilder::simple80x50().with_title("Flappy Bird").build()?;

    main_loop(ctx, State::new())
}
