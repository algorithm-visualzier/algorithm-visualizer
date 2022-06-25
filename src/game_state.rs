use std::error::Error;

use egui_tetra::egui;
use egui_tetra::egui::CtxRef;
use tetra::graphics::scaling::{ScalingMode, ScreenScaler};
use tetra::graphics::text::Font;
use tetra::graphics::{self, Camera, Color, FilterMode};

use tetra::input::MouseButton;
use tetra::Context;

use crate::camera_handling::camera_state::CameraState;
use crate::graph::{Graph, GraphOnCanvas};
use crate::input::input_state::{InputState, StateData};
use crate::step_algorithms::StepAlgorithmResult;
use crate::ui::ui_drawing::create_ui;
use crate::ui::ui_state::UiData;

use crate::constants::{FONT_SIZE_SQUARED, SCREEN_HEIGHT, SCREEN_WIDTH};

pub enum AppMode {
    Write,
    Normal,
}

pub struct GameState {
    pub graph: Graph,
    // This is problematic to make nonpublic.
    //todo pack the junk into a struct
    pub input_state: InputState,
    camera: Camera,

    scaler: ScreenScaler,

    pub ui_data: UiData,
    font: Font,
    mode: AppMode,

    algorithm: Option<StepAlgorithmResult>,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameState {
        GameState {
            graph: Graph::new(),
            input_state: InputState::Move(StateData::default()),
            camera: Camera::new(SCREEN_WIDTH, SCREEN_HEIGHT),
            scaler: ScreenScaler::with_window_size(
                ctx,
                SCREEN_WIDTH as i32,
                SCREEN_HEIGHT as i32,
                ScalingMode::ShowAllPixelPerfect,
            )
            .unwrap(),
            ui_data: UiData::new(),
            algorithm: None,
            font: {
                let mut font = Font::vector(
                    ctx,
                    "resources/fonts/JetBrainsMono-Regular.ttf",
                    FONT_SIZE_SQUARED,
                )
                .unwrap();
                font.set_filter_mode(ctx, FilterMode::Linear);
                font
            },
            mode: AppMode::Normal,
        }
    }

    pub fn add_algorithm(&mut self, mut algorithm_res: StepAlgorithmResult) {
        algorithm_res.show_algorithm(&mut self.graph);
        self.algorithm = Some(algorithm_res);
    }

    pub fn font(&self) -> Font {
        self.font.clone()
    }
}

impl egui_tetra::State<Box<dyn Error>> for GameState {
    fn ui(&mut self, ctx: &mut Context, egui_ctx: &CtxRef) -> Result<(), Box<dyn Error>> {
        create_ui(self, ctx, egui_ctx);

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context, egui_ctx: &CtxRef) -> Result<(), Box<dyn Error>> {
        self.graph.update(
            ctx,
            egui_ctx,
            self.ui_data.push_conf(),
            self.ui_data.pull_conf(),
            &self.camera,
            &mut self.mode,
        );

        if let Some(alg) = &mut self.algorithm {
            alg.update(ctx, &mut self.graph);
        }

        if let AppMode::Normal = self.mode {
            self.camera.update_camera_transformation(ctx)
        } else {
            Ok(())
        }
    }

    fn draw(&mut self, ctx: &mut Context, egui_ctx: &egui::CtxRef) -> Result<(), Box<dyn Error>> {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        graphics::set_transform_matrix(ctx, self.camera.as_matrix());

        self.graph.draw(
            self.camera.mouse_position(ctx),
            ctx,
            egui_ctx,
            self.camera.rotation,
            self.ui_data.is_directed(),
        );

        graphics::reset_transform_matrix(ctx);

        self.scaler.draw(ctx);

        Ok(())
    }

    fn event(
        &mut self,
        ctx: &mut Context,
        _egui_ctx: &CtxRef,
        event: tetra::Event,
    ) -> Result<(), Box<dyn Error>> {
        if let tetra::Event::MouseMoved { .. } = &event {
            self.input_state
                .on_mouse_drag(ctx, &mut self.graph, self.camera.mouse_position(ctx));
        }

        if let tetra::Event::MouseButtonPressed {
            button: MouseButton::Left,
        } = &event
        {
            self.input_state.on_left_click(
                ctx,
                &mut self.graph,
                self.camera.mouse_position(ctx),
                self.font.clone(),
            );
        }

        if let tetra::Event::MouseButtonPressed {
            button: MouseButton::Right,
        } = &event
        {
            if self
                .graph
                .node_from_point(self.camera.mouse_position(ctx))
                .is_some()
            {
                self.mode = AppMode::Write;
            }
        }

        if self
            .graph
            .node_from_point(self.camera.mouse_position(ctx))
            .is_none()
        {
            self.mode = AppMode::Normal;
        }

        if let AppMode::Normal = self.mode {
            self.camera.handle_camera_events(event)
        } else {
            Ok(())
        }
    }
}
