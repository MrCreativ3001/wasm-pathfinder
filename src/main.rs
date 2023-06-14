use crate::pathfinders::{Grid, PathFinders, Pos, Tile};
use crate::ui::options::Options;
use std::ops::Deref;
use ui::grid::GridComponent;
use yew::prelude::*;

mod pathfinders;
mod ui;

#[function_component]
fn App() -> Html {
    let grid = use_state(|| Grid::new(10, 10, Pos { x: 0, y: 0 }, Pos { x: 9, y: 9 }));
    let cached_path: UseStateHandle<Vec<Pos>> = use_state(|| Vec::with_capacity(0));

    let on_tile_click = {
        let grid = grid.clone();
        Callback::from(move |pos| {
            let mut grid_mut = grid.deref().clone();

            let tile = grid_mut.tile(pos);
            let tile = match tile {
                Tile::None => Tile::Wall,
                Tile::Wall => Tile::None,
            };
            grid_mut.set_tile(pos, tile);

            grid.set(grid_mut);
        })
    };

    let on_find_path = {
        let grid = grid.clone();
        let cached_path = cached_path.clone();

        Callback::from(move |pathfinder: PathFinders| {
            let grid = grid.deref();
            let path = pathfinder.find_path(grid);

            cached_path.set(path.unwrap_or(Vec::with_capacity(0)));
        })
    };

    html!(
        <div>
          <Options on_find_path={on_find_path} />
          <GridComponent grid={grid.deref().clone()} path={cached_path.deref().clone()} on_tile_click={on_tile_click}/>
        </div>
    )
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
