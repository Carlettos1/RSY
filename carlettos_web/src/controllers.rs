use carlettos_chess::Pos;
use chess_api::{Board, Color};
use yew::UseReducerHandle;

use crate::{
    state::{
        CarlettosChessAction, CarlettosChessState, ChessAction, ChessState, TaskAction, TaskState,
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
