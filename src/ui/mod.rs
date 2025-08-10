use smithay::utils::{Logical, Point, Rectangle, Size};

use crate::layout::focus_ring::{FocusRing, FocusRingRenderElement};
use crate::render_helpers::renderer::NiriRenderer;

pub mod config_error_notification;
pub mod exit_confirm_dialog;
pub mod hotkey_overlay;
pub mod screen_transition;
pub mod screenshot_ui;

pub struct DialogueBackground {
    background: FocusRing,
    border: FocusRing,
}

impl DialogueBackground {
    pub fn new(background: niri_config::Border, border: niri_config::Border) -> Self {
        Self {
            background: FocusRing::new(background.into()),
            border: FocusRing::new(border.into()),
        }
    }

    pub fn update_config(&mut self, background: niri_config::Border, border: niri_config::Border) {
        self.background.update_config(background.into());
        self.border.update_config(border.into());
    }

    pub fn update_render_elements(
        &mut self,
        size: Size<f64, Logical>,
        view_rect: Rectangle<f64, Logical>,
        radius: niri_config::CornerRadius,
        scale: f64,
        alpha: f32,
    ) {
        self.border
            .update_render_elements(size, true, true, false, view_rect, radius, scale, alpha);
        self.background
            .update_render_elements(size, true, false, false, view_rect, radius, scale, alpha);
    }

    pub fn render(
        &self,
        renderer: &mut impl NiriRenderer,
        location: Point<f64, Logical>,
    ) -> impl Iterator<Item = FocusRingRenderElement> {
        self.border
            .render(renderer, location)
            .chain(self.background.render(renderer, location))
    }
}
