use crate::pathfinders::PathFinders;
use yew::{classes, function_component, html, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct OptionsProps {
    #[prop_or(PathFinders::BreadthFirst)]
    pub default_path_finder: PathFinders,
    pub on_find_path: Callback<PathFinders>,
}

#[function_component]
pub fn Options(props: &OptionsProps) -> Html {
    let on_click_find_path = {
        let on_find_path = props.on_find_path.clone();
        let default_path_finder = props.default_path_finder;

        Callback::from(move |_| on_find_path.emit(default_path_finder))
    };

    html! {
        <div class={classes!("options")}>
            <button onclick={on_click_find_path}>{"Find path"}</button>
        </div>
    }
}
