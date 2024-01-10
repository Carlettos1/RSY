use gloo::timers::callback::Interval;
use rand::Rng;
use yew::{html::Scope, prelude::*};

use crate::cells::Cell;

pub enum CellMsg {
    Random,
    Start,
    Step,
    Reset,
    Stop,
    ToggleCell(usize),
    Tick,
}

pub struct GameOfLife {
    active: bool,
    cells: Vec<Cell>,
    cell_width: usize,
    cell_height: usize,
    _interval: Interval,
}

impl GameOfLife {
    pub fn random_mutate(&mut self) {
        for cell in self.cells.iter_mut() {
            if rand::thread_rng().gen_bool(0.5) {
                cell.set_alive();
            } else {
                cell.set_dead()
            }
        }
    }

    pub fn reset(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.set_dead();
        }
    }

    pub fn step(&mut self) {
        let mut to_dead = Vec::new();
        let mut to_live = Vec::new();
        for row in 0..self.cell_height {
            for col in 0..self.cell_width {
                let neighbors = self.neighbors(row as isize, col as isize);

                let current_idx = self.row_col_as_idx(row as isize, col as isize);
                if self.cells[current_idx].is_alive() {
                    if Cell::alone(&neighbors) || Cell::overpopulated(&neighbors) {
                        to_dead.push(current_idx);
                    }
                } else if Cell::can_be_revived(&neighbors) {
                    to_live.push(current_idx);
                }
            }
        }
        for i in to_dead {
            self.cells[i].set_dead();
        }
        for i in to_live {
            self.cells[i].set_alive();
        }
    }

    fn neighbors(&self, row: isize, col: isize) -> [Cell; 8] {
        [
            self.cells[self.row_col_as_idx(row + 1, col)],
            self.cells[self.row_col_as_idx(row + 1, col + 1)],
            self.cells[self.row_col_as_idx(row + 1, col - 1)],
            self.cells[self.row_col_as_idx(row - 1, col)],
            self.cells[self.row_col_as_idx(row - 1, col + 1)],
            self.cells[self.row_col_as_idx(row - 1, col - 1)],
            self.cells[self.row_col_as_idx(row, col - 1)],
            self.cells[self.row_col_as_idx(row, col + 1)],
        ]
    }

    fn row_col_as_idx(&self, row: isize, col: isize) -> usize {
        let row = wrap(row, self.cell_height as isize);
        let col = wrap(col, self.cell_width as isize);

        row * self.cell_width + col
    }

    fn view_cell(&self, idx: usize, cell: &Cell, link: &Scope<Self>) -> Html {
        let status = if cell.is_alive() {
            "cell-live"
        } else {
            "cell-dead"
        };

        html! {
            <div key={idx} class={classes!("game-cell", status)} onclick={link.callback(move |_| CellMsg::ToggleCell(idx))}>
            </div>
        }
    }
}

impl Component for GameOfLife {
    type Message = CellMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|_| CellMsg::Tick);
        let interval = Interval::new(200, move || callback.emit(()));

        let (w, h) = (40, 30);

        Self {
            active: false,
            cells: vec![Cell::new_dead(); w * h],
            cell_width: w,
            cell_height: h,
            _interval: interval,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            CellMsg::Random => {
                self.random_mutate();
                log::info!("Random");
                true
            }
            CellMsg::Start => {
                self.active = true;
                log::info!("Start");
                false
            }
            CellMsg::Step => {
                self.step();
                true
            }
            CellMsg::Reset => {
                self.reset();
                log::info!("Reset");
                true
            }
            CellMsg::Stop => {
                self.active = false;
                log::info!("Stop");
                false
            }
            CellMsg::ToggleCell(idx) => {
                self.cells[idx].toggle();
                true
            }
            CellMsg::Tick => {
                if self.active {
                    self.step();
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let rows = self
            .cells
            .chunks(self.cell_width)
            .enumerate()
            .map(|(y, cells)| {
                let offset = y * self.cell_width;

                let cells = cells
                    .iter()
                    .enumerate()
                    .map(|(x, cell)| self.view_cell(offset + x, cell, ctx.link()));
                html! {
                    <div key={y} class="game-row">
                        { for cells }
                    </div>
                }
            });

        html! {
            <div>
                <section class="game-container">
                    <header class="app-header">
                        <img alt="The app logo" src="favicon.ico" class="app-logo"/>
                        <h1 class="app-title">{ "Game of Life" }</h1>
                    </header>
                    <section class="game-area">
                        <div class="game-of-life">
                            { for rows }
                        </div>
                        <div class="game-buttons">
                            <button class="game-button" onclick={ctx.link().callback(|_| CellMsg::Random)}>{ "Random" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| CellMsg::Step)}>{ "Step" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| CellMsg::Start)}>{ "Start" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| CellMsg::Stop)}>{ "Stop" }</button>
                            <button class="game-button" onclick={ctx.link().callback(|_| CellMsg::Reset)}>{ "Reset" }</button>
                        </div>
                    </section>
                </section>
            </div>
        }
    }
}

fn wrap(coord: isize, range: isize) -> usize {
    let result = if coord < 0 {
        coord + range
    } else if coord >= range {
        coord - range
    } else {
        coord
    };
    result as usize
}
