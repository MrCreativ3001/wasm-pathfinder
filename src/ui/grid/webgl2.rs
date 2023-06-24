use crate::pathfinders::{Grid, Pos, Tile};
use crate::ui::grid::GridProps;
use js_sys::Float32Array;
use std::cell::RefCell;

use std::rc::Rc;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{
    window, HtmlCanvasElement, MouseEvent, WebGl2RenderingContext as GL, WebGlBuffer, WebGlProgram,
    WebGlUniformLocation, WebGlVertexArrayObject,
};
use yew::{classes, html, Callback, Component, Context, Html, NodeRef};

pub struct WebGL2GridComponent {
    node_ref: NodeRef,
    grid_sender: Sender<VisualState>,
    grid_receiver: Option<Receiver<VisualState>>,
    old_pos: Option<Pos>,
    is_dragging_start: bool, // Old tile is start
    is_dragging_end: bool,   // Old tile is end
}

pub enum WebGL2GridMsg {
    MouseEvent { event: MouseEvent },
}

impl Component for WebGL2GridComponent {
    type Message = WebGL2GridMsg;
    type Properties = GridProps;

    fn create(ctx: &Context<Self>) -> Self {
        let (sender, receiver) = mpsc::channel();
        // The first message we send is the initial state
        sender
            .send(VisualState::from(ctx.props()))
            .expect("Failed to send visual state to render thread!");
        Self {
            node_ref: NodeRef::default(),
            grid_sender: sender,
            grid_receiver: Some(receiver),
            old_pos: None,
            is_dragging_start: false,
            is_dragging_end: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            WebGL2GridMsg::MouseEvent { event } => {
                let grid = &ctx.props().grid;

                let new_pos = match Self::mouse_event_to_tile(&event, grid.width(), grid.height()) {
                    Some(pos) => pos,
                    None => return false,
                };
                const LEFT_MOUSE_BUTTON: u16 = 1;
                let mouse_down = event.buttons() & LEFT_MOUSE_BUTTON != 0;

                if mouse_down {
                    if ctx.props().grid.start() == new_pos || self.is_dragging_start {
                        self.is_dragging_start = true;
                        if let Some(old_pos) = self.old_pos {
                            if old_pos != new_pos {
                                ctx.props().on_start_move.emit(new_pos);
                            }
                        }
                        self.old_pos = Some(new_pos);
                    } else if ctx.props().grid.end() == new_pos || self.is_dragging_end {
                        self.is_dragging_end = true;
                        if let Some(old_pos) = self.old_pos {
                            if old_pos != new_pos {
                                ctx.props().on_end_move.emit(new_pos);
                            }
                        }
                        self.old_pos = Some(new_pos);
                    } else {
                        if let Some(old_pos) = self.old_pos {
                            if old_pos != new_pos {
                                ctx.props().on_tile_click.emit(new_pos);
                            }
                        } else {
                            ctx.props().on_tile_click.emit(new_pos);
                        }
                        self.old_pos = Some(new_pos);
                    }
                } else {
                    self.old_pos = None;
                    self.is_dragging_start = false;
                    self.is_dragging_end = false;
                }
            }
        }

        // We don't need to rerender ourselves because we're just a canvas
        false
    }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        let old_state = VisualState::from(old_props);
        let new_state = VisualState::from(ctx.props());

        if old_state != new_state {
            self.grid_sender
                .send(new_state)
                .expect("Failed to send grid to render thread! Did the render loop crash?");
        }

        // We don't need to rerender ourselves because we're just a canvas
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_mouse_event = {
            let grid = &ctx.props().grid;
            let (_grid_rows, _grid_columns) = (grid.height(), grid.width());

            let link = ctx.link().clone();
            Callback::from(move |event: MouseEvent| {
                link.send_message(WebGL2GridMsg::MouseEvent { event });
            })
        };

        html!(
            <canvas class={classes!("grid", "canvas-grid")} ref={self.node_ref.clone()} width={1000} height={1000} onmousemove={on_mouse_event.clone()} onmousedown={on_mouse_event.clone()} onmouseup={on_mouse_event} />
        )
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let canvas = self
                .node_ref
                .cast::<HtmlCanvasElement>()
                .expect("Unable to cast canvas");
            let gl: GL = canvas
                .get_context("webgl2")
                .expect("Unable to get webgl context")
                .expect("Unable to get webgl context")
                .dyn_into()
                .expect("Unable to cast webgl context");
            self.start_render_loop(gl);
        }
    }
}

impl WebGL2GridComponent {
    fn tile_size(canvas_width: i32, canvas_height: i32, grid_width: i32, grid_height: i32) -> f32 {
        let canvas_shortest_side = canvas_width.min(canvas_height);

        let grid_longest_side = grid_width.max(grid_height);

        canvas_shortest_side as f32 / grid_longest_side as f32
    }

    fn mouse_event_to_tile(event: &MouseEvent, grid_width: i32, grid_height: i32) -> Option<Pos> {
        let canvas_element = event
            .target()
            .expect("Unable to get target")
            .dyn_into::<HtmlCanvasElement>()
            .expect("Unable to cast target");
        let rect = canvas_element.get_bounding_client_rect();

        let tile_size = Self::tile_size(
            rect.width() as i32,
            rect.height() as i32,
            grid_width,
            grid_height,
        );

        let mouse_x = event.client_x() as f32 - rect.left() as f32;
        let mouse_y = event.client_y() as f32 - rect.top() as f32;

        let tile_x = (mouse_x / tile_size) as i32;
        let tile_y = (mouse_y / tile_size) as i32;

        if tile_x >= 0 && tile_x < grid_width && tile_y >= 0 && tile_y < grid_height {
            Some(Pos {
                x: tile_x,
                y: tile_y,
            })
        } else {
            None
        }
    }

    fn start_render_loop(&mut self, gl: GL) {
        let mut grid_renderer = GlGridRenderer::new(gl);
        grid_renderer.init_gl();

        let callback = {
            let grid_receiver = self
                .grid_receiver
                .take()
                .expect("Grid receiver already taken!");

            // Create a callback that will be called every frame
            let callback = Rc::new(RefCell::new(None));
            *callback.borrow_mut() = Some(Closure::wrap(Box::new({
                let callback = callback.clone();
                move || {
                    // Use try_recv to avoid blocking
                    let state = grid_receiver.try_recv();

                    if let Ok(state) = state {
                        grid_renderer.update_state(state);
                    } else if let Err(TryRecvError::Disconnected) = state {
                        // The sender has been dropped, so we should stop rendering
                        grid_renderer.cleanup();
                        return;
                    }

                    grid_renderer.render();

                    // Request another frame
                    Self::request_animation_frame(callback.borrow().as_ref().unwrap());
                }
            }) as Box<dyn FnMut()>));

            callback
        };

        Self::request_animation_frame(callback.borrow().as_ref().unwrap());
    }
    fn request_animation_frame(callback: &Closure<dyn FnMut()>) {
        window()
            .expect("Unable to get window")
            .request_animation_frame(callback.as_ref().unchecked_ref())
            .expect("Failed to register animation frame callback");
    }
}

#[derive(Debug, Clone, PartialEq)]
struct VisualState {
    grid: Grid,
    path: Vec<Pos>,
    visited: Vec<Pos>,
}

impl From<&GridProps> for VisualState {
    fn from(props: &GridProps) -> Self {
        let grid = props.grid.clone();
        let path = props.path.clone();
        let visited = props.visited.clone();

        Self {
            grid,
            path,
            visited,
        }
    }
}

struct GlGridRenderer {
    state: Option<VisualState>,
    tile_size: f32,
    // WebGl
    gl: GL,
    shader_program: Option<WebGlProgram>,
    vao: Option<WebGlVertexArrayObject>,
    // buffers
    position_buffer: Option<WebGlBuffer>,
    // shader program locations
    resolution_location: Option<WebGlUniformLocation>,
    tile_size_location: Option<WebGlUniformLocation>,
    position_location: Option<WebGlUniformLocation>,
    color_location: Option<WebGlUniformLocation>,
}

impl GlGridRenderer {
    fn new(gl: GL) -> Self {
        Self {
            state: None,
            tile_size: 0.0,
            gl,
            shader_program: None,
            vao: None,
            position_buffer: None,
            resolution_location: None,
            tile_size_location: None,
            position_location: None,
            color_location: None,
        }
    }

    fn init_gl(&mut self) {
        let gl = &self.gl;

        // Set viewport
        gl.viewport(0, 0, gl.drawing_buffer_width(), gl.drawing_buffer_height());

        // Load vertex shader
        const VERTEX_SHADER: &str = include_str!("webgl2/basic.vert");
        let vertex_shader = gl
            .create_shader(GL::VERTEX_SHADER)
            .expect("Unable to create shader");
        gl.shader_source(&vertex_shader, VERTEX_SHADER);
        gl.compile_shader(&vertex_shader);
        if !gl
            .get_shader_parameter(&vertex_shader, GL::COMPILE_STATUS)
            .as_bool()
            .expect("Unable to get shader parameter")
        {
            panic!("Failed to compile vertex shader");
        }

        // Load fragment shader
        const FRAGMENT_SHADER: &str = include_str!("webgl2/basic.frag");
        let fragment_shader = gl
            .create_shader(GL::FRAGMENT_SHADER)
            .expect("Unable to create shader");
        gl.shader_source(&fragment_shader, FRAGMENT_SHADER);
        gl.compile_shader(&fragment_shader);
        if !gl
            .get_shader_parameter(&fragment_shader, GL::COMPILE_STATUS)
            .as_bool()
            .expect("Unable to get shader parameter")
        {
            panic!("Failed to compile fragment shader");
        }

        // Create/Use shader program
        let shader_program = gl
            .create_program()
            .expect("Unable to create shader program");
        gl.attach_shader(&shader_program, &vertex_shader);
        gl.attach_shader(&shader_program, &fragment_shader);
        gl.link_program(&shader_program);
        if !gl
            .get_program_parameter(&shader_program, GL::LINK_STATUS)
            .as_bool()
            .expect("Unable to get program parameter")
        {
            panic!("Failed to link shader program");
        }
        self.shader_program = Some(shader_program);
        let shader_program = self.shader_program.as_ref().unwrap();

        gl.use_program(Some(shader_program));

        // Delete shaders
        gl.delete_shader(Some(&vertex_shader));
        gl.delete_shader(Some(&fragment_shader));

        // get shader locations
        self.resolution_location = gl.get_uniform_location(shader_program, "u_resolution");
        self.tile_size_location = gl.get_uniform_location(shader_program, "u_tile_size");
        self.position_location = gl.get_uniform_location(shader_program, "u_position");
        self.color_location = gl.get_uniform_location(shader_program, "u_color");

        // Create/Load position buffer
        const ZERO: f32 = 0.025;
        const ONE: f32 = 0.975;
        let position_content = [
            // We don't want to use 0 or 1 but smaller numbers to have lines between the tiles
            ZERO, ZERO, ZERO, ONE, ONE, ZERO, // First triangle
            ONE, ZERO, ZERO, ONE, ONE, ONE, // Second triangle
        ];

        let position_buffer = gl.create_buffer().expect("Unable to create buffer");
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&position_buffer));
        gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER,
            &Float32Array::from(position_content.as_slice()),
            GL::STATIC_DRAW,
        );

        // Create/Bind VAO
        self.vao = Some(
            gl.create_vertex_array()
                .expect("Unable to create vertex array"),
        );
        gl.bind_vertex_array(self.vao.as_ref());

        // Enable position attribute
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&position_buffer));
        let position_location = gl.get_attrib_location(shader_program, "a_position");
        gl.vertex_attrib_pointer_with_i32(position_location as u32, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position_location as u32);

        // Unbind VAO
        gl.bind_vertex_array(None);

        // Save position buffer
        self.position_buffer = Some(position_buffer);
    }

    fn update_state(&mut self, state: VisualState) {
        let gl = &self.gl;

        let grid = &state.grid;
        self.tile_size = WebGL2GridComponent::tile_size(
            gl.drawing_buffer_width(),
            gl.drawing_buffer_height(),
            grid.height(),
            grid.width(),
        );

        self.state = Some(state);
    }

    fn render(&self) {
        let gl = &self.gl;

        // Update viewport
        gl.viewport(0, 0, gl.drawing_buffer_width(), gl.drawing_buffer_height());

        // Clear the screen
        gl.clear_color(1.0, 1.0, 1.0, 1.0);
        gl.clear(GL::COLOR_BUFFER_BIT);

        // Draw
        self.draw_grid();
    }

    fn draw_grid(&self) {
        let state = &self.state;
        if state.is_none() {
            return;
        }
        let state = state.as_ref().unwrap();
        let grid = &state.grid;
        let start = grid.start();
        let end = grid.end();

        for x in 0..grid.width() {
            for y in 0..grid.height() {
                let pos = Pos { x, y };

                let tile = grid.tile(pos);
                let is_start = pos == start;
                let is_end = pos == end;
                let is_path = state.path.contains(&pos);
                let is_visited = state.visited.contains(&pos);

                let color = match (tile, is_start, is_end, is_path, is_visited) {
                    (_, true, _, _, _) => Color::TILE_START,
                    (_, _, true, _, _) => Color::TILE_END,
                    (Tile::Wall, _, _, _, _) => Color::TILE_WALL,
                    (_, _, _, _, true) => Color::TILE_VISITED,
                    (_, _, _, true, _) => Color::TILE_PATH,
                    (Tile::None, _, _, _, _) => Color::TILE_NONE,
                };
                self.draw_tile(x as f32, y as f32, color);
            }
        }
    }

    fn draw_tile(&self, x: f32, y: f32, color: Color) {
        let gl = &self.gl;

        // Bind shader program
        gl.use_program(self.shader_program.as_ref());

        // Bind VAO
        gl.bind_vertex_array(self.vao.as_ref());

        // Set uniforms
        gl.uniform2f(
            self.resolution_location.as_ref(),
            gl.drawing_buffer_width() as f32,
            gl.drawing_buffer_height() as f32,
        );
        gl.uniform2f(
            self.tile_size_location.as_ref(),
            self.tile_size,
            self.tile_size,
        );
        gl.uniform2f(self.position_location.as_ref(), x, y);
        gl.uniform3f(self.color_location.as_ref(), color.r, color.g, color.b);

        // Draw
        gl.draw_arrays(GL::TRIANGLES, 0, 6);
    }
    /// After this function is called, no other function on this should be called
    fn cleanup(&self) {
        let gl = &self.gl;
        gl.delete_program(Some(self.shader_program.as_ref().unwrap()));
        gl.delete_vertex_array(Some(self.vao.as_ref().unwrap()));
        gl.delete_buffer(self.position_buffer.as_ref());
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

macro_rules! color_rgb_255 {
    ($r:expr, $g:expr, $b:expr) => {
        Color {
            r: ($r as f32) / 255.0,
            g: ($g as f32) / 255.0,
            b: ($b as f32) / 255.0,
        }
    };
}

impl Color {
    pub const TILE_NONE: Color = color_rgb_255!(52, 206, 255);
    pub const TILE_WALL: Color = color_rgb_255!(0, 0, 0);
    pub const TILE_START: Color = color_rgb_255!(0, 255, 0);
    pub const TILE_END: Color = color_rgb_255!(255, 0, 0);
    pub const TILE_PATH: Color = color_rgb_255!(255, 255, 0);
    pub const TILE_VISITED: Color = color_rgb_255!(255, 0, 255);
}
