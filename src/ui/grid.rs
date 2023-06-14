use crate::pathfinders::{Grid, Pos, Tile, Unit};
use crate::ui::grid::_TileProps::{is_end, is_start};
use std::ops::{BitAnd, Range};
use yew::{
    classes, function_component, html, Callback, Classes, DragEvent, Html, MouseEvent, Properties,
};

#[derive(Properties, PartialEq)]
pub struct GridProps {
    pub grid: Grid,
    #[prop_or_default]
    pub on_tile_click: Callback<Pos>,
}

#[function_component]
pub fn GridComponent(props: &GridProps) -> Html {
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

                        let on_tile_click = {
                            let on_tile_click = props.on_tile_click.clone();
                            Callback::from(move |_| {
                                on_tile_click.emit(pos)
                            })
                        };

                        html!(<TileComponent tile={tile} is_start={is_tile_start} is_end={is_tile_end} on_tile_click={on_tile_click} />)
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
    pub on_tile_click: Callback<()>,
}
#[function_component]
fn TileComponent(props: &TileProps) -> Html {
    let tile = &props.tile;
    let class = match (tile, props.is_start, props.is_end) {
        (_, true, _) => "tile-start",
        (_, _, true) => "tile-end",
        (Tile::None, _, _) => "tile-none",
        (Tile::Wall, _, _) => "tile-wall",
    };

    let on_mouse_check = {
        let on_tile_click = props.on_tile_click.clone();

        Callback::from(move |e: MouseEvent| {
            const LEFT_MOUSE_BUTTON_BITMASK: u16 = 1;
            let mouse_down = e.buttons().bitand(LEFT_MOUSE_BUTTON_BITMASK) != 0;

            if mouse_down {
                on_tile_click.emit(())
            }
        })
    };

    let prevent_drag = { Callback::from(move |e: DragEvent| e.prevent_default()) };

    html!(
        <div class={classes!("tile", class)} onmousedown={on_mouse_check.clone()} onmouseenter={on_mouse_check} ondragstart={prevent_drag} />
    )
}
