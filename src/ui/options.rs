use crate::pathfinders::{Grid, PathFindAlgorithms, Pos, Unit};
use crate::ui::grid::GridRenderMode;
use std::ops::Deref;
use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::{
    classes, function_component, html, use_mut_ref, use_state, Callback, Event, Html, Properties,
};

#[derive(Copy, Clone, PartialEq)]
pub struct GridOptions {
    pub rows: usize,
    pub columns: usize,
    pub start_pos: Pos,
    pub end_pos: Pos,
}

impl Default for GridOptions {
    fn default() -> Self {
        Self {
            rows: 10,
            columns: 10,
            start_pos: Pos { x: 0, y: 0 },
            end_pos: Pos { x: 9, y: 9 },
        }
    }
}

impl From<GridOptions> for Grid {
    fn from(value: GridOptions) -> Self {
        Self::new(
            value.rows as Unit,
            value.columns as Unit,
            value.start_pos,
            value.end_pos,
        )
    }
}

#[derive(Properties, PartialEq)]
pub struct OptionsProps {
    #[prop_or(PathFindAlgorithms::BreadthFirst)]
    pub default_path_finder: PathFindAlgorithms,
    pub on_find_path: Callback<PathFindAlgorithms>,

    #[prop_or_default]
    pub default_grid_options: GridOptions,
    #[prop_or(Callback::noop())]
    pub on_grid_options_change: Callback<GridOptions>,

    #[prop_or(GridRenderMode::WebGL2)]
    pub default_grid_renderer: GridRenderMode,
    #[prop_or(Callback::noop())]
    pub on_grid_renderer_change: Callback<GridRenderMode>,
}

#[function_component]
pub fn Options(props: &OptionsProps) -> Html {
    let selected_path_finder = {
        let default_path_finder = props.default_path_finder;
        use_mut_ref(|| default_path_finder)
    };
    let grid_options = use_state(|| props.default_grid_options);
    let grid_renderer = {
        let default_grid_renderer = props.default_grid_renderer;
        use_mut_ref(|| default_grid_renderer)
    };

    // Pathfinder
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

    // Rows/Columns
    let on_rows_change = {
        let on_grid_options_change = props.on_grid_options_change.clone();
        let grid_options = grid_options.clone();

        Callback::from(move |e: Event| {
            let target = e
                .target()
                .expect("Unable to get event target")
                .dyn_into::<HtmlInputElement>()
                .expect("Unable to cast to HtmlInputElement");
            let rows = target
                .value()
                .parse::<usize>()
                .expect("Unable to parse rows to usize");

            let mut new_grid_options = *grid_options.deref();
            new_grid_options.rows = rows;
            update_start_end(&mut new_grid_options);

            on_grid_options_change.emit(new_grid_options);
            grid_options.set(new_grid_options);
        })
    };
    let on_columns_change = {
        let on_grid_options_change = props.on_grid_options_change.clone();
        let grid_options = grid_options.clone();

        Callback::from(move |e: Event| {
            let target = e
                .target()
                .expect("Unable to get event target")
                .dyn_into::<HtmlInputElement>()
                .expect("Unable to cast to HtmlInputElement");
            let columns = target
                .value()
                .parse::<usize>()
                .expect("Unable to parse rows to usize");

            let mut new_grid_options = *grid_options.deref();
            new_grid_options.columns = columns;
            update_start_end(&mut new_grid_options);

            on_grid_options_change.emit(new_grid_options);
            grid_options.set(new_grid_options);
        })
    };

    // Grid Renderer
    let grid_renderer = *grid_renderer.borrow().deref();
    let on_grid_renderer_change = {
        let on_grid_renderer_change = props.on_grid_renderer_change.clone();

        Callback::from(move |e: Event| {
            let target = e
                .target()
                .expect("Unable to get event target")
                .dyn_into::<HtmlSelectElement>()
                .expect("Unable to cast to HtmlSelectElement");
            let selected = target.value();

            let grid_renderer =
                grid_renderer_from_str(&selected).expect("Unable to parse grid renderer");
            on_grid_renderer_change.emit(grid_renderer);
        })
    };

    html! {
        <div class={classes!("options")}>
            <select onchange={selection_on_change}>
                {create_path_finder_option(PathFindAlgorithms::DepthFirst, selected_path_finder, "Depth First")}
                {create_path_finder_option(PathFindAlgorithms::BreadthFirst, selected_path_finder, "Breadth First")}
                {create_path_finder_option(PathFindAlgorithms::Dijkstra, selected_path_finder, "Dijkstra")}
                {create_path_finder_option(PathFindAlgorithms::AStar, selected_path_finder, "A*")}
            </select>
            <button onclick={on_click_find_path}>{"Start Search"}</button>

            <div>
                <h3 class={classes!("options-grid-header")}>{"Grid Options"}</h3>
                <div class={classes!("options-grid")}>
                    <div>
                        <label>{"Rows: "}</label>
                        <input type="range" min="1" max="25" value={grid_options.rows.to_string()} onchange={on_rows_change} />
                    </div>
                    <div>
                        <label>{"Columns: "}</label>
                        <input type="range" min="1" max="25" value={grid_options.columns.to_string()} onchange={on_columns_change} />
                    </div>
                </div>
            </div>

            <div>
                <h3 class={classes!("options-renderer-header")}>{"Grid Renderer"}</h3>
                <select class={classes!("options-renderer")} onchange={on_grid_renderer_change}>
                    {create_grid_renderer_option(GridRenderMode::Dom, grid_renderer, "DOM (slow)")}
                    {create_grid_renderer_option(GridRenderMode::WebGL2, grid_renderer, "WebGL 2")}
                </select>
            </div>
        </div>
    }
}

fn create_path_finder_option(
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

fn update_start_end(grid_options: &mut GridOptions) {
    fn update_pos(grid_options: &GridOptions, pos: Pos) -> Pos {
        Pos {
            x: pos.x.min(grid_options.rows as Unit - 1),
            y: pos.y.min(grid_options.columns as Unit - 1),
        }
    }
    grid_options.start_pos = update_pos(grid_options, grid_options.start_pos);
    grid_options.end_pos = update_pos(grid_options, grid_options.end_pos);
}

fn path_finder_str(path_finder: PathFindAlgorithms) -> &'static str {
    match path_finder {
        PathFindAlgorithms::DepthFirst => "depth_first",
        PathFindAlgorithms::BreadthFirst => "breadth_first",
        PathFindAlgorithms::Dijkstra => "dijkstra",
        PathFindAlgorithms::AStar => "a_star",
    }
}

fn path_finder_from_str(str: &str) -> Option<PathFindAlgorithms> {
    match str {
        "depth_first" => Some(PathFindAlgorithms::DepthFirst),
        "breadth_first" => Some(PathFindAlgorithms::BreadthFirst),
        "dijkstra" => Some(PathFindAlgorithms::Dijkstra),
        "a_star" => Some(PathFindAlgorithms::AStar),
        _ => None,
    }
}

fn create_grid_renderer_option(
    grid_renderer: GridRenderMode,
    selected_grid_renderer: GridRenderMode,
    name: &str,
) -> Html {
    let grid_renderer_str = grid_renderer_str(grid_renderer);
    let selected = grid_renderer == selected_grid_renderer;

    html! {
        <option value={grid_renderer_str} selected={selected}>{name}</option>
    }
}

fn grid_renderer_str(grid_renderer: GridRenderMode) -> &'static str {
    match grid_renderer {
        GridRenderMode::Dom => "dom",
        GridRenderMode::WebGL2 => "webgl_2",
    }
}

fn grid_renderer_from_str(str: &str) -> Option<GridRenderMode> {
    match str {
        "dom" => Some(GridRenderMode::Dom),
        "webgl_2" => Some(GridRenderMode::WebGL2),
        _ => None,
    }
}
