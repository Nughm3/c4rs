use ggez::{
    conf::{WindowMode, WindowSetup},
    event::{run, EventHandler, KeyCode, MouseButton},
    graphics::{self, Color, DrawMode, DrawParam, Font, MeshBuilder, PxScale, Rect, Text},
    mint::Point2,
    Context, ContextBuilder, GameResult,
};
use std::fmt;
use std::process::exit;
use {Tile::*, Turn::*};

const X_OFFSET: usize = 330; // 1280 / 2 - (90 * 7 - 10) / 2
const Y_OFFSET: usize = 95; // 720 / 2 - (90 * 6 - 10) / 2
const TILE_SIZE: usize = 90;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Red,
    Green,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Turn {
    Player1, // red
    Player2, // green
}

impl From<Turn> for Color {
    fn from(t: Turn) -> Self {
        match t {
            Player1 => hex("e06c75"),
            Player2 => hex("98c379"),
        }
    }
}

impl fmt::Display for Turn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Player1 => "Player 1",
                Player2 => "Player 2",
            }
        )
    }
}

struct State {
    board: Vec<Vec<Tile>>,
    turn: Turn,
    col: Option<usize>,
    active: bool,
    round: u32,
    winner: Option<Turn>,
    font: Font,
}

impl State {
    fn new(ctx: &mut Context) -> Self {
        // WARNING: This 2D array is a list of columns, not rows
        // A lot of transposing is done as well
        let board = vec![vec![Empty; 6]; 7];
        let font = Font::new(ctx, "/iosevka.ttf").expect("Failed to load font");
        State {
            board,
            turn: Player1,
            col: None,
            active: false,
            round: 1,
            winner: None,
            font,
        }
    }

    fn clear(&mut self) {
        self.board = vec![vec![Empty; 6]; 7];
        self.winner = None;
    }

    fn winner(&self) -> Option<Turn> {
        let board = &self.board;

        for col in board {
            for range in [0..3, 1..4, 2..5] {
                let segment = &col[range];
                if segment.iter().all(|tile| *tile == Red) {
                    return Some(Player1);
                }
                if segment.iter().all(|tile| *tile == Green) {
                    return Some(Player2);
                }
            }
        }

        let board: Vec<Vec<Tile>> = (0..board[0].len())
            .map(|i| board.iter().map(|c| c[i]).collect())
            .collect();

        for row in &board {
            for range in [0..4, 1..5, 2..6, 3..7] {
                let segment = &row[range];
                if segment.iter().all(|tile| *tile == Red) {
                    return Some(Player1);
                }
                if segment.iter().all(|tile| *tile == Green) {
                    return Some(Player2);
                }
            }
        }

        for (a, y) in board.iter().enumerate() {
            for (b, x) in y.iter().enumerate() {
                if a + 3 < board.len() && b + 3 < board[0].len() {
                    if [*x; 3]
                        == [
                            board[a + 1][b + 1],
                            board[a + 2][b + 2],
                            board[a + 3][b + 3],
                        ]
                    {
                        match x {
                            Red => return Some(Player1),
                            Green => return Some(Player2),
                            Empty => {}
                        }
                    }
                }

                if a.checked_sub(3).is_some() && b.checked_sub(3).is_some() {
                    if [*x; 3]
                        == [
                            board[a - 1][b - 1],
                            board[a - 2][b - 2],
                            board[a - 3][b - 3],
                        ]
                    {
                        match x {
                            Red => return Some(Player1),
                            Green => return Some(Player2),
                            Empty => {}
                        }
                    }
                }

                if a + 3 < board.len() && b.checked_sub(3).is_some() {
                    if [*x; 3]
                        == [
                            board[a + 1][b - 1],
                            board[a + 2][b - 2],
                            board[a + 3][b - 3],
                        ]
                    {
                        match x {
                            Red => return Some(Player1),
                            Green => return Some(Player2),
                            Empty => {}
                        }
                    }
                }

                if a.checked_sub(3).is_some() && b + 3 < board[0].len() {
                    if [*x; 3]
                        == [
                            board[a - 1][b + 1],
                            board[a - 2][b + 2],
                            board[a - 3][b + 3],
                        ]
                    {
                        match x {
                            Red => return Some(Player1),
                            Green => return Some(Player2),
                            Empty => {}
                        }
                    }
                }
            }
        }

        None
    }
}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, hex("1e222a"));

        let stroke = DrawMode::stroke(2.0);
        let fill = DrawMode::fill();

        let mut mb = MeshBuilder::new();

        for (a, r) in self.board.iter().enumerate() {
            for (b, c) in r.iter().enumerate() {
                let (x, y) = (
                    (a * TILE_SIZE + X_OFFSET) as f32,
                    (b * TILE_SIZE + Y_OFFSET) as f32,
                );
                mb.rounded_rectangle(
                    stroke,
                    Rect::new(x, y, TILE_SIZE as f32 - 10.0, TILE_SIZE as f32 - 10.0),
                    10.0,
                    if !self.active {
                        hex("3e4451")
                    } else if self.col == Some(a) {
                        Color::WHITE
                    } else {
                        hex("abb2bf")
                    },
                )?;

                let center = Point2 {
                    x: x + 40.0,
                    y: y + 40.0,
                };
                match c {
                    Red => {
                        mb.circle(fill, center, 32.0, 0.001, hex("e06c75"))?;
                    }
                    Green => {
                        mb.circle(fill, center, 32.0, 0.001, hex("98c379"))?;
                    }
                    Empty => {}
                }
            }
        }

        if self.active {
            let turn = Text::new(format!("{}'s turn", self.turn.to_string()))
                .set_font(self.font, PxScale::from(24.0))
                .to_owned();
            let turn_width = turn.width(ctx);

            let round = Text::new(format!("Turn {}", self.round))
                .set_font(self.font, PxScale::from(24.0))
                .to_owned();
            let round_width = round.width(ctx);

            graphics::queue_text(
                ctx,
                &turn,
                Point2 {
                    x: 640.0 - turn_width / 2.0,
                    y: 660.0,
                },
                Some(self.turn.into()),
            );

            graphics::queue_text(
                ctx,
                &round,
                Point2 {
                    x: 640.0 - round_width / 2.0,
                    y: 40.0,
                },
                Some(hex("c8ccd4")),
            );
        } else {
            let text = if let Some(w) = self.winner {
                Text::new(format!("{} won!", w.to_string()))
                    .set_font(self.font, PxScale::from(24.0))
                    .to_owned()
            } else {
                Text::new("Click to start")
                    .set_font(self.font, PxScale::from(24.0))
                    .to_owned()
            };
            let width = text.width(ctx);

            graphics::queue_text(
                ctx,
                &text,
                Point2 {
                    x: 640.0 - width / 2.0,
                    y: 660.0,
                },
                Some(hex("e5c07b")),
            );
        }

        let mesh = mb.build(ctx)?;
        graphics::draw(ctx, &mesh, DrawParam::default())?;
        graphics::draw_queued_text(
            ctx,
            DrawParam::default(),
            None,
            graphics::FilterMode::Linear,
        )?;
        graphics::present(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        if self.active {
            if button == MouseButton::Left && self.col.is_some() {
                self.round += 1;
                let mut board: Vec<Vec<_>> = (0..self.board[0].len())
                    .map(|i| self.board.iter().map(|c| c[i]).collect())
                    .collect();
                let col = self.col.unwrap();
                let lowest_empty = board.iter_mut().rev().find(|row| row[col] == Empty);
                if let Some(lowest) = lowest_empty {
                    self.turn = match self.turn {
                        Player1 => {
                            lowest[col] = Red;
                            Player2
                        }
                        Player2 => {
                            lowest[col] = Green;
                            Player1
                        }
                    };
                    self.board = (0..board[0].len())
                        .map(|i| board.iter().map(|c| c[i]).collect())
                        .collect();

                    if let Some(w) = self.winner() {
                        self.winner = Some(w);
                        self.active = false;
                    }

                    if board
                        .iter()
                        .find(|inner| inner.iter().find(|t| **t == Empty).is_some())
                        .is_none()
                    {
                        self.active = false;
                    }
                }
            }
        } else {
            self.clear();
            self.active = true;
        }
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        let (x, y) = (x as usize, y as usize);
        self.col = if x >= X_OFFSET && x <= X_OFFSET + 620 && y >= Y_OFFSET && y <= Y_OFFSET + 530 {
            Some(match x {
                330..=415 => 0,
                416..=500 => 1,
                501..=585 => 2,
                586..=670 => 3,
                671..=755 => 4,
                756..=840 => 5,
                841..=950 => 6,
                _ => unreachable!(),
            })
        } else {
            None
        };
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: ggez::event::KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Q => exit(0),
            _ => {}
        }
    }
}

fn main() -> GameResult {
    let (mut ctx, el) = ContextBuilder::new("Connect 4", "Nughm3")
        .window_mode(WindowMode::default().dimensions(1280.0, 720.0))
        .window_setup(WindowSetup::default().title("Connect 4"))
        .build()?;

    let game = State::new(&mut ctx);
    run(ctx, el, game)
}

/// Create a color from a hexadecimal
fn hex(s: &str) -> Color {
    let r = u8::from_str_radix(&s[0..2], 16).expect("Failed parsing color") as f32 / 255.0;
    let g = u8::from_str_radix(&s[2..4], 16).expect("Failed parsing color") as f32 / 255.0;
    let b = u8::from_str_radix(&s[4..6], 16).expect("Failed parsing color") as f32 / 255.0;

    Color { r, g, b, a: 1.0 }
}
