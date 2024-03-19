use std::ops::AddAssign;

use csta::prelude::*;
use csta_derive::Randomizable;
use rand::prelude::*;
use yew::prelude::*;

const L: usize = 4;

#[derive(Debug, Default)]
pub struct Energy {
    epsilon: isize,
    phi: isize,
    xi: isize,
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

#[derive(Clone, Debug, Randomizable)]
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

pub enum C2048Msg {
    Move(Move),
    ClickTile(usize),
    SeeEpsilon,
    SeePhi,
    SeeXi,
    Nothing,
}

#[derive(Debug, Clone, Default)]
pub struct C2048 {
    pub grid: [Tile; L * L],
    pub has_moved: bool,
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
        c2048
    }

    pub fn spawn_tile<R: Rng + ?Sized>(&mut self, rng: &mut R, chance: f64) {
        let random_exp = if rng.gen_bool(chance) { 2 } else { 1 };

        let random_tile = self
            .grid
            .iter_mut()
            .filter(|tile| tile.exp == 0)
            .choose(rng);
        if let Some(tile) = random_tile {
            tile.exp = random_exp;
        }
    }

    pub fn set_tile(&mut self, pos: usize, exp: u8) {
        //unsafe { self.grid.get_unchecked_mut(pos).exp = exp };
        self.grid[pos].exp = exp;
    }

    pub fn highest(&self) -> &Tile {
        self.grid.iter().max().unwrap()
    }

    pub fn score(&self) -> usize {
        self.grid.iter().map(|t| 1 << t.exp).sum()
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

    pub fn energy_at(&self, i: usize) -> Energy {
        let mut e = Energy::default();
        let exp = self.grid[i].exp;
        if exp == 0 {
            return Energy {
                epsilon: -1,
                ..Default::default()
            };
        } else {
            e.epsilon = exp as isize;
        }
        let iexp = exp as isize;

        let left = i.checked_sub(1).and_then(|i| self.grid.get(i));
        let right = self.grid.get(i + 1);
        let down = i.checked_sub(L).and_then(|i| self.grid.get(i));
        let up = self.grid.get(i + L);

        let x = [left, right];
        let y = [up, down];
        let directions = [x, y];

        for other in directions.iter().flatten().flatten() {
            if other.exp == exp {
                e.phi -= iexp;
            } else {
                e.phi += iexp;
            }
        }

        for axis in directions {
            if let [Some(j), Some(k)] = axis {
                let j = j.exp;
                let k = k.exp;

                if (j == exp + 1 && k == exp - 1) || (j == exp - 1 && k == exp + 1) {
                    e.xi -= iexp;
                } else {
                    e.xi += iexp;
                }
            }
        }
        e
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

    fn create(ctx: &Context<Self>) -> Self {
        Self::sample_uniform(&mut thread_rng())
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            C2048Msg::Move(movement) => {
                match movement {
                    Move::Down => self.down(),
                    Move::Left => self.left(),
                    Move::Right => self.right(),
                    Move::Up => self.up(),
                }
                self.spawn_tile(&mut thread_rng(), 0.1);
                self.reset();
            }
            C2048Msg::ClickTile(idx) => {
                // TODO: handle click
                let tile = &self.grid[idx];
            }
            C2048Msg::SeeEpsilon => (),
            C2048Msg::SeePhi => (),
            C2048Msg::SeeXi => (),
            C2048Msg::Nothing => (),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let game = self.grid.chunks(L).enumerate().map(|(y, row)| {
            let offset = y * L;

            let row = row.iter().enumerate().map(|(x, tile)| {
                let number: usize = 1 << tile.exp;

                html! {
                    <div key={x + offset} class={classes!("c2048-number", format!("c2048-number-{}", number))} onclick={ctx.link().callback(move |_| C2048Msg::ClickTile(x + offset))}>
                    { if number != 1 {
                        html! {number}
                    } else { html! { } } }
                    </div>
                }
            });
            html! {
                <div key={y} class="c2048-row">
                    { for row }
                </div>
            }
        });

        let cb = ctx.link().callback(|kbe: KeyboardEvent| {
            if kbe.key() == *"d" || kbe.key() == *"d" {
                C2048Msg::Move(Move::Right)
            } else if kbe.key() == *"S" || kbe.key() == *"s" {
                C2048Msg::Move(Move::Up)
            } else if kbe.key() == *"W" || kbe.key() == *"w" {
                C2048Msg::Move(Move::Down)
            } else if kbe.key() == *"A" || kbe.key() == *"a" {
                C2048Msg::Move(Move::Left)
            } else {
                log::info!("Inputeado");
                C2048Msg::Nothing
            }
        });

        html! {
            <div onkeydown={cb} tabIndex="-1" class="c2048">
                <section class="c2048-container">
                    <div class="c2048-game">
                        { for game }
                    </div>
                    <div class="c2048-buttons">
                        <button class="c2048-button" onclick={ctx.link().callback(|_| C2048Msg::SeeEpsilon)}>{ "ε" }</button>
                        <button class="c2048-button" onclick={ctx.link().callback(|_| C2048Msg::SeePhi)}>{ "φ" }</button>
                        <button class="c2048-button" onclick={ctx.link().callback(|_| C2048Msg::SeeXi)}>{ "ξ" }</button>
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
            </div>
        }
    }
}
