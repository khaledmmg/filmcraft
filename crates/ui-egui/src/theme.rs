//! Design tokens. Every widget reads colours/sizes from [`Tokens`] so themes apply everywhere.
//!
//! Default theme: simple, Apple-like light UI with muted neutrals, soft borders and rounded
//! surfaces. Dark mode remains available as a user preference.

use std::sync::Arc;

use egui::{Color32, FontData, FontDefinitions, FontFamily, FontId, Stroke, TextStyle, Visuals};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeKind {
    /// Minimal Apple-like light mode (default).
    #[default]
    Light,
    /// Slightly lighter grey panels (Premiere's brightness slider mid position).
    Medium,
    /// Premiere-style darkest (legacy dark mode).
    Dark,
}

impl ThemeKind {
    /// Settings ▸ Appearance ▸ Color Theme value (`darkest`, `dark`, `light`).
    pub fn pref_name(self) -> &'static str {
        match self {
            ThemeKind::Dark => "darkest",
            ThemeKind::Medium => "dark",
            ThemeKind::Light => "light",
        }
    }
    pub fn from_pref(s: &str) -> ThemeKind {
        match s {
            "dark" => ThemeKind::Medium,
            "light" => ThemeKind::Light,
            _ => ThemeKind::Light,
        }
    }
    pub fn from_name(s: &str) -> Option<ThemeKind> {
        match s.to_ascii_lowercase().as_str() {
            "dark" | "darkest" => Some(ThemeKind::Dark),
            "medium" | "grey" | "gray" => Some(ThemeKind::Medium),
            "light" => Some(ThemeKind::Light),
            _ => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tokens {
    pub kind: ThemeKind,
    pub app_bg: Color32,
    pub header_bg: Color32,
    pub panel_bg: Color32,
    pub tab_bg: Color32,
    pub tab_text: Color32,
    pub tab_text_active: Color32,
    pub focus: Color32,
    pub accent: Color32,
    pub accent_hover: Color32,
    pub text: Color32,
    pub text_dim: Color32,
    pub text_faint: Color32,
    pub icon: Color32,
    pub icon_active: Color32,
    pub hover: Color32,
    pub pressed: Color32,
    pub field_bg: Color32,
    pub field_border: Color32,
    pub separator: Color32,
    pub row_alt: Color32,
    pub row_selected: Color32,
    pub hot_text: Color32,
    pub pill_active_bg: Color32,
    pub clip_bar_bg: Color32,
    pub control_surface: Color32,
    pub control_well: Color32,
    pub control_border: Color32,
    pub control_handle_bg: Color32,
    pub control_handle: Color32,
    pub control_handle_dim: Color32,
    pub plot_bg: Color32,
    pub plot_grid: Color32,
    pub plot_axis: Color32,
    pub plot_handle: Color32,
    pub plot_handle_dim: Color32,
    pub meter_bg: Color32,
    pub meter_readout_bg: Color32,
    pub meter_clip_off: Color32,
    pub panner_bg: Color32,
    pub panner_border: Color32,
    pub panner_speaker: Color32,
    pub fader_knob: Color32,
    pub eq_track: Color32,
    pub eq_tick: Color32,
    pub eq_knob: Color32,
    pub eq_knob_active: Color32,
    pub eq_node: Color32,
    pub eq_node_off: Color32,
    pub crossover_handle: Color32,
    pub keyframe_plot_bg: Color32,
    pub keyframe_plot_grid: Color32,
    pub keyframe_plot_axis: Color32,
    pub keyframe_handle: Color32,
    pub switch_fill: Color32,
    pub switch_knob: Color32,
    pub switch_border: Color32,
    pub slider_disabled_track: Color32,
    pub slider_disabled_knob: Color32,
    pub button_hover: Color32,
    pub button_border: Color32,
    pub caption_track_bg: Color32,
    pub scroll_thumb: Color32,
    pub scroll_thumb_hover: Color32,
    pub vertical_scroll_thumb: Color32,
    pub disabled_clip_bg: Color32,
    pub tl_bg: Color32,
    pub tl_track_bg: Color32,
    pub tl_track_bg_alt: Color32,
    pub tl_header_bg: Color32,
    pub tl_ruler_bg: Color32,
    pub tl_ruler_tick: Color32,
    pub tl_ruler_text: Color32,
    pub playhead: Color32,
    pub track_target: Color32,
    pub in_out_shade: Color32,
    pub clip_selected_border: Color32,
    pub render_red: Color32,
    pub render_yellow: Color32,
    pub render_green: Color32,
    pub monitor_bg: Color32,
    pub timecode: Color32,
    pub danger: Color32,
    pub radius: f32,
    pub radius_sm: f32,
    pub gap: f32,
    pub tab_h: f32,
}

impl Tokens {
    pub fn for_kind(kind: ThemeKind) -> Self {
        let dark = Tokens {
            kind,
            app_bg: Color32::from_rgb(0, 0, 0),
            header_bg: Color32::from_rgb(0x1d, 0x1d, 0x1d),
            panel_bg: Color32::from_rgb(0x1d, 0x1d, 0x1d),
            tab_bg: Color32::from_rgb(0x1d, 0x1d, 0x1d),
            tab_text: Color32::from_rgb(0xb0, 0xb0, 0xb0),
            tab_text_active: Color32::from_rgb(0xd1, 0xd1, 0xd1),
            focus: Color32::from_rgb(0x57, 0x94, 0xec),
            accent: Color32::from_rgb(0x2f, 0x6b, 0xdf),
            accent_hover: Color32::from_rgb(0x3f, 0x7c, 0xe8),
            text: Color32::from_rgb(0xd1, 0xd1, 0xd1),
            text_dim: Color32::from_rgb(0xb0, 0xb0, 0xb0),
            text_faint: Color32::from_rgb(0x6e, 0x6e, 0x6e),
            icon: Color32::from_rgb(0xb0, 0xb0, 0xb0),
            icon_active: Color32::from_rgb(0xd1, 0xd1, 0xd1),
            hover: Color32::from_rgb(0x2c, 0x2c, 0x2c),
            pressed: Color32::from_rgb(0x4b, 0x4b, 0x4b),
            field_bg: Color32::from_rgb(0x0e, 0x0e, 0x0e),
            field_border: Color32::from_rgb(0x30, 0x30, 0x30),
            separator: Color32::from_rgb(0x30, 0x30, 0x30),
            row_alt: Color32::from_rgb(0x21, 0x21, 0x21),
            row_selected: Color32::from_rgb(0x33, 0x33, 0x33),
            hot_text: Color32::from_rgb(0x40, 0x96, 0xf3),
            pill_active_bg: Color32::from_rgb(0x3a, 0x3a, 0x3a),
            clip_bar_bg: Color32::from_rgb(58, 58, 70),
            control_surface: Color32::from_rgb(0x24, 0x24, 0x24),
            control_well: Color32::from_rgb(0x10, 0x10, 0x10),
            control_border: Color32::from_rgb(0x50, 0x50, 0x50),
            control_handle_bg: Color32::from_rgb(0x2a, 0x2a, 0x2a),
            control_handle: Color32::from_rgb(0xe0, 0xe0, 0xe0),
            control_handle_dim: Color32::from_rgb(0xb0, 0xb0, 0xb0),
            plot_bg: Color32::from_rgb(0x18, 0x18, 0x18),
            plot_grid: Color32::from_rgb(0x2c, 0x2c, 0x2c),
            plot_axis: Color32::from_rgb(0x44, 0x44, 0x44),
            plot_handle: Color32::from_rgb(0xe0, 0xe0, 0xe0),
            plot_handle_dim: Color32::from_rgb(0x90, 0x90, 0x90),
            meter_bg: Color32::BLACK,
            meter_readout_bg: Color32::from_rgb(0x14, 0x14, 0x14),
            meter_clip_off: Color32::from_rgb(0x30, 0x30, 0x30),
            panner_bg: Color32::from_gray(22),
            panner_border: Color32::from_gray(96),
            panner_speaker: Color32::from_gray(144),
            fader_knob: Color32::from_gray(90),
            eq_track: Color32::from_gray(48),
            eq_tick: Color32::from_gray(96),
            eq_knob: Color32::from_gray(192),
            eq_knob_active: Color32::from_gray(240),
            eq_node: Color32::from_gray(232),
            eq_node_off: Color32::from_gray(112),
            crossover_handle: Color32::from_gray(255),
            keyframe_plot_bg: Color32::from_gray(25),
            keyframe_plot_grid: Color32::from_gray(42),
            keyframe_plot_axis: Color32::from_gray(51),
            keyframe_handle: Color32::from_gray(208),
            switch_fill: Color32::from_gray(212),
            switch_knob: Color32::from_gray(29),
            switch_border: Color32::from_gray(138),
            slider_disabled_track: Color32::from_gray(74),
            slider_disabled_knob: Color32::from_gray(106),
            button_hover: Color32::from_gray(42),
            button_border: Color32::from_gray(75),
            caption_track_bg: Color32::from_rgb(0x23, 0x20, 0x2a),
            scroll_thumb: Color32::from_gray(0x4b),
            scroll_thumb_hover: Color32::from_gray(0x6a),
            vertical_scroll_thumb: Color32::from_gray(80),
            disabled_clip_bg: Color32::from_gray(0x2a),
            tl_bg: Color32::from_rgb(0x1d, 0x1d, 0x1d),
            tl_track_bg: Color32::from_rgb(0x1d, 0x1d, 0x1d),
            tl_track_bg_alt: Color32::from_rgb(0x1d, 0x1d, 0x1d),
            tl_header_bg: Color32::from_rgb(0x1d, 0x1d, 0x1d),
            tl_ruler_bg: Color32::from_rgb(0x1d, 0x1d, 0x1d),
            tl_ruler_tick: Color32::from_rgb(0x8d, 0x8d, 0x8d),
            tl_ruler_text: Color32::from_rgb(0xb0, 0xb0, 0xb0),
            playhead: Color32::from_rgb(0x58, 0x95, 0xec),
            track_target: Color32::from_rgb(0x26, 0x5b, 0xc1),
            in_out_shade: Color32::from_rgb(0x3f, 0x3f, 0x3f),
            clip_selected_border: Color32::from_rgb(0xeb, 0xeb, 0xeb),
            render_red: Color32::from_rgb(0xe3, 0x48, 0x50),
            render_yellow: Color32::from_rgb(0xf0, 0xf0, 0x4f),
            render_green: Color32::from_rgb(0x2d, 0x9d, 0x78),
            monitor_bg: Color32::from_rgb(0x1d, 0x1d, 0x1d),
            timecode: Color32::from_rgb(0x40, 0x96, 0xf3),
            danger: Color32::from_rgb(0xdc, 0x51, 0x3d),
            radius: 0.0,
            radius_sm: 4.0,
            gap: 4.0,
            tab_h: 32.0,
        };

        match kind {
            ThemeKind::Dark => dark,
            ThemeKind::Medium => Tokens {
                app_bg: Color32::from_rgb(20, 20, 20),
                header_bg: Color32::from_rgb(0x32, 0x32, 0x32),
                panel_bg: Color32::from_rgb(0x32, 0x32, 0x32),
                tab_bg: Color32::from_rgb(0x32, 0x32, 0x32),
                field_bg: Color32::from_rgb(0x22, 0x22, 0x22),
                tl_bg: Color32::from_rgb(0x32, 0x32, 0x32),
                tl_track_bg: Color32::from_rgb(0x32, 0x32, 0x32),
                tl_track_bg_alt: Color32::from_rgb(0x32, 0x32, 0x32),
                tl_header_bg: Color32::from_rgb(0x32, 0x32, 0x32),
                tl_ruler_bg: Color32::from_rgb(0x32, 0x32, 0x32),
                monitor_bg: Color32::from_rgb(0x32, 0x32, 0x32),
                row_alt: Color32::from_rgb(50, 50, 50),
                hover: Color32::from_rgb(64, 64, 64),
                ..dark
            },
            ThemeKind::Light => Tokens {
                app_bg: Color32::from_rgb(244, 244, 246),
                header_bg: Color32::from_rgb(250, 250, 252),
                panel_bg: Color32::from_rgb(255, 255, 255),
                tab_bg: Color32::from_rgb(241, 241, 244),
                tab_text: Color32::from_rgb(90, 90, 95),
                tab_text_active: Color32::from_rgb(20, 20, 25),
                focus: Color32::from_rgb(10, 132, 255),
                accent: Color32::from_rgb(10, 132, 255),
                accent_hover: Color32::from_rgb(48, 150, 255),
                text: Color32::from_rgb(28, 28, 30),
                text_dim: Color32::from_rgb(80, 80, 85),
                text_faint: Color32::from_rgb(142, 142, 147),
                icon: Color32::from_rgb(65, 65, 70),
                icon_active: Color32::from_rgb(25, 25, 30),
                hover: Color32::from_rgb(236, 236, 240),
                pressed: Color32::from_rgb(223, 224, 229),
                field_bg: Color32::from_rgb(247, 247, 248),
                field_border: Color32::from_rgb(208, 208, 214),
                separator: Color32::from_rgb(228, 228, 231),
                row_alt: Color32::from_rgb(249, 249, 250),
                row_selected: Color32::from_rgb(233, 242, 255),
                hot_text: Color32::from_rgb(0, 122, 255),
                pill_active_bg: Color32::from_rgb(230, 236, 244),
                clip_bar_bg: Color32::from_rgb(214, 228, 246),
                control_surface: Color32::from_rgb(247, 247, 248),
                control_well: Color32::from_rgb(239, 239, 243),
                control_border: Color32::from_rgb(201, 201, 208),
                control_handle_bg: Color32::from_rgb(218, 218, 224),
                control_handle: Color32::from_rgb(39, 39, 42),
                control_handle_dim: Color32::from_rgb(94, 94, 100),
                plot_bg: Color32::from_rgb(252, 252, 253),
                plot_grid: Color32::from_rgb(232, 232, 234),
                plot_axis: Color32::from_rgb(204, 204, 210),
                plot_handle: Color32::from_rgb(50, 50, 54),
                plot_handle_dim: Color32::from_rgb(95, 95, 101),
                meter_bg: Color32::from_rgb(235, 235, 240),
                meter_readout_bg: Color32::from_rgb(249, 249, 251),
                meter_clip_off: Color32::from_rgb(188, 188, 194),
                panner_bg: Color32::from_gray(240),
                panner_border: Color32::from_gray(181),
                panner_speaker: Color32::from_gray(112),
                fader_knob: Color32::from_gray(122),
                eq_track: Color32::from_gray(219),
                eq_tick: Color32::from_gray(190),
                eq_knob: Color32::from_gray(82),
                eq_knob_active: Color32::from_gray(25),
                eq_node: Color32::from_gray(36),
                eq_node_off: Color32::from_gray(148),
                crossover_handle: Color32::from_gray(35),
                keyframe_plot_bg: Color32::from_gray(248),
                keyframe_plot_grid: Color32::from_gray(223),
                keyframe_plot_axis: Color32::from_gray(195),
                keyframe_handle: Color32::from_gray(34),
                switch_fill: Color32::from_gray(220),
                switch_knob: Color32::from_gray(255),
                switch_border: Color32::from_gray(170),
                slider_disabled_track: Color32::from_gray(214),
                slider_disabled_knob: Color32::from_gray(177),
                button_hover: Color32::from_gray(234),
                button_border: Color32::from_gray(196),
                caption_track_bg: Color32::from_rgb(239, 234, 246),
                scroll_thumb: Color32::from_gray(197),
                scroll_thumb_hover: Color32::from_gray(170),
                vertical_scroll_thumb: Color32::from_gray(182),
                disabled_clip_bg: Color32::from_gray(218),
                in_out_shade: Color32::from_gray(216),
                tl_bg: Color32::from_rgb(248, 248, 250),
                tl_track_bg: Color32::from_rgb(255, 255, 255),
                tl_track_bg_alt: Color32::from_rgb(249, 249, 251),
                tl_header_bg: Color32::from_rgb(245, 245, 247),
                tl_ruler_bg: Color32::from_rgb(247, 247, 249),
                tl_ruler_text: Color32::from_rgb(90, 90, 95),
                monitor_bg: Color32::from_rgb(255, 255, 255),
                danger: Color32::from_rgb(255, 69, 58),
                radius: 10.0,
                radius_sm: 8.0,
                gap: 6.0,
                tab_h: 28.0,
                ..dark
            },
        }
    }

    pub fn with_appearance(mut self, highlight: Option<[u8; 3]>, accessible_contrast: bool) -> Self {
        if let Some([r, g, b]) = highlight {
            let c = Color32::from_rgb(r, g, b);
            if c != self.accent {
                let tint = |s: f32, v: f32| {
                    let mut h = egui::ecolor::HsvaGamma::from(c);
                    h.s = (h.s * s).clamp(0.0, 1.0);
                    h.v = (h.v * v).clamp(0.0, 1.0);
                    Color32::from(h)
                };
                self.focus = tint(0.92, 1.06);
                self.playhead = self.focus;
                self.timecode = tint(0.98, 1.09);
                self.hot_text = self.timecode;
                self.track_target = tint(1.0, 0.865);
            }
            let lift = |v: u8| v.saturating_add(16);
            self.accent = c;
            self.accent_hover = Color32::from_rgb(lift(r), lift(g), lift(b));
        }
        if accessible_contrast {
            if self.kind == ThemeKind::Light {
                self.text_dim = Color32::from_rgb(40, 40, 40);
                self.text_faint = Color32::from_rgb(80, 80, 80);
                self.field_border = Color32::from_rgb(110, 110, 110);
            } else {
                self.text_dim = self.text;
                self.text_faint = Color32::from_rgb(0xa8, 0xa8, 0xa8);
                self.tab_text = self.tab_text_active;
                self.field_border = Color32::from_rgb(0x6a, 0x6a, 0x6a);
                self.separator = Color32::from_rgb(0x50, 0x50, 0x50);
            }
        }
        self
    }

    pub fn mono(size: f32) -> FontId {
        FontId::new(size, FontFamily::Monospace)
    }
    pub fn ui(size: f32) -> FontId {
        FontId::new(size, FontFamily::Proportional)
    }
    pub fn semibold(size: f32) -> FontId {
        FontId::new(size, FontFamily::Name("semibold".into()))
    }
    pub fn timecode() -> FontId {
        FontId::new(16.0, FontFamily::Proportional)
    }
}

pub const MENU_TEXT_SIZE: f32 = 13.0;
pub const MENU_TEXT: Color32 = Color32::from_rgb(0xde, 0xde, 0xde);

pub fn menu_style(s: &mut egui::Style) {
    egui::containers::menu::menu_style(s);
    s.text_styles.insert(TextStyle::Button, FontId::new(MENU_TEXT_SIZE, FontFamily::Proportional));
    s.text_styles.insert(TextStyle::Body, FontId::new(MENU_TEXT_SIZE, FontFamily::Proportional));
    s.spacing.item_spacing.y = 0.0;
    if s.visuals.dark_mode {
        s.visuals.override_text_color = Some(MENU_TEXT);
    }
}

pub fn font_families() -> Vec<FontFamily> {
    vec![FontFamily::Proportional, FontFamily::Monospace, FontFamily::Name("semibold".into()), FontFamily::Name("medium".into())]
}

pub fn install(ctx: &egui::Context, t: &Tokens) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert("inter".into(), Arc::new(FontData::from_static(filmcraft_text::fonts::INTER_REGULAR)));
    fonts.font_data.insert("inter-medium".into(), Arc::new(FontData::from_static(filmcraft_text::fonts::INTER_MEDIUM)));
    fonts.font_data.insert("inter-semibold".into(), Arc::new(FontData::from_static(filmcraft_text::fonts::INTER_SEMIBOLD)));
    fonts.font_data.insert("jbmono".into(), Arc::new(FontData::from_static(filmcraft_text::fonts::JETBRAINS_MONO_REGULAR)));
    fonts.families.entry(FontFamily::Proportional).or_default().insert(0, "inter".into());
    fonts.families.entry(FontFamily::Monospace).or_default().insert(0, "jbmono".into());
    fonts.families.insert(FontFamily::Name("semibold".into()), vec!["inter-semibold".into(), "inter".into()]);
    fonts.families.insert(FontFamily::Name("medium".into()), vec!["inter-medium".into(), "inter".into()]);
    add_craft_fonts(&mut fonts);
    crate::cjk::install(&mut fonts);
    crate::rtl::install(&mut fonts);
    ctx.set_fonts(fonts);
    apply_visuals(ctx, t);
}

fn craft_font_name(f: &filmcraft_text::fonts::CraftFont) -> String {
    format!("craft:{} {}", f.family, f.style)
}

fn add_craft_fonts(fonts: &mut FontDefinitions) {
    let jpan: Vec<_> = filmcraft_text::fonts::craft_japanese().collect();
    for f in &jpan {
        fonts.font_data.insert(craft_font_name(f), Arc::new(FontData::from_static(f.bytes)));
    }
    for (family, stack) in fonts.families.iter_mut() {
        let heavy = matches!(family, FontFamily::Name(n) if matches!(n.as_ref(), "semibold" | "medium"));
        let mut order = jpan.clone();
        order.sort_by_key(|f| (!f.family.contains("Gothic"), (f.style == "Bold") != heavy));
        stack.extend(order.iter().map(|f| craft_font_name(f)));
    }
}

pub fn apply_visuals(ctx: &egui::Context, t: &Tokens) {
    let light = t.kind == ThemeKind::Light;
    ctx.set_theme(if light { egui::Theme::Light } else { egui::Theme::Dark });
    let mut v = if light { Visuals::light() } else { Visuals::dark() };
    v.panel_fill = t.panel_bg;
    v.window_fill = t.panel_bg;
    v.extreme_bg_color = t.field_bg;
    v.faint_bg_color = t.row_alt;
    v.override_text_color = Some(t.text);
    v.selection.bg_fill = t.accent;
    v.selection.stroke = Stroke::new(1.0, Color32::WHITE);
    v.hyperlink_color = t.accent;
    v.window_stroke = Stroke::new(1.0, t.field_border);
    v.window_corner_radius = egui::CornerRadius::same(t.radius_sm as u8);
    v.menu_corner_radius = egui::CornerRadius::same(t.radius_sm as u8);
    v.popup_shadow = egui::epaint::Shadow { offset: [0, 3], blur: 12, spread: 0, color: Color32::from_black_alpha(110) };
    v.window_shadow = v.popup_shadow;
    for w in [&mut v.widgets.noninteractive, &mut v.widgets.inactive, &mut v.widgets.hovered, &mut v.widgets.active, &mut v.widgets.open] {
        w.corner_radius = egui::CornerRadius::same(t.radius_sm as u8);
    }
    v.widgets.noninteractive.bg_fill = t.panel_bg;
    v.widgets.noninteractive.weak_bg_fill = t.panel_bg;
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, t.separator);
    v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, t.text);
    v.widgets.inactive.bg_fill = t.field_bg;
    v.widgets.inactive.weak_bg_fill = t.field_bg;
    v.widgets.inactive.bg_stroke = Stroke::new(1.0, t.field_border);
    v.widgets.inactive.fg_stroke = Stroke::new(1.0, t.text);
    v.widgets.hovered.bg_fill = t.hover;
    v.widgets.hovered.weak_bg_fill = t.hover;
    v.widgets.hovered.bg_stroke = Stroke::new(1.0, t.field_border);
    v.widgets.hovered.fg_stroke = Stroke::new(1.0, t.tab_text_active);
    v.widgets.active.bg_fill = t.pressed;
    v.widgets.active.weak_bg_fill = t.pressed;
    v.widgets.active.fg_stroke = Stroke::new(1.0, t.tab_text_active);
    v.widgets.open.bg_fill = t.hover;
    v.widgets.open.weak_bg_fill = t.hover;
    ctx.set_visuals(v);
    ctx.global_style_mut(|s| {
        s.spacing.item_spacing = egui::vec2(6.0, 4.0);
        s.spacing.button_padding = egui::vec2(8.0, 6.0);
        s.spacing.interact_size.y = 30.0;
        s.spacing.menu_margin = egui::Margin::same(4);
        s.text_styles.insert(TextStyle::Body, FontId::new(12.0, FontFamily::Proportional));
        s.text_styles.insert(TextStyle::Button, FontId::new(12.0, FontFamily::Proportional));
        s.text_styles.insert(TextStyle::Small, FontId::new(11.0, FontFamily::Proportional));
        s.text_styles.insert(TextStyle::Heading, FontId::new(15.0, FontFamily::Name("semibold".into())));
        s.text_styles.insert(TextStyle::Monospace, FontId::new(12.0, FontFamily::Monospace));
        s.animation_time = ANIMATION_TIME;
    });
}

pub const ANIMATION_TIME: f32 = 0.08;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_theme_has_light_chrome_and_plot_surfaces() {
        let dark = Tokens::for_kind(ThemeKind::Dark);
        let light = Tokens::for_kind(ThemeKind::Light);
        let luminance = |c: Color32| u16::from(c.r()) + u16::from(c.g()) + u16::from(c.b());

        for (name, dark_surface, light_surface) in [
            ("active pill", dark.pill_active_bg, light.pill_active_bg),
            ("clip bar", dark.clip_bar_bg, light.clip_bar_bg),
            ("control surface", dark.control_surface, light.control_surface),
            ("control well", dark.control_well, light.control_well),
            ("plot", dark.plot_bg, light.plot_bg),
            ("keyframe plot", dark.keyframe_plot_bg, light.keyframe_plot_bg),
            ("panner", dark.panner_bg, light.panner_bg),
            ("caption lane", dark.caption_track_bg, light.caption_track_bg),
            ("active toggle", dark.pressed, light.pressed),
            ("meter", dark.meter_bg, light.meter_bg),
            ("meter readout", dark.meter_readout_bg, light.meter_readout_bg),
        ] {
            assert!(luminance(light_surface) > luminance(dark_surface), "{name} did not adapt to Light");
        }
        assert!(luminance(light.control_handle) < luminance(light.control_handle_bg));
        assert!(luminance(light.plot_handle) < luminance(light.plot_bg));
        assert!(luminance(light.icon_active) < luminance(light.pressed));
    }
}
