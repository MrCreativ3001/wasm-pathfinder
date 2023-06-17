use crate::pathfinders::{Grid, PathFindAlgorithm, PathFindAlgorithms, Pos, Tile};
use crate::ui::options::{GridOptions, Options};
use gloo::timers::callback::Interval;
use std::ops::Deref;
use ui::grid::GridComponent;
use yew::prelude::*;

mod pathfinders;
mod ui;

#[function_component]
fn App() -> Html {
    let default_grid_options = GridOptions {
        rows: 10,
        columns: 10,
        start_pos: Pos { x: 0, y: 0 },
        end_pos: Pos { x: 9, y: 9 },
    };

    let grid: UseStateHandle<Grid> = use_state(|| GridOptions::into(default_grid_options));
    let path_finder_state = use_mut_ref::<Option<Box<dyn PathFindAlgorithm>>, _>(|| None);
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
        let path_finder_state = path_finder_state.clone();

        Callback::from(move |pathfinder: PathFindAlgorithms| {
            let grid = grid.deref();

            let new_state = pathfinder.make_state(grid.clone());

            path_finder_state.replace_with(|_| Some(new_state));
        })
    };

    {
        let path_finder_state = path_finder_state.clone();
        let cached_path = cached_path.clone();

        use_effect_with_deps(
            move |_| {
                let interval = Interval::new(50, move || {
                    let path_finder_state_rc = path_finder_state.clone();
                    let mut path_finder_state_ref = path_finder_state_rc.borrow_mut();
                    let path_finder_state = match path_finder_state_ref.as_mut() {
                        Some(state) => state,
                        None => return,
                    };

                    match path_finder_state.next_step() {
                        Ok(path) => {
                            cached_path.set(path);
                            // drop the reference to the state, because it would be still in use when we replace it (leading to a panic)
                            drop(path_finder_state_ref);
                            path_finder_state_rc.replace_with(|_| None);
                        }
                        Err(_) => {
                            if !cached_path.is_empty() {
                                cached_path.set(Vec::with_capacity(0));
                            }
                            // rerender the grid component (this is a hack, because we need to change the state of it to rerender it)
                            cached_path.set(cached_path.deref().clone());
                        }
                    };
                });

                // while we still own the interval, it will keep running, for cleanup we need to drop it
                move || drop(interval)
            },
            grid.clone(),
        );
    }

    let on_grid_options_change = {
        let grid = grid.clone();
        let cached_path = cached_path.clone();
        let path_finder_state = path_finder_state.clone();

        Callback::from(move |new_options: GridOptions| {
            let new_grid: Grid = new_options.into();

            path_finder_state.replace_with(|_| None);
            grid.set(new_grid);
            cached_path.set(Vec::with_capacity(0));
        })
    };

    let grid_component_visited = {
        let path_finder_state = path_finder_state.clone();

        Callback::from(move |pos| {
            let path_finder_state_rc = path_finder_state.clone();
            let path_finder_state_ref = path_finder_state_rc.borrow();
            let path_finder_state = match path_finder_state_ref.as_ref() {
                Some(state) => state,
                None => return false,
            };
            path_finder_state.visited(pos)
        })
    };

    html!(
        <div>
          <Options on_find_path={on_find_path} default_grid_options={default_grid_options} on_grid_options_change={on_grid_options_change} />
          <GridComponent grid={grid.deref().clone()} path={cached_path.deref().clone()} visited={grid_component_visited} on_tile_click={on_tile_click}/>
        </div>
    )
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
