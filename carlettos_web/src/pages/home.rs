use std::rc::Rc;

use yew::prelude::*;

use crate::components::*;
use crate::controllers::*;
use crate::state::*;

#[function_component(Home)]
pub fn app() -> Html {
    let tasks = use_reducer(TaskState::default);
    let tasks_controller = Rc::new(TaskController::new(tasks.clone()));

    {
        let tasks_controller = tasks_controller.clone();
        use_effect_with((), move |_| {
            tasks_controller.init_tasks();
            || () // return empty destructor closure
        }); // only call on first render
    }

    let on_create_task = {
        let tasks_controller = tasks_controller.clone();
        Callback::from(move |title: String| {
            tasks_controller.create_task(title);
        })
    };

    let on_delete_task = {
        let tasks_controller = tasks_controller.clone();
        Callback::from(move |id: String| {
            tasks_controller.delete_task(id);
        })
    };

    let on_toggle_task = {
        let tasks_controller = tasks_controller.clone();
        Callback::from(move |id: String| {
            tasks_controller.toggle_task(id);
        })
    };

    html! {
        <div class="container">
            <a href="chess"> { "Chess Page" } </a>
            <TaskForm on_create_task={on_create_task} />
            <h3>{ "Todo" }</h3>
            <div>
                <TaskList
                    tasks={tasks.tasks.clone()}
                    on_delete_task={on_delete_task}
                    on_toggle_task={on_toggle_task}
                />
            </div>
        </div>
    }
}
