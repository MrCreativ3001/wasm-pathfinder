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
    let _pathfinder = use_state(|| PathFinders::AStar);

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

    html!(
        <div>
          <Options />
          <GridComponent grid={grid.deref().clone()} on_tile_click={on_tile_click}/>
        </div>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
