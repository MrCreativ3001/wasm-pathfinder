use crate::pathfinders::{Grid, Pos, Tile, Unit};
use crate::ui::grid::GridProps;
use std::ops::{BitAnd, Deref, Range, RangeBounds};
use yew::{
    classes, function_component, html, props, Callback, Classes, Component, Context, DragEvent,
    Html, MouseEvent, Properties,
};

#[derive(Clone, Copy, PartialEq)]
enum DragState {
    None,
    Start,
    End,
}

pub struct DOMGridComponent {
    drag_state: DragState,
}

pub enum GridMsg {
    DragStart,
    DragEnd,
    DragNone,
}

impl Component for DOMGridComponent {
    type Message = GridMsg;
    type Properties = GridProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            drag_state: DragState::None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GridMsg::DragStart => {
                self.drag_state = DragState::Start;
                false
            }
            GridMsg::DragEnd => {
                self.drag_state = DragState::End;
                false
            }
            GridMsg::DragNone => {
                self.drag_state = DragState::None;
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let grid = &ctx.props().grid;
        let props = ctx.props();
        let drag_state = &self.drag_state;
        let drag_state_change = ctx.link().callback(|new: DragState| match new {
            DragState::None => GridMsg::DragNone,
            DragState::Start => GridMsg::DragStart,
            DragState::End => GridMsg::DragEnd,
        });
        let start = grid.start();
        let end = grid.end();

        html!(
            <div class={classes!("grid", "dom-grid")}>
                {for gen_2d_iter(0..grid.rows(), 0..grid.columns()).map(|(x, y)| {
                    let pos = Pos { x, y };
                    let tile = grid.tile(pos);
                    let is_tile_start= pos == start;
                    let is_tile_end = pos == end;
                    let is_tile_path = props.path.contains(&pos);
                    let is_visited = props.visited.emit(pos);
                    let is_new_line = y == 0;

                    let tile_on_tile_click = {
                        let on_tile_click = props.on_tile_click.clone();
                        let on_start_move = props.on_start_move.clone();
                        let on_end_move = props.on_end_move.clone();
                        let drag_state = *drag_state;
                        let drag_state_change = drag_state_change.clone();

                        Callback::from(move |e| {
                            if is_tile_start {
                                drag_state_change.emit(DragState::Start);
                            } else if is_tile_end {
                                drag_state_change.emit(DragState::End);
                            } else {
                                match drag_state {
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
                        let drag_state = *drag_state;
                        let drag_state_change = drag_state_change.clone();

                        Callback::from(move |mouse_down| {
                            if mouse_down {
                                match drag_state {
                                    DragState::Start => on_start_move.emit(pos),
                                    DragState::End => on_end_move.emit(pos),
                                    DragState::None => on_tile_click.emit(pos),
                                }
                            } else {
                                drag_state_change.emit(DragState::None);
                            }
                        })
                    };

                    html!{
                        <TileComponent
                            tile={tile}
                            is_start={is_tile_start}
                            is_end={is_tile_end}
                            is_path={is_tile_path}
                            is_visited={is_visited}
                            on_tile_click={tile_on_tile_click}
                            on_tile_mouse_enter={on_tile_mouse_enter}
                            tile_key={pos}
                            is_new_line={is_new_line}
                        />
                    }
                }) }
            </div>
        )
    }
}

fn gen_2d_iter<N>(x: Range<N>, y: Range<N>) -> impl Iterator<Item = (N, N)>
where
    N: Copy,
    Range<N>: IntoIterator<Item = N>,
{
    x.into_iter()
        .flat_map(move |x| y.clone().into_iter().map(move |y| (x, y)))
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
    pub tile_key: Pos,
    pub is_new_line: bool,
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
    let tile_classes: Classes = classes!("tile", class);

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

            on_tile_mouse_enter.emit(mouse_down)
        })
    };

    let prevent_drag = { Callback::from(move |e: DragEvent| e.prevent_default()) };

    html!(
        <>
            if props.is_new_line {
                <div class={classes!("grid-newline")} />
            }
            <div class={tile_classes} key={format!("{}-{}", props.tile_key.x, props.tile_key.x)} onmousedown={on_mouse_down} onmouseenter={on_mouse_enter} ondragstart={prevent_drag} />
        </>
    )
}
