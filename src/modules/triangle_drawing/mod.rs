//! Triangle drawing.

use super::{tas_optimizer, Module};
use crate::hooks::engine::{self};
use crate::utils::*;

pub mod triangle_api;
pub use triangle_api::TriangleApi;

pub struct TriangleDrawing;
impl Module for TriangleDrawing {
    fn name(&self) -> &'static str {
        "Triangle drawing"
    }

    fn description(&self) -> &'static str {
        "Makes bxt-rs able to draw shapes in the 3D space."
    }

    fn is_enabled(&self, marker: MainThreadMarker) -> bool {
        engine::ClientDLL_DrawTransparentTriangles.is_set(marker) && engine::tri.is_set(marker)
    }
}

pub unsafe fn on_draw_transparent_triangles(marker: MainThreadMarker) {
    if !TriangleDrawing.is_enabled(marker) {
        return;
    }

    let tri = TriangleApi::new(&*engine::tri.get(marker));

    // TODO: set white texture.

    tas_optimizer::draw(marker, &tri);

    // Required for the WON DLLs.
    tri.render_mode(triangle_api::RenderMode::Normal);
}
