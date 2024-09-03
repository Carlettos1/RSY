use carlettos_chess::Pos;
use chess_api::{Board, Color};
use yew::UseReducerHandle;

use crate::{
    c2048_leader_board::Entry,
    models::Vote,
    state::{
        C2048LeaderboardAction, C2048LeaderboardState, CarlettosChessAction, CarlettosChessState,
        ChessAction, ChessState, TaskAction, TaskState, VoteAction, VotesState,
    },
    sub_api,
};

pub struct ChessController {
    state: UseReducerHandle<ChessState>,
}

impl ChessController {
    pub fn new(state: UseReducerHandle<ChessState>) -> ChessController {
        ChessController { state }
    }

    pub fn get_chess(&self) {
        let chess = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_board = sub_api::get_chess_game().await.unwrap();
            chess.dispatch(ChessAction::Get(fetched_board))
        })
    }

    pub fn update_chess(&self, board: Board) {
        let chess = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let updated_board = sub_api::update_chess_game(board).await.unwrap();
            chess.dispatch(ChessAction::Update(updated_board))
        })
    }

    pub fn on_click(&self, from: (usize, usize)) {
        let chess = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let mut board = chess.board.clone();
            board.on_click(from);
            let updated_board = sub_api::update_chess_game(board).await.unwrap();
            chess.dispatch(ChessAction::Update(updated_board))
        })
    }

    pub fn winner(&self) -> &Option<Color> {
        &self.state.winner
    }
}

#[derive(PartialEq)]
pub struct CarlettosChessController {
    state: UseReducerHandle<CarlettosChessState>,
}

impl CarlettosChessController {
    pub fn new(state: UseReducerHandle<CarlettosChessState>) -> CarlettosChessController {
        CarlettosChessController { state }
    }

    pub fn start(&self) {
        let chess = self.state.clone();
        chess.dispatch(CarlettosChessAction::Start);
    }

    pub fn on_click(&self, from: Pos) {
        let chess = self.state.clone();
        chess.dispatch(CarlettosChessAction::OnClick(from));
    }

    pub fn on_display_click(&self, from: Pos) {
        let chess = self.state.clone();
        chess.dispatch(CarlettosChessAction::DisplayClick(from));
    }
}

pub struct TaskController {
    state: UseReducerHandle<TaskState>,
}

impl TaskController {
    pub fn new(state: UseReducerHandle<TaskState>) -> TaskController {
        TaskController { state }
    }

    pub fn init_tasks(&self) {
        let tasks = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_tasks = sub_api::fetch_tasks().await.unwrap();
            tasks.dispatch(TaskAction::Set(fetched_tasks))
        });
    }

    pub fn create_task(&self, title: String) {
        let tasks = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let response = sub_api::create_task(&title).await.unwrap();
            tasks.dispatch(TaskAction::Add(response));
        });
    }

    pub fn toggle_task(&self, id: String) {
        let tasks = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let response = sub_api::toggle_task(id.clone()).await.unwrap();
            if response.rows_affected == 1 {
                tasks.dispatch(TaskAction::Toggle(id.clone()));
            }
        });
    }

    pub fn delete_task(&self, id: String) {
        let tasks = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let response = sub_api::delete_task(id.clone()).await.unwrap();
            if response.rows_affected == 1 {
                tasks.dispatch(TaskAction::Delete(id.clone()));
            }
        });
    }
}

pub struct VotesController {
    pub state: UseReducerHandle<VotesState>,
}

impl VotesController {
    pub fn new(state: UseReducerHandle<VotesState>) -> VotesController {
        VotesController { state }
    }

    pub fn init_votes(&self, id: String) {
        let votes = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let fetched_votes = sub_api::get_votes(id.clone()).await;
            let mut fetched_votes = fetched_votes.unwrap();
            fetched_votes.id = id;
            votes.dispatch(VoteAction::Set(fetched_votes))
        });
    }

    pub fn remove_vote(&self, vote_id: usize) {
        let votes = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let response = sub_api::remove_vote(votes.votes.id.clone(), vote_id)
                .await
                .unwrap();
            votes.dispatch(VoteAction::Set(response));
        });
    }

    pub fn add_vote(&self, vote_id: usize) {
        let votes = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let response = sub_api::add_vote(votes.votes.id.clone(), vote_id)
                .await
                .unwrap();
            votes.dispatch(VoteAction::Set(response));
        });
    }

    pub fn click(&self, image_id: usize) -> VoteAction {
        if self.state.votes.votes.contains(&Vote { id: image_id }) {
            VoteAction::Remove(Vote { id: image_id })
        } else {
            VoteAction::Add(Vote { id: image_id })
        }
    }
}

pub struct C2048LeaderboardController {
    pub state: UseReducerHandle<C2048LeaderboardState>,
}

impl C2048LeaderboardController {
    pub fn new(state: UseReducerHandle<C2048LeaderboardState>) -> C2048LeaderboardController {
        C2048LeaderboardController { state }
    }

    pub fn get_highscores(&self) {
        let state = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let highscores = sub_api::get_highscores().await.unwrap();
            state.dispatch(C2048LeaderboardAction::Load(highscores));
        })
    }

    pub fn add_highscore(&self, entry: Entry) {
        let state = self.state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let entry = sub_api::add_highscore(&entry).await;
            println!("{entry:?}");
            let entry = entry.unwrap();
            state.dispatch(C2048LeaderboardAction::Add(entry));
        })
    }
}
