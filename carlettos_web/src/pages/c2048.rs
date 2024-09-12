use std::ops::AddAssign;

use c2048_leaderboard::C2048Leaderboard;
use csta::prelude::*;
use csta_derive::Randomizable;
use rand::prelude::*;
use yew::prelude::*;

const L: usize = 4;

pub mod c2048_leaderboard;

#[derive(Debug, Default, PartialEq)]
pub struct Energy {
    epsilon: isize,
    phi: isize,
    xi: isize,
}

#[derive(Debug, Default)]
pub struct FullEnergy {
    epsilon: isize,
    phi_up: Option<isize>,
    phi_down: Option<isize>,
    phi_left: Option<isize>,
    phi_right: Option<isize>,
    xi_vertical: Option<isize>,
    xi_horizontal: Option<isize>,
}

impl FullEnergy {
    pub fn reduce(self) -> Energy {
        Energy {
            epsilon: self.epsilon,
            phi: self.phi_down.unwrap_or_default()
                + self.phi_up.unwrap_or_default()
                + self.phi_left.unwrap_or_default()
                + self.phi_right.unwrap_or_default(),
            xi: self.xi_horizontal.unwrap_or_default() + self.xi_vertical.unwrap_or_default(),
        }
    }
}

impl Energy {
    pub fn sum(&self) -> isize {
        self.epsilon + self.phi + self.xi
    }
}

impl AddAssign for Energy {
    fn add_assign(&mut self, rhs: Self) {
        self.epsilon += rhs.epsilon;
        self.phi += rhs.phi;
        self.xi += rhs.xi;
    }
}

#[derive(Clone, Debug, Randomizable, PartialEq, PartialOrd)]
pub enum Move {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Default, Clone, Randomizable)]
pub struct Tile {
    #[rng(default)]
    pub exp: u8,
    #[rng(default)]
    pub is_merged: bool,
}

impl Eq for Tile {}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.exp.eq(&other.exp)
    }
}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.exp.cmp(&other.exp)
    }
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[macro_export]
macro_rules! avance {
    ($self:tt, $from:tt, $to:tt) => {
        if ($self.grid[$to].exp == $self.grid[$from].exp
            && (!$self.grid[$to].is_merged && !$self.grid[$from].is_merged))
        {
            $self.grid[$to].exp += 1;
            $self.grid[$from].exp = 0;
            $self.grid[$to].is_merged = true;
            $self.has_moved = true;
        } else if ($self.grid[$to].exp == 0) {
            $self.grid[$to].exp = $self.grid[$from].exp;
            $self.grid[$to].is_merged = $self.grid[$from].is_merged;
            $self.grid[$from].exp = 0;
            $self.grid[$from].is_merged = false;
            $self.has_moved = true;
        } else {
            break;
        }
    };
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum C2048Msg {
    Move(Move),
    ClickTile(usize),
    SeeEpsilon,
    SeePhi,
    SeeXi,
    Nothing,
    Automove,
    Reset,
    Touch((i32, i32)),
    TouchEnd,
}

#[derive(Debug, Clone, Default)]
pub enum Mode {
    #[default]
    None,
    Epsilon,
    Phi,
    Xi,
}

#[derive(Debug, Clone, Default)]
pub struct C2048 {
    pub grid: [Tile; L * L],
    pub has_moved: bool,
    pub selected: Option<usize>,
    pub mode: Mode,
    pub touched: bool,
    pub touch: Option<(i32, i32)>,
    pub automoved: bool,
    pub show_leaderboard: bool,
    pub energies: Vec<isize>,
    pub score: usize,
}

impl Randomizable for C2048 {
    fn sample<D: Distribution<f64>, R: Rng + ?Sized>(_: &D, rng: &mut R) -> Self {
        C2048::new(rng)
    }
}

impl C2048 {
    pub fn new<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let mut c2048 = Self::default();
        c2048.spawn_tile(rng, 0.0);
        c2048.spawn_tile(rng, 0.0);
        c2048.energies.push(c2048.energy().sum());
        c2048
    }

    pub fn spawn_tile<R: Rng + ?Sized>(&mut self, rng: &mut R, chance: f64) {
        self.energies.push(self.energy().sum());
        let random_exp = if rng.gen_bool(chance) { 2 } else { 1 };

        let random_tile = self
            .grid
            .iter_mut()
            .filter(|tile| tile.exp == 0)
            .choose(rng);
        if let Some(tile) = random_tile {
            tile.exp = random_exp;
            self.energies.push(self.energy().sum());
        }
    }

    pub fn _set_tile(&mut self, pos: usize, exp: u8) {
        //unsafe { self.grid.get_unchecked_mut(pos).exp = exp };
        self.grid[pos].exp = exp;
    }

    pub fn avg_energy(&self) -> isize {
        self.energies.iter().sum::<isize>() / self.energies.len() as isize
    }

    pub fn highest(&self) -> &Tile {
        self.grid.iter().max().unwrap()
    }

    pub fn is_lose(&self) -> bool {
        if self.grid.iter().any(|tile| tile.exp == 0) {
            return false;
        }

        for x in 0..L - 1 {
            for y in 0..L - 1 {
                let i = x + y * L;
                if self.grid[i] == self.grid[i + 1] || self.grid[i] == self.grid[i + L] {
                    return false;
                }
                let i = L - 1 + y * L;
                if self.grid[i] == self.grid[i + L] {
                    return false;
                }
                let i = x + (L - 1) * L;
                if self.grid[i] == self.grid[i + 1] {
                    return false;
                }
            }
        }
        true
    }

    pub fn full_energy_at(&self, i: usize) -> FullEnergy {
        let mut e = FullEnergy::default();
        let exp = self.grid[i].exp;
        if exp == 0 {
            e.epsilon = -1;
            return e;
        } else {
            e.epsilon = exp as isize;
        }
        let iexp = exp as isize;

        let x = i % L;
        let y = i / L;
        let right = if x + 1 < L {
            Some(&self.grid[i + 1])
        } else {
            None
        };
        let left = if x > 0 { Some(&self.grid[i - 1]) } else { None };
        let up = if y + 1 < L {
            Some(&self.grid[i + L])
        } else {
            None
        };
        let down = if y > 0 { Some(&self.grid[i - L]) } else { None };

        if let Some(left) = left {
            if left.exp == exp {
                e.phi_left = Some(-iexp);
            } else {
                e.phi_left = Some(iexp);
            }
        }
        if let Some(right) = right {
            if right.exp == exp {
                e.phi_right = Some(-iexp);
            } else {
                e.phi_right = Some(iexp);
            }
        }
        if let Some(up) = up {
            if up.exp == exp {
                e.phi_up = Some(-iexp);
            } else {
                e.phi_up = Some(iexp);
            }
        }
        if let Some(down) = down {
            if down.exp == exp {
                e.phi_down = Some(-iexp);
            } else {
                e.phi_down = Some(iexp);
            }
        }

        if let (Some(up), Some(down)) = (up, down) {
            let up = up.exp;
            let down = down.exp;

            if (up == exp + 1 && down == exp - 1) || (up == exp - 1 && down == exp + 1) {
                e.xi_vertical = Some(-iexp);
            } else {
                e.xi_vertical = Some(iexp);
            }
        }

        if let (Some(left), Some(right)) = (left, right) {
            let left = left.exp;
            let right = right.exp;

            if (left == exp + 1 && right == exp - 1) || (left == exp - 1 && right == exp + 1) {
                e.xi_horizontal = Some(-iexp);
            } else {
                e.xi_horizontal = Some(iexp);
            }
        }

        e
    }

    pub fn energy_at(&self, i: usize) -> Energy {
        self.full_energy_at(i).reduce()
    }

    pub fn energy(&self) -> Energy {
        let mut energy = Energy::default();
        for x in 0..L {
            for y in 0..L {
                let i = x + y * L;
                energy += self.energy_at(i);
            }
        }
        energy
    }

    pub fn reset(&mut self) {
        for tile in self.grid.iter_mut() {
            if tile.is_merged {
                self.score += 1 << tile.exp;
            }
            tile.is_merged = false;
        }
        self.has_moved = false;
    }

    pub fn clone_move(&self, mv: Move) -> Self {
        let mut clone = self.clone();
        match mv {
            Move::Up => clone.up(),
            Move::Right => clone.right(),
            Move::Down => clone.down(),
            Move::Left => clone.left(),
        }
        clone
    }

    pub fn left(&mut self) {
        for y in 0..L {
            for x in 1..L {
                let i = x + y * L;
                if self.grid[i].exp == 0 {
                    continue;
                }

                for c in 0..x {
                    let from = i - c;
                    let to = i - c - 1;
                    avance!(self, from, to);
                }
            }
        }
    }

    pub fn right(&mut self) {
        for y in 0..L {
            for x in (0..L - 1).rev() {
                let i = x + y * L;
                if self.grid[i].exp == 0 {
                    continue;
                }

                for c in 0..=2 - x {
                    let from = i + c;
                    let to = i + c + 1;
                    avance!(self, from, to);
                }
            }
        }
    }

    pub fn up(&mut self) {
        for x in 0..L {
            for y in (0..L - 1).rev() {
                let i = x + y * L;
                if self.grid[i].exp == 0 {
                    continue;
                }

                for c in 0..=2 - y {
                    let from = i + c * L;
                    let to = i + (c + 1) * L;
                    avance!(self, from, to);
                }
            }
        }
    }

    pub fn down(&mut self) {
        for x in 0..L {
            for y in 1..L {
                let i = x + y * L;
                if self.grid[i].exp == 0 {
                    continue;
                }

                for c in 0..y {
                    let from = i - c * L;
                    let to = i - (c + 1) * L;
                    avance!(self, from, to);
                }
            }
        }
    }

    fn show_energy(&self, id: usize) -> Html {
        let e = self.full_energy_at(id);
        html! {
            <div class="show-energy">
                <div class="energy-epsilon">
                    { e.epsilon }
                </div>
                <div class={classes!("energy-phi", "phi-up")}>
                    { e.phi_up }
                </div>
                <div class={classes!("energy-phi", "phi-down")}>
                    { e.phi_down }
                </div>
                <div class={classes!("energy-phi", "phi-left")}>
                    { e.phi_left }
                </div>
                <div class={classes!("energy-phi", "phi-right")}>
                    { e.phi_right }
                </div>
                <div class={classes!("energy-xi", "xi-vertical")}>
                    { e.xi_vertical } { if e.xi_vertical.is_some() { "↑" } else { "" } }
                </div>
                <div class={classes!("energy-xi", "xi-horizontal")}>
                    { e.xi_horizontal } { if e.xi_horizontal.is_some() { "→" } else { "" } }
                </div>
            </div>
        }
    }
}

impl PartialEq for C2048 {
    fn eq(&self, other: &Self) -> bool {
        self.energy().sum().eq(&other.energy().sum())
    }
}

impl PartialOrd for C2048 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for C2048 {}

impl Ord for C2048 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.energy().sum().cmp(&other.energy().sum())
    }
}

impl Component for C2048 {
    type Message = C2048Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::sample_uniform(&mut thread_rng())
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        if msg == C2048Msg::TouchEnd {
            self.touched = false;
        }
        if self.touched || msg == C2048Msg::Nothing {
            return false;
        }
        match msg {
            C2048Msg::Move(movement) => {
                match movement {
                    Move::Down => self.down(),
                    Move::Left => self.left(),
                    Move::Right => self.right(),
                    Move::Up => self.up(),
                }
                if self.has_moved {
                    self.spawn_tile(&mut thread_rng(), 0.1);
                    self.reset();
                    if self.is_lose() && !self.automoved {
                        self.show_leaderboard = true;
                    }
                }
                self.selected = None;
                self.mode = Mode::None;
            }
            C2048Msg::Touch(touch) => {
                if let Some((x, y)) = self.touch {
                    let dx = touch.0 - x;
                    let dy = touch.1 - y;
                    if dx*dx + dy*dy >= 50*50 {
                        match dx.abs() > dy.abs() {
                            true if dx > 0 => self.right(),
                            true => self.left(),
                            false if dy > 0 => self.up(),
                            false => self.down(),
                        }
                        if self.has_moved {
                            self.spawn_tile(&mut thread_rng(), 0.1);
                            self.reset();
                            if self.is_lose() && !self.automoved {
                                self.show_leaderboard = true;
                            }
                        }
                        self.selected = None;
                        self.mode = Mode::None;
                        self.touched = true;
                        self.touch = None;
                    }
                } else {
                    self.touch = Some(touch);
                }
            }
            C2048Msg::ClickTile(idx) => {
                self.selected = match self.selected {
                    Some(id) if id == idx => None,
                    _ => Some(idx),
                };
                self.mode = Mode::None;
            }
            C2048Msg::SeeEpsilon => {
                self.selected = None;
                self.mode = match self.mode {
                    Mode::Epsilon => Mode::None,
                    _ => Mode::Epsilon,
                };
            }
            C2048Msg::SeePhi => {
                self.selected = None;
                self.mode = match self.mode {
                    Mode::Phi => Mode::None,
                    _ => Mode::Phi,
                };
            }
            C2048Msg::SeeXi => {
                self.selected = None;
                self.mode = match self.mode {
                    Mode::Xi => Mode::None,
                    _ => Mode::Xi,
                };
            }
            C2048Msg::Automove => {
                let down = self.clone_move(Move::Down);
                let up = self.clone_move(Move::Up);
                let left = self.clone_move(Move::Left);
                let right = self.clone_move(Move::Right);
                let moves = vec![down, up, left, right];
                let min = moves.into_iter().filter(|g| g.has_moved).min();
                if let Some(min) = min {
                    self.automoved = true;
                    *self = min;
                    self.spawn_tile(&mut thread_rng(), 0.1);
                    self.reset();
                    self.selected = None;
                    self.mode = Mode::None;
                }
            }
            C2048Msg::Reset => {
                *self = Self::new(&mut thread_rng());
                self.automoved = false;
            }
            C2048Msg::TouchEnd => {
                self.touched = false;
            }
            C2048Msg::Nothing => (),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let game = self.grid.chunks(L).enumerate().map(|(y, row)| {
            let offset = y * L;

            let row = row.iter().enumerate().map(|(x, tile)| {
                let number: usize = 1 << tile.exp;
                let i = x + offset;

                let tile = match self.mode {
                    Mode::None => match self.selected {
                        Some(id) if id == i => self.show_energy(id),
                        _ if number != 1 => html! { number },
                        _ => html! {}
                    },
                    Mode::Epsilon => {
                        let energy = self.full_energy_at(i);
                        html! { { energy.epsilon } }
                    },
                    Mode::Phi => {
                        let energy = self.full_energy_at(i);

                        html! {
                            <div class="show-energy">
                                <div class={classes!("energy-phi", "phi-up", "only-phi")}>
                                    { energy.phi_up }
                                </div>
                                <div class={classes!("energy-phi", "phi-down", "only-phi")}>
                                    { energy.phi_down }
                                </div>
                                <div class={classes!("energy-phi", "phi-left", "only-phi")}>
                                    { energy.phi_left }
                                </div>
                                <div class={classes!("energy-phi", "phi-right", "only-phi")}>
                                    { energy.phi_right }
                                </div>
                            </div>
                        }
                    },
                    Mode::Xi => {
                        let e = self.full_energy_at(i);
                        
                        html! {
                            <div class="show-energy">
                                <div class={classes!("energy-xi", "xi-vertical")}>
                                    { e.xi_vertical } { if e.xi_vertical.is_some() { "↑" } else { "" } }
                                </div>
                                <div class={classes!("energy-xi", "xi-horizontal")}>
                                    { e.xi_horizontal } { if e.xi_horizontal.is_some() { "→" } else { "" } }
                                </div>
                            </div>
                        }
                    },
                };

                html! {
                    <div key={i} class={classes!("c2048-number", format!("c2048-number-{}", number))} onclick={ctx.link().callback(move |_| C2048Msg::ClickTile(i))}>
                    { tile }
                    </div>
                }
            });
            html! {
                <div key={y} class="c2048-row">
                    { for row }
                </div>
            }
        });

        let lose = self.is_lose();
        let cb = ctx.link().callback(move |kbe: KeyboardEvent| {
            if lose {
                C2048Msg::Nothing
            } else if kbe.key() == *"d" || kbe.key() == *"d" || kbe.key() == *"ArrowRight" {
                C2048Msg::Move(Move::Right)
            } else if kbe.key() == *"S" || kbe.key() == *"s" || kbe.key() == *"ArrowDown" {
                C2048Msg::Move(Move::Up)
            } else if kbe.key() == *"W" || kbe.key() == *"w" || kbe.key() == *"ArrowUp" {
                C2048Msg::Move(Move::Down)
            } else if kbe.key() == *"A" || kbe.key() == *"a" || kbe.key() == *"ArrowLeft" {
                C2048Msg::Move(Move::Left)
            } else if kbe.key() == *"P" || kbe.key() == *"p" {
                C2048Msg::Automove
            } else if kbe.key() == *"R" || kbe.key() == *"r" {
                C2048Msg::Reset
            } else {
                log::info!("Inputeado {}", kbe.key());
                C2048Msg::Nothing
            }
        });

        let tcb = ctx.link().callback(|te: TouchEvent|  {
            let touches = te.touches();
            let first = touches.get(0);
            if let Some(touch) = first {
                C2048Msg::Touch((touch.page_x(), touch.page_y()))
            } else {
                C2048Msg::Nothing
            }
        });

        let ote = ctx.link().callback(|_te: TouchEvent|  {
            C2048Msg::TouchEnd
        });

        html! {
            <div onkeydown={cb} tabIndex="0" class="c2048">
                <section class="c2048-container">
                    <h2 class="c2048-score"> {format!("Score: {}", self.score)} </h2>
                    <div ontouchmove={tcb} ontouchend={ote} class="c2048-game">
                        { for game }
                    </div>
                    <div class="c2048-buttons">
                        <button class="c2048-button" onclick={ctx.link().callback(|_| C2048Msg::SeeEpsilon)}>{ "ε" }</button>
                        <button class="c2048-button" onclick={ctx.link().callback(|_| C2048Msg::SeePhi)}>{ "φ" }</button>
                        <button class="c2048-button" onclick={ctx.link().callback(|_| C2048Msg::SeeXi)}>{ "ξ" }</button>
                    </div>
                    <div class="c2048-buttons">
                        <button class="c2048-button" onclick={ctx.link().callback(|_| C2048Msg::Automove)}>{ "auto" }</button>
                        <button class="c2048-button" onclick={ctx.link().callback(|_| C2048Msg::Reset)}>{ "reset" }</button>
                    </div>
                    <div class="c2048-energy-container">
                        <div class="c2048-energy">
                            { format!("Current Energy: {:?}", self.energy().sum()) }
                        </div>
                        <div class="c2048-energy">
                            { format!("ε: {}", self.energy().epsilon) }
                        </div>
                        <div class="c2048-energy">
                            { format!("φ: {}", self.energy().phi) }
                        </div>
                        <div class="c2048-energy">
                            { format!("ξ: {}", self.energy().xi) }
                        </div>
                    </div>
                </section>
                <C2048Leaderboard show_leaderboard={self.show_leaderboard} score={self.score} max_tile={1 << self.highest().exp} max_energy={self.energies.iter().max().unwrap()} avg_energy={self.avg_energy()}/>
            </div>
        }
    }
}
