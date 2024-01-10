use yew::{html::Scope, prelude::*};
use yew_router::prelude::*;

mod controllers;
mod models;
mod state;
mod sub_api;

mod pages {
    pub mod chess;
    mod game_of_life;
    mod home;
    mod page_not_found;

    pub use chess::*;
    pub use game_of_life::*;
    pub use home::*;
    pub use page_not_found::*;
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
    #[at("/")]
    Home,
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
                        { "Soy gay" }
                    </div>
                </footer>
            </BrowserRouter>
        }
    }
}

impl App {
    fn view_nav(&self, link: &Scope<Self>) -> Html {
        let Self { navbar_active } = *self;

        let active_class = if !navbar_active { "is-active" } else { "" };

        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                    <h1 class="navbar-item is-size-3">{ "Hola???" }</h1>

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

                        <div class="navbar-item has-dropdown is-hoverable">
                            <div class="navbar-link">
                                { "More" }
                            </div>
                            <div class="navbar-dropdown">
                                <Link<Route> classes={classes!("navbar-item")} to={Route::Chess}>
                                    { "Chess?" }
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
            html! { <Home /> }
        }
        Route::Chess => {
            html! { <crate::pages::chess::ChessBoard /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
        Route::GameOfLife => {
            html! { <GameOfLife /> }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
