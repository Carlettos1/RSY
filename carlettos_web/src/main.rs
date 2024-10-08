use yew::{html::Scope, prelude::*};
use yew_router::prelude::*;

mod controllers;
mod models;
mod state;
mod sub_api;

mod pages {
    mod c2048;
    mod carlettos_chess;
    mod chess;
    mod game_of_life;
    mod home;
    mod page_not_found;
    mod todo_app;
    mod votacion;

    pub use c2048::*;
    pub use carlettos_chess::*;
    pub use chess::*;
    pub use game_of_life::*;
    pub use home::*;
    pub use page_not_found::*;
    pub use todo_app::*;
    pub use votacion::*;
}

mod components {
    mod task_form;
    mod task_item;
    mod task_list;

    pub use task_form::*;
    pub use task_item::*;
    pub use task_list::*;
}

mod cells {
    mod cell;

    pub use cell::*;
}

use crate::pages::*;

#[derive(Debug, Routable, PartialEq, Eq, Clone)]
pub enum Route {
    #[at("/chess")]
    Chess,
    #[at("/game_of_life")]
    GameOfLife,
    #[at("/todo_app")]
    TodoApp,
    #[at("/")]
    Home,
    #[at("/carlettos_chess")]
    CarlettosChess,
    #[at("/votacion")]
    Votacion,
    #[at("/c2048")]
    C2048,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub enum Msg {
    ToggleNavbar,
}

pub struct App {
    navbar_active: bool,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            navbar_active: false,
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                { self.view_nav(ctx.link()) }

                <main>
                    <Switch<Route> render={switch}/>
                </main>
                <footer class="footer">
                    <div class="content has-text-centered">
                        { "Made by Carlettos" }
                    </div>
                </footer>
            </BrowserRouter>
        }
    }
}

impl App {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        let Self { navbar_active } = *self;

        let active_class = if navbar_active { "is-active" } else { "" };
        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">{ "Happy" }</h1>

                    <button class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={link.callback(|_| Msg::ToggleNavbar)}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-start">
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                            { "Home" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::C2048}>
                            { "2048!" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::GameOfLife}>
                            { "Game Of Life" }
                        </Link<Route>>
                    </div>
                </div>
            </nav>
        }
    }

    #[allow(dead_code)]
    fn view_nav_pro(&self, link: &Scope<Self>) -> Html {
        let Self { navbar_active } = *self;

        let active_class = if !navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">{ "Happy" }</h1>

                    <button class={classes!("navbar-burger", "burger", active_class)}
                        aria-label="menu" aria-expanded="false"
                        onclick={link.callback(|_| Msg::ToggleNavbar)}
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </button>
                </div>
                <div class={classes!("navbar-menu", active_class)}>
                    <div class="navbar-start">
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Home}>
                            { "Home" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::CarlettosChess}>
                            { "Carlettos' Chess" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::Votacion}>
                            { "Votacion" }
                        </Link<Route>>
                        <Link<Route> classes={classes!("navbar-item")} to={Route::C2048}>
                            { "2048!" }
                        </Link<Route>>

                        <div class="navbar-item has-dropdown is-hoverable">
                            <div class="navbar-link">
                                { "Apps" }
                            </div>
                            <div class="navbar-dropdown">
                                <Link<Route> classes={classes!("navbar-item")} to={Route::TodoApp}>
                                    { "Todo App" }
                                </Link<Route>>
                                <Link<Route> classes={classes!("navbar-item")} to={Route::Chess}>
                                    { "Chess" }
                                </Link<Route>>
                                <Link<Route> classes={classes!("navbar-item")} to={Route::GameOfLife}>
                                    { "Game Of Life" }
                                </Link<Route>>
                            </div>
                        </div>
                    </div>
                </div>
            </nav>
        }
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home/> }
        }
        Route::TodoApp => {
            html! { <TodoApp /> }
        }
        Route::Chess => {
            html! { <ChessBoard /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
        Route::GameOfLife => {
            html! { <GameOfLife /> }
        }
        Route::CarlettosChess => {
            html! { <CarlettosChess /> }
        }
        Route::Votacion => {
            html! { <Votacion /> }
        }
        Route::C2048 => {
            html! { <C2048 /> }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
