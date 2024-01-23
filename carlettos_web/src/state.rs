use carlettos_chess::Action;
use chess_api::{Board, Color};
use yew::Reducible;

use crate::models::Task;

pub enum ChessAction {
    Get(Board),
    Update(Board),
}

#[derive(Default)]
pub struct ChessState {
    pub board: Board,
    pub winner: Option<Color>,
}

impl Reducible for ChessState {
    type Action = ChessAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let next_chess = match action {
            ChessAction::Update(board) => board,
            ChessAction::Get(board) => board,
        };

        let winner = if next_chess.is_check_mate(&Color::Black) {
            Some(Color::White)
        } else if next_chess.is_check_mate(&Color::White) {
            Some(Color::Black)
        } else {
            None
        };

        Self {
            board: next_chess,
            winner,
        }
        .into()
    }
}

pub enum CarlettosChessAction {
    Start,
    OnClick(carlettos_chess::prelude::Pos),
}

#[derive(Default)]
pub struct CarlettosChessState {
    pub board: carlettos_chess::prelude::Board,
    pub selected: Option<carlettos_chess::prelude::Pos>,
    pub moves: Vec<carlettos_chess::prelude::Pos>,
    pub takes: Vec<carlettos_chess::prelude::Pos>,
    pub attacks: Vec<carlettos_chess::prelude::Pos>,
}

impl Reducible for CarlettosChessState {
    type Action = CarlettosChessAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            CarlettosChessAction::Start => Self {
                board: carlettos_chess::prelude::Board::default_chessboard(),
                ..Default::default()
            },
            CarlettosChessAction::OnClick(pos) if self.selected.is_none() => {
                if self.board.get(&pos).is_none() {
                    return self;
                }
                let b = self.board.clone();
                let selected = Some(pos.clone());
                let mut moves = vec![];
                let mut takes = vec![];
                let mut attacks = vec![];
                let piece = &b.get(&pos).unwrap().piece;
                for to in b.shape().points_iter() {
                    let move_action = Action::Move {
                        from: pos.clone(),
                        to: to.clone(),
                    };
                    let take_action = Action::Take {
                        from: pos.clone(),
                        to: to.clone(),
                    };
                    let attack_action = Action::Attack {
                        from: pos.clone(),
                        to: to.clone(),
                    };
                    if piece.can_do(&b, move_action) {
                        moves.push(to.clone());
                    }
                    if piece.can_do(&b, take_action) {
                        takes.push(to.clone());
                    }
                    if piece.can_do(&b, attack_action) {
                        attacks.push(to.clone());
                    }
                }
                Self {
                    board: b,
                    selected,
                    moves,
                    takes,
                    attacks,
                }
            }
            CarlettosChessAction::OnClick(to) => {
                let mut board = self.board.clone();
                if self.attacks.contains(&to) {
                    board.make(Action::Attack {
                        from: self.selected.clone().unwrap(),
                        to: to.clone(),
                    })
                } else if self.takes.contains(&to) {
                    board.make(Action::Take {
                        from: self.selected.clone().unwrap(),
                        to: to.clone(),
                    })
                } else if self.moves.contains(&to) {
                    board.make(Action::Move {
                        from: self.selected.clone().unwrap(),
                        to: to.clone(),
                    })
                }
                Self {
                    board,
                    selected: None,
                    moves: vec![],
                    takes: vec![],
                    attacks: vec![],
                }
            }
        }
        .into()
    }
}

pub enum TaskAction {
    Set(Vec<Task>),
    Add(Task),
    Delete(String),
    Toggle(String),
}

#[derive(Default)]
pub struct TaskState {
    pub tasks: Vec<Task>,
}

impl Reducible for TaskState {
    type Action = TaskAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let next_tasks = match action {
            TaskAction::Set(tasks) => tasks,
            TaskAction::Add(task) => {
                let mut tasks = self.tasks.clone();
                tasks.push(task);
                tasks
            }
            TaskAction::Delete(id) => {
                let mut tasks = self.tasks.clone();
                tasks.retain(|task| task.id != id);
                tasks
            }
            TaskAction::Toggle(id) => {
                let mut tasks = self.tasks.clone();
                let task = tasks.iter_mut().find(|task| task.id == id);
                if let Some(task) = task {
                    task.completed = !task.completed;
                }
                tasks
            }
        };

        Self { tasks: next_tasks }.into()
    }
}
