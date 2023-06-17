use crate::pathfinders::{Grid, Pos, Tile, Unit};
use std::ops::{BitAnd, Deref, Range};
use yew::{
    classes, function_component, html, use_mut_ref, Callback, Classes, DragEvent, Html, MouseEvent,
    Properties,
};

#[derive(Properties, PartialEq)]
pub struct GridProps {
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

#[derive(Clone, Copy, PartialEq)]
enum DragState {
    None,
    Start,
    End,
}

#[function_component]
pub fn GridComponent(props: &GridProps) -> Html {
    let drag_state = use_mut_ref(|| DragState::None);

    let grid = &props.grid;
    let start = grid.start();
    let end = grid.end();

    html!(
        <div class={classes!("grid")}>
            {
                html_list(0..grid.rows(), classes!("rows"), |x| {
                    html_list(0..grid.columns(), classes!("row"), |y| {
                        let pos = Pos { x, y };
                        let tile = grid.tile(pos);
                        let is_tile_start= pos == start;
                        let is_tile_end = pos == end;
                        let is_tile_path = props.path.contains(&pos);
                        let is_visited = props.visited.emit(pos);

                        let tile_on_tile_click = {
                            let on_tile_click = props.on_tile_click.clone();
                            let on_start_move = props.on_start_move.clone();
                            let on_end_move = props.on_end_move.clone();
                            let drag_state = drag_state.clone();

                            Callback::from(move |e| {
                                if is_tile_start {
                                    drag_state.replace_with(|_| DragState::Start);
                                } else if is_tile_end {
                                    drag_state.replace_with(|_| DragState::End);
                                } else {
                                    match drag_state.borrow().deref() {
                                        DragState::Start => on_start_move.emit(pos),
                                        DragState::End => on_end_move.emit(pos),
                                        DragState::None => on_tile_click.emit(pos)
                                    }
                                }
                            })
                        };
                        let on_tile_mouse_enter = {
                            let on_start_move = props.on_start_move.clone();
                            let on_end_move = props.on_end_move.clone();
                            let on_tile_click = props.on_tile_click.clone();
                            let drag_state = drag_state.clone();

                            Callback::from(move |mouse_down| {
                                if mouse_down {
                                    match drag_state.borrow().deref() {
                                        DragState::Start => on_start_move.emit(pos),
                                        DragState::End => on_end_move.emit(pos),
                                        DragState::None => on_tile_click.emit(pos),
                                    }
                                } else {
                                    drag_state.replace_with(|_| DragState::None);
                                }
                            })
                        };

                        html!(<TileComponent tile={tile} is_start={is_tile_start} is_end={is_tile_end} is_path={is_tile_path} is_visited={is_visited} on_tile_click={tile_on_tile_click} on_tile_mouse_enter={on_tile_mouse_enter} />)
                    })
                })
        }
        </div>
    )
}

fn html_list<F>(range: Range<Unit>, classes: Classes, f: F) -> Html
where
    F: Fn(Unit) -> Html,
{
    html!(
        <div class={classes}>
            { range.map(f).collect::<Vec<_>>() }
        </div>
    )
}

#[derive(Properties, PartialEq)]
struct TileProps {
    pub tile: Tile,
    pub is_start: bool,
    pub is_end: bool,
    pub is_path: bool,
    pub is_visited: bool,
    pub on_tile_click: Callback<()>,
    pub on_tile_mouse_enter: Callback<bool>,
}
#[function_component]
fn TileComponent(props: &TileProps) -> Html {
    let tile = &props.tile;
    let class = match (
        tile,
        props.is_start,
        props.is_end,
        props.is_path,
        props.is_visited,
    ) {
        (_, true, _, _, _) => "tile-start",
        (_, _, true, _, _) => "tile-end",
        (Tile::Wall, _, _, _, _) => "tile-wall",
        (_, _, _, _, true) => "tile-visited",
        (_, _, _, true, _) => "tile-path",
        (Tile::None, _, _, _, _) => "tile-none",
    };

    const LEFT_MOUSE_BUTTON_BITMASK: u16 = 1;
    let on_mouse_down = {
        let on_tile_click = props.on_tile_click.clone();

        Callback::from(move |e: MouseEvent| {
            let mouse_down = e.buttons().bitand(LEFT_MOUSE_BUTTON_BITMASK) != 0;

            if mouse_down {
                on_tile_click.emit(());
            }
        })
    };
    let on_mouse_enter = {
        let on_tile_mouse_enter = props.on_tile_mouse_enter.clone();

        Callback::from(move |e: MouseEvent| {
            let mouse_down = e.buttons().bitand(LEFT_MOUSE_BUTTON_BITMASK) != 0;

            if mouse_down {
                on_tile_mouse_enter.emit(mouse_down)
            }
        })
    };

    let prevent_drag = { Callback::from(move |e: DragEvent| e.prevent_default()) };

    html!(
        <div class={classes!("tile", class)} onmousedown={on_mouse_down} onmouseenter={on_mouse_enter} ondragstart={prevent_drag} />
    )
}
