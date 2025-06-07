use std::sync::Arc;
use std::time::{Duration, Instant};

use skia_safe::{Canvas, Color, Paint, Point};

use crate::profiling::tracy_zone;
use crate::renderer::fonts::font_loader::{FontKey, FontLoader, FontPair};
use crate::settings::Settings;

pub struct ReconnectIndicator {
    font: Arc<FontPair>,
    active: bool,
    address: String,
    end_time: Instant,
    angle: f32,
    #[allow(dead_code)]
    settings: Arc<Settings>,
}

impl ReconnectIndicator {
    pub fn new(settings: Arc<Settings>) -> Self {
        let font_key = FontKey::default();
        let mut loader = FontLoader::new(16.0);
        let font = loader.get_or_load(&font_key).expect("Font load failed");
        Self {
            font,
            active: false,
            address: String::new(),
            end_time: Instant::now(),
            angle: 0.0,
            settings,
        }
    }

    pub fn start(&mut self, address: String, wait: Duration) {
        self.address = address;
        self.end_time = Instant::now() + wait;
        self.angle = 0.0;
        self.active = true;
    }

    pub fn stop(&mut self) {
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn update(&mut self, dt: f32) {
        if self.active {
            self.angle += dt * std::f32::consts::PI * 2.0;
            if self.angle > std::f32::consts::PI * 2.0 {
                self.angle -= std::f32::consts::PI * 2.0;
            }
        }
    }

    pub fn draw(&self, canvas: &Canvas) {
        tracy_zone!("reconnect_indicator_draw");
        if !self.active {
            return;
        }
        let remaining = self.end_time.saturating_duration_since(Instant::now());
        let secs = remaining.as_secs_f32().ceil() as u64;
        let text = format!("Reconnecting to {} in {}s", self.address, secs);

        canvas.save();

        let mut paint = Paint::default();
        paint.set_anti_alias(true);

        let spinner_radius = self.font.skia_font.size();
        let center = (40.0, 40.0);

        paint.set_color(Color::from_argb(255, 80, 80, 80));
        canvas.draw_circle(center, spinner_radius, &paint);

        paint.set_color(Color::WHITE);
        let arm_x = center.0 + spinner_radius * self.angle.cos();
        let arm_y = center.1 + spinner_radius * self.angle.sin();
        canvas.draw_line(center, (arm_x, arm_y), &paint);

        let text_pos = Point::new(
            center.0 + spinner_radius + 10.0,
            center.1 + self.font.skia_font.size() / 2.0,
        );
        canvas.draw_str(text, text_pos, &self.font.skia_font, &paint);

        canvas.restore();
    }
}
