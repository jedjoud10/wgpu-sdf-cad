use std::{sync::Arc, num::NonZeroU32};

use egui::{ViewportId, FullOutput};
use hierarchy::Hierarchy;
use render::PersistentRenderState;
use wgpu::{PowerPreference, PresentMode, Backends, TextureFormat};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

mod interface;
mod render;
mod hierarchy;
mod gpu;

fn main() {    
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let config = egui_wgpu::WgpuConfiguration {
        supported_backends: Backends::all(),
        device_descriptor: Arc::new(|_| wgpu::DeviceDescriptor::default()),
        present_mode: PresentMode::Immediate,
        power_preference: PowerPreference::HighPerformance,
        on_surface_error: Arc::new(|x| panic!("{}", x)),
    };

    let mut painter = egui_wgpu::winit::Painter::new(
        config,
        1,
        Some(TextureFormat::Depth32Float),
        false,
    );

    pollster::block_on(painter.set_window(ViewportId::ROOT, Some(&window))).unwrap();
    let context = egui::Context::default();
    let state = egui_winit::State::new(ViewportId::ROOT, &window, None, painter.max_texture_side());
    let render_state = &painter.render_state().unwrap();
    let render_state = PersistentRenderState::new(&render_state.device, render_state.target_format);

    let mut interface = interface::World {
        ctx: context,
        persistent: Arc::new(render_state),
        painter,
        state,
        hierarchy: Hierarchy::default(),
    };

    event_loop.run(move |event, _, flow| {
        match &event {
            Event::MainEventsCleared => {
                let input = interface.state.take_egui_input(&window);
                interface.ctx.begin_frame(input);

                interface::draw(&mut interface);

                let FullOutput { 
                    textures_delta,
                    shapes,
                    pixels_per_point,
                    ..
                } = interface.ctx.end_frame();

                let clipped_primitives = interface.ctx.tessellate(shapes, pixels_per_point);
                
                interface.painter.paint_and_update_textures(
                    ViewportId::ROOT,
                    pixels_per_point, [0.0; 4],
                    &clipped_primitives,
                    &textures_delta,
                    false
                );
            },
            Event::WindowEvent { event, .. } => {
                match &event {
                    WindowEvent::Resized(x) => {
                        interface.painter.on_window_resized(ViewportId::ROOT, NonZeroU32::new(x.width).unwrap(), NonZeroU32::new(x.height).unwrap());
                    },
                    WindowEvent::CloseRequested => {
                        flow.set_exit();
                    }
                    _ => {}
                }

                let _ = interface.state.on_window_event(&interface.ctx, &event);
            }
            _ => ()
        }
    });

}
