use crate::pathfinders::{Grid, Pos};
use crate::ui::grid::dom::DOMGridComponent;
use crate::ui::grid::webgl2::WebGL2GridComponent;
use yew::{function_component, html, Callback, Html, Properties};

#[derive(Clone, Copy, PartialEq)]
pub enum GridRenderMode {
    WebGL2,
    DOM,
}

#[derive(Properties, Clone, PartialEq)]
pub struct GridProps {
    #[prop_or(GridRenderMode::WebGL2)]
    pub mode: GridRenderMode,
    pub grid: Grid,
    #[prop_or_default]
    pub path: Vec<Pos>,
    #[prop_or(Callback::from(|_| false))]
    pub visited: Callback<Pos, bool>,
    #[prop_or_default]
    pub on_tile_click: Callback<Pos>,
    #[prop_or_default]
    pub on_start_move: Callback<Pos>,
    #[prop_or_default]
    pub on_end_move: Callback<Pos>,
}

pub mod dom;
pub mod webgl2;

#[function_component]
pub fn GridComponent(props: &GridProps) -> Html {
    let props = props.clone();
    match props.mode {
        GridRenderMode::DOM => html!(<DOMGridComponent ..props />),
        GridRenderMode::WebGL2 => html!(<WebGL2GridComponent ..props />),
    }
}
