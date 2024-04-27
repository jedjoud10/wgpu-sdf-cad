use std::{rc::Rc, sync::Arc};
use egui::*;
use vek::*;
use crate::{hierarchy::{Hierarchy, Object, Shape}, render::{PersistentRenderState, RenderCallback}};

pub struct World {
    pub ctx: egui::Context,
    pub painter: egui_wgpu::winit::Painter,
    pub state: egui_winit::State,
    pub hierarchy: Hierarchy,
    pub persistent: Arc<PersistentRenderState>
}

pub fn draw(world: &mut World) {
    let World {
        ctx,
        painter,
        state,
        hierarchy,
        ..
    } = world;

    TopBottomPanel::top("Menu Bar").show(&ctx, |ui| {
        ui.horizontal(|ui| {
            ui.menu_button("Add", |ui| {
                if ui.button("Cuboid").clicked() {
                    hierarchy.push(Rc::new(Object {
                        name: "Cuboid".to_string(),
                        shape: Shape::Cuboid { half_extents: Vec3::one(), offset: Vec3::zero() },
                        aabb: Aabb::default(),
                    }))
                } 
            });
        })
    });

    SidePanel::left("Hierarchy").show(&ctx, |ui| {
        ui.separator();

        ScrollArea::vertical().show(ui, |ui| {
            for obj in hierarchy.iter() {
            }            
        });

    });

    let res = CentralPanel::default().show(&ctx, |ui| {
        let rect = ui.available_rect_before_wrap();
        let callback = egui_wgpu::Callback::new_paint_callback(rect, RenderCallback { persistent: world.persistent.clone() });
        ui.painter().add(callback);
    });
}