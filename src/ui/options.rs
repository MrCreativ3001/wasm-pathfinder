use crate::pathfinders::PathFindAlgorithms;
use std::ops::{Deref, DerefMut};
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;
use yew::{
    classes, function_component, html, use_mut_ref, Callback, Event, Html, Properties, TargetCast,
};

#[derive(Properties, PartialEq)]
pub struct OptionsProps {
    #[prop_or(PathFindAlgorithms::BreadthFirst)]
    pub default_path_finder: PathFindAlgorithms,
    pub on_find_path: Callback<PathFindAlgorithms>,
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
                {create_option(PathFindAlgorithms::BreadthFirst, selected_path_finder, "Breadth first")}
            </select>
            <button onclick={on_click_find_path}>{"Start Search"}</button>
        </div>
    }
}

fn create_option(
    path_finder: PathFindAlgorithms,
    selected_path_finder: PathFindAlgorithms,
    name: &str,
) -> Html {
    let path_finder_str = path_finder_str(path_finder);
    let selected = path_finder == selected_path_finder;

    html! {
        <option value={path_finder_str} selected={selected}>{name}</option>
    }
}

fn path_finder_str(path_finder: PathFindAlgorithms) -> &'static str {
    match path_finder {
        PathFindAlgorithms::BreadthFirst => "breadth_first",
    }
}

fn path_finder_from_str(str: &str) -> Option<PathFindAlgorithms> {
    match str {
        "breadth_first" => Some(PathFindAlgorithms::BreadthFirst),
        _ => None,
    }
}
