use carlettos_chess::chess_controller::CChess;
use chess_api::{Board, Color};
use yew::Reducible;

use crate::{
    c2048_leader_board::Entry,
    models::{Check, Task, Vote, Votes},
};

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
    DisplayClick(carlettos_chess::prelude::Pos),
}

#[derive(Default, PartialEq)]
pub struct CarlettosChessState {
    pub board: CChess,
    pub display: CChess,
}

impl Reducible for CarlettosChessState {
    type Action = CarlettosChessAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            CarlettosChessAction::Start => Self {
                board: CChess::cchessboard(),
                display: CChess::default_display(),
            },
            CarlettosChessAction::OnClick(pos) => {
                let mut board = self.board.clone();
                let mut display = self.display.clone();
                match display.selected {
                    Some(dis) => {
                        // put the selected piece into the board
                        let piece = display.board.get(&dis).unwrap().piece.clone();
                        board.board.get_mut(&pos).unwrap().replace(piece);
                    }
                    None => {
                        board.click(pos);
                    }
                }
                display.selected = None;
                Self { board, display }
            }
            CarlettosChessAction::DisplayClick(pos) => {
                let mut display = self.display.clone();
                display.selected = Some(pos);
                Self {
                    board: self.board.clone(),
                    display,
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

pub enum VoteAction {
    Set(Votes),
    Add(Vote),
    Remove(Vote),
}

#[derive(Default)]
pub struct VotesState {
    pub votes: Votes,
    pub login: bool,
    pub checks: Vec<Check>,
}

impl Reducible for VotesState {
    type Action = VoteAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut checks = self.checks.clone();
        let next_votes = match action {
            VoteAction::Set(votes) => votes,
            VoteAction::Add(vote) => {
                let mut votes = self.votes.clone();
                if votes.votes.len() < 3 {
                    votes.add(vote.id);
                }
                votes
            }
            VoteAction::Remove(vote) => {
                let mut votes = self.votes.clone();
                checks[vote.id] = Check::None;
                votes.remove(vote.id);
                votes
            }
        };

        let checks = Check::update_from_votes(&next_votes.votes);
        VotesState {
            votes: next_votes,
            login: true,
            checks,
        }
        .into()
    }
}

#[derive(Default)]
pub struct C2048LeaderboardState {
    pub entries: Vec<Entry>,
}

pub enum C2048LeaderboardAction {
    Add(Entry),
    Load(Vec<Entry>),
}

impl Reducible for C2048LeaderboardState {
    type Action = C2048LeaderboardAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            C2048LeaderboardAction::Load(entries) => C2048LeaderboardState { entries }.into(),
            C2048LeaderboardAction::Add(entry) => {
                let mut entries = self.entries.clone();
                entries.push(entry);
                C2048LeaderboardState { entries }.into()
            }
        }
    }
}
