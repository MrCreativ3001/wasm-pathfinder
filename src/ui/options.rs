use crate::pathfinders::PathFinders;
use std::mem::swap;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::html::IntoPropValue;
use yew::{
    classes, function_component, html, use_mut_ref, Callback, Event, Html, InputEvent, Properties,
    TargetCast,
};

#[derive(Properties, PartialEq)]
pub struct OptionsProps {
    #[prop_or(PathFinders::BreadthFirst)]
    pub default_path_finder: PathFinders,
    pub on_find_path: Callback<PathFinders>,
}

#[function_component]
pub fn Options(props: &OptionsProps) -> Html {
    let selected_path_finder = {
        let default_path_finder = props.default_path_finder;
        use_mut_ref(|| default_path_finder)
    };

    let on_click_find_path = {
        let on_find_path = props.on_find_path.clone();
        let selected_path_finder = selected_path_finder.clone();

        Callback::from(move |_| on_find_path.emit(*selected_path_finder.borrow().deref()))
    };
    let selection_on_change = {
        let selected_path_finder = selected_path_finder.clone();

        Callback::from(move |e: Event| {
            let target = e
                .target()
                .expect("Unable to get event target")
                .dyn_into::<HtmlSelectElement>()
                .expect("Unable to cast to HtmlSelectElement");
            let selected = target.value();

            selected_path_finder.replace_with(|_| {
                path_finder_from_str(&selected).expect("Unable to parse path finder")
            });
        })
    };

    let selected_path_finder = *selected_path_finder.borrow().deref();
    html! {
        <div class={classes!("options")}>
            <select onchange={selection_on_change}>
                {create_option(PathFinders::BreadthFirst, selected_path_finder, "Breadth first")}
                {create_option(PathFinders::AStar, selected_path_finder, "A*")}
            </select>
            <button onclick={on_click_find_path}>{"Find path"}</button>
        </div>
    }
}

fn create_option(path_finder: PathFinders, selected_path_finder: PathFinders, name: &str) -> Html {
    let path_finder_str = path_finder_str(path_finder);
    let selected = path_finder == selected_path_finder;

    html! {
        <option value={path_finder_str} selected={selected}>{name}</option>
    }
}

fn path_finder_str(path_finder: PathFinders) -> &'static str {
    match path_finder {
        PathFinders::BreadthFirst => "breadth_first",
        PathFinders::AStar => "astar",
    }
}

fn path_finder_from_str(str: &str) -> Option<PathFinders> {
    match str {
        "breadth_first" => Some(PathFinders::BreadthFirst),
        "astar" => Some(PathFinders::AStar),
        _ => None,
    }
}
