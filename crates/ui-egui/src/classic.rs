//! Classic Blue: an Interface theme beside Light and Dark (File › Options › General, View ›
//! Interface Theme, `ui.theme` `classic-blue`), a shaded blue look in the style of Office 2010.
//! Every colour is our own value and every shape is drawn in code; no Microsoft artwork.
//!
//! Everything about it lives in this file, so the rest of the UI only carries one-line hooks:
//! - [`apply`] (from `theme::apply`) installs its egui visuals and switches it on;
//! - [`tokens`] (from `Tokens::get`) is the flat palette every widget draws with while it is on:
//!   Light's tokens with the blue frame, navy text and gold highlights over them;
//! - [`LOOK`] holds what a flat palette can't say: gradients, outlines, the tab and sidebar fills;
//! - the drawing hooks ([`title_bar`], [`tab_strip`], [`tab`], [`ribbon_body`], [`button_bg`],
//!   [`group_separator`], [`status_bar`], [`nav_item`], [`nav_ink`]) do nothing unless it is on.

use egui::{Color32, CornerRadius, Rect, Response, Stroke, Ui, pos2};

use crate::theme::{Appearance, Tokens};

/// A top-to-bottom fill.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Gradient {
    pub top: Color32,
    pub bottom: Color32,
}

impl Gradient {
    pub const fn new(top: Color32, bottom: Color32) -> Self {
        Gradient { top, bottom }
    }

    /// The colour `f` of the way down (0 = top, 1 = bottom).
    pub fn at(self, f: f32) -> Color32 {
        self.top.lerp_to_gamma(self.bottom, f.clamp(0.0, 1.0))
    }
}

/// A button highlight: its fill, a light line just inside the outline, and the outline.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Highlight {
    pub fill: Gradient,
    pub shine: Color32,
    pub border: Color32,
}

/// The shading the flat palette can't carry.
#[derive(Clone, Copy, Debug)]
pub struct Look {
    /// The window frame: title bar, tab strip, status bar.
    pub title: Gradient,
    pub tab_strip: Gradient,
    pub status: Gradient,
    /// The ribbon body, its outline and the light bevel inside it.
    pub ribbon: Gradient,
    pub ribbon_border: Color32,
    pub ribbon_shine: Color32,
    /// Tabs: the selected one (joined to the ribbon, outlined in `ribbon_border`), one under the
    /// pointer, and their labels.
    pub tab_selected: Gradient,
    pub tab_hover: Gradient,
    pub tab_hover_border: Color32,
    pub tab_text: Color32,
    /// The File tab, filled with the app colour, and its label.
    pub file_tab: Gradient,
    pub file_tab_hover: Gradient,
    pub on_file_tab: Color32,
    /// Button highlights: under the pointer, held down, toggled on.
    pub hover: Highlight,
    pub pressed: Highlight,
    pub checked: Highlight,
    /// The light line beside each group divider (the divider itself is `Tokens::border`).
    pub group_sep_light: Color32,
    /// The File page's side bar (`Tokens::nav`): its text and icons, the open page's fill and text.
    pub nav_ink: Color32,
    pub nav_selected: Gradient,
    pub on_nav_selected: Color32,
}

const fn rgb(r: u8, g: u8, b: u8) -> Color32 {
    Color32::from_rgb(r, g, b)
}

/// Translucent white (premultiplied).
const fn white(a: u8) -> Color32 {
    Color32::from_rgba_premultiplied(a, a, a, a)
}

/// The app colour, and its darker shade for text and hovers.
const BLUE: Color32 = rgb(0x2B, 0x57, 0x9A);
const BLUE_INK: Color32 = rgb(0x1F, 0x47, 0x85);
const NAVY: Color32 = rgb(0x1E, 0x39, 0x5B);
const FILE_BLUE: Gradient = Gradient::new(rgb(0x3A, 0x6E, 0xB5), rgb(0x25, 0x5A, 0x9E));

pub const LOOK: Look = Look {
    title: Gradient::new(rgb(0xE6, 0xEE, 0xF8), rgb(0xD9, 0xE4, 0xF3)),
    tab_strip: Gradient::new(rgb(0xD9, 0xE4, 0xF3), rgb(0xD2, 0xDF, 0xF0)),
    status: Gradient::new(rgb(0xDB, 0xE6, 0xF5), rgb(0xC2, 0xD3, 0xEB)),
    ribbon: Gradient::new(rgb(0xFB, 0xFC, 0xFE), rgb(0xDB, 0xE6, 0xF4)),
    ribbon_border: rgb(0x8D, 0xB2, 0xE3),
    ribbon_shine: white(0xC8),
    tab_selected: Gradient::new(Color32::WHITE, rgb(0xFB, 0xFC, 0xFE)),
    tab_hover: Gradient::new(rgb(0xF6, 0xF9, 0xFE), rgb(0xE3, 0xEC, 0xF8)),
    tab_hover_border: rgb(0xB4, 0xC9, 0xE7),
    tab_text: rgb(0x15, 0x42, 0x8B),
    file_tab: FILE_BLUE,
    file_tab_hover: Gradient::new(rgb(0x3F, 0x74, 0xBB), rgb(0x2C, 0x63, 0xAA)),
    on_file_tab: Color32::WHITE,
    hover: Highlight { fill: Gradient::new(rgb(0xFE, 0xF8, 0xDC), rgb(0xFD, 0xE5, 0x9C)), shine: white(0x96), border: rgb(0xEB, 0xC7, 0x63) },
    pressed: Highlight { fill: Gradient::new(rgb(0xFC, 0xE2, 0xA3), rgb(0xF9, 0xC5, 0x65)), shine: white(0x3C), border: rgb(0xC5, 0x9A, 0x3D) },
    checked: Highlight { fill: Gradient::new(rgb(0xFD, 0xEB, 0xB6), rgb(0xFB, 0xD6, 0x7F)), shine: white(0x6E), border: rgb(0xD7, 0xA8, 0x45) },
    group_sep_light: white(0xC8),
    nav_ink: NAVY,
    nav_selected: FILE_BLUE,
    on_nav_selected: Color32::WHITE,
};

/// The flat palette: Light's tokens with the blue frame, navy text and gold highlights. Tokens
/// not named here (and any Light gains later) keep Light's values.
pub fn palette() -> Tokens {
    Tokens {
        title_bar: rgb(0xDE, 0xE8, 0xF5),
        ribbon: rgb(0xEA, 0xF0, 0xF9),
        tab_strip: rgb(0xD6, 0xE2, 0xF2),
        canvas: rgb(0xAB, 0xBC, 0xD3),
        panel: rgb(0xEE, 0xF3, 0xFA),
        border: rgb(0xB8, 0xCB, 0xE5),
        border_strong: rgb(0x8D, 0xA6, 0xC9),
        text: NAVY,
        text_dim: rgb(0x43, 0x58, 0x77),
        text_disabled: rgb(0x9F, 0xAE, 0xC3),
        icon: rgb(0x2D, 0x4B, 0x73),
        hover: rgb(0xFD, 0xEF, 0xB9),
        pressed: rgb(0xFA, 0xD3, 0x83),
        checked: rgb(0xFC, 0xE1, 0x9B),
        accent: BLUE,
        nav: rgb(0xE6, 0xED, 0xF8),
        accent_text: BLUE_INK,
        input_border: rgb(0x65, 0x85, 0xB0),
        group_label: rgb(0x3B, 0x64, 0xA0),
        ruler_margin: rgb(0xC6, 0xD5, 0xEA),
        ruler_tick: rgb(0x43, 0x58, 0x77),
        status_bar: rgb(0xCD, 0xDB, 0xEF),
        selection: Color32::from_rgba_unmultiplied_const(0x2B, 0x57, 0x9A, 0x48),
        ..Tokens::light()
    }
}

/// Where the context remembers that Classic Blue is on (set by [`apply`]).
fn on_id() -> egui::Id {
    egui::Id::new("classic_blue_on")
}

/// Where the tab strip leaves the selected tab's left and right edges for [`ribbon_body`], which
/// opens its outline under it.
fn selected_tab_id() -> egui::Id {
    egui::Id::new("classic_blue_selected_tab")
}

/// Height of a drawn tab, standing on the bottom of the tab strip.
const TAB_H: f32 = 24.0;

pub fn active(ctx: &egui::Context) -> bool {
    ctx.data(|d| d.get_temp::<bool>(on_id())).unwrap_or(false)
}

/// The shading, while Classic Blue is on.
pub fn look(ctx: &egui::Context) -> Option<&'static Look> {
    active(ctx).then_some(&LOOK)
}

/// The palette `Tokens::get` returns while Classic Blue is on.
pub fn tokens(ctx: &egui::Context) -> Option<Tokens> {
    active(ctx).then(palette)
}

/// The end of `theme::apply`: Classic Blue installs its visuals as egui's light ones (its egui
/// preference is Light) and switches the hooks on; any other appearance switches them off.
pub fn apply(ctx: &egui::Context, appearance: Appearance) {
    let on = appearance == Appearance::ClassicBlue;
    if on {
        ctx.set_visuals_of(egui::Theme::Light, crate::theme::visuals(&palette()));
    }
    ctx.data_mut(|d| d.insert_temp(on_id(), on));
}

/// The outline of `rect` with its corners rounded by `radius`, clockwise from the top-left corner.
/// With `open_bottom` the path runs from the bottom-left corner up, over the top and down to the
/// bottom-right corner (a tab's outline).
fn rounded_outline(rect: Rect, radius: CornerRadius, open_bottom: bool) -> Vec<egui::Pos2> {
    use std::f32::consts::{FRAC_PI_2, PI};
    let max_r = rect.width().min(rect.height()) / 2.0;
    let corner = |r: u8| f32::from(r).min(max_r).max(0.0);
    let mut pts = Vec::with_capacity(40);
    let arc = |pts: &mut Vec<egui::Pos2>, c: egui::Pos2, r: f32, from: f32| {
        if r < 0.5 {
            pts.push(c);
            return;
        }
        let steps = (r.ceil() as usize).clamp(2, 8);
        for i in 0..=steps {
            let a = from + FRAC_PI_2 * i as f32 / steps as f32;
            pts.push(pos2(c.x + r * a.cos(), c.y + r * a.sin()));
        }
    };
    let (nw, ne) = (corner(radius.nw), corner(radius.ne));
    if open_bottom {
        pts.push(rect.left_bottom());
    }
    arc(&mut pts, pos2(rect.min.x + nw, rect.min.y + nw), nw, PI);
    arc(&mut pts, pos2(rect.max.x - ne, rect.min.y + ne), ne, PI + FRAC_PI_2);
    if open_bottom {
        pts.push(rect.right_bottom());
        return pts;
    }
    let (se, sw) = (corner(radius.se), corner(radius.sw));
    arc(&mut pts, pos2(rect.max.x - se, rect.max.y - se), se, 0.0);
    arc(&mut pts, pos2(rect.min.x + sw, rect.max.y - sw), sw, FRAC_PI_2);
    pts
}

/// Fill `rect` (corners rounded by `radius`) with `g`, top to bottom.
pub fn paint_gradient(painter: &egui::Painter, rect: Rect, radius: impl Into<CornerRadius>, g: Gradient) {
    let radius = radius.into();
    if g.top == g.bottom || rect.height() < 1.0 {
        painter.rect_filled(rect, radius, g.top);
        return;
    }
    // A fan from the centre: every vertex takes the gradient's colour at its height, so each
    // triangle reproduces the gradient exactly.
    let pts = rounded_outline(rect, radius, false);
    let color = |y: f32| g.at((y - rect.min.y) / rect.height());
    let mut mesh = egui::Mesh::default();
    mesh.colored_vertex(rect.center(), color(rect.center().y));
    for p in &pts {
        mesh.colored_vertex(*p, color(p.y));
    }
    let n = pts.len() as u32;
    for i in 0..n {
        mesh.add_triangle(0, 1 + i, 1 + (i + 1) % n);
    }
    painter.add(egui::Shape::mesh(mesh));
}

/// A button highlight on `rect`: the gradient, the light line inside, then the outline.
pub fn paint_highlight(painter: &egui::Painter, rect: Rect, radius: u8, h: Highlight) {
    paint_gradient(painter, rect, radius, h.fill);
    painter.rect_stroke(rect.shrink(1.0), radius.saturating_sub(1), Stroke::new(1.0, h.shine), egui::StrokeKind::Inside);
    painter.rect_stroke(rect, radius, Stroke::new(1.0, h.border), egui::StrokeKind::Inside);
}

/// A tab on `rect`: `fill`, and with a visible `border` its outline over the top and down both
/// sides, open at the bottom where it meets the ribbon.
fn paint_tab(painter: &egui::Painter, rect: Rect, fill: Gradient, border: Color32) {
    let top = CornerRadius { nw: 3, ne: 3, sw: 0, se: 0 };
    paint_gradient(painter, rect, top, fill);
    if border.a() > 0 {
        // Half a pixel in, so the 1px line lands on whole pixels inside the tab.
        let inner = Rect::from_min_max(rect.min + egui::vec2(0.5, 0.5), pos2(rect.max.x - 0.5, rect.max.y));
        painter.add(egui::Shape::line(rounded_outline(inner, top, true), Stroke::new(1.0, border)));
    }
}

/// Shade a whole panel with `g`: the panel's frame insets its content (`ui.max_rect()`) by `left`
/// and `right`, and `top` is left as it is (the status bar's border line).
fn band(ui: &Ui, left: f32, right: f32, top: f32, g: Gradient) {
    let inner = ui.max_rect();
    let rect = Rect::from_min_max(pos2(inner.min.x - left, inner.min.y + top), pos2(inner.max.x + right, inner.max.y));
    paint_gradient(&ui.ctx().layer_painter(ui.layer_id()), rect, 0, g);
}

/// The title bar's shading (call first in its panel).
pub fn title_bar(ui: &Ui, left: f32, right: f32) {
    if let Some(l) = look(ui.ctx()) {
        band(ui, left, right, 0.0, l.title);
    }
}

/// The status bar's shading under its top border line (call first in its panel).
pub fn status_bar(ui: &Ui, left: f32, right: f32) {
    if let Some(l) = look(ui.ctx()) {
        band(ui, left, right, 1.0, l.status);
    }
}

/// The tab strip's shading (call first in its panel, before [`tab`]).
pub fn tab_strip(ui: &Ui, left: f32, right: f32) {
    if let Some(l) = look(ui.ctx()) {
        band(ui, left, right, 0.0, l.tab_strip);
        ui.data_mut(|d| d.remove::<(f32, f32)>(selected_tab_id()));
    }
}

/// A ribbon tab on `r` (the strip's full height). With Classic Blue: paints the tab standing on the
/// ribbon (the File tab in the app colour, the selected tab joined to the ribbon body) and returns
/// the rect its label centres in and the tokens to finish it with: no flat hover or underline, and
/// a white label on the File tab. Otherwise returns `r` and `t` as they are.
pub fn tab(ui: &Ui, r: Rect, t: Tokens, file: bool, selected: bool, hovered: bool, backstage: bool) -> (Rect, Tokens) {
    let Some(l) = look(ui.ctx()) else {
        return (r, t);
    };
    let shape = Rect::from_min_max(pos2(r.min.x, r.max.y - TAB_H), r.max);
    if file {
        paint_tab(ui.painter(), shape, if hovered || backstage { l.file_tab_hover } else { l.file_tab }, Color32::TRANSPARENT);
    } else if selected {
        paint_tab(ui.painter(), shape, l.tab_selected, l.ribbon_border);
        ui.data_mut(|d| d.insert_temp(selected_tab_id(), (shape.min.x, shape.max.x)));
    } else if hovered {
        paint_tab(ui.painter(), shape, l.tab_hover, l.tab_hover_border);
    }
    let label = if file { l.on_file_tab } else { l.tab_text };
    let accent_text = if file { label } else { t.accent_text };
    (shape, Tokens { hover: Color32::TRANSPARENT, accent: Color32::TRANSPARENT, text: label, accent_text, ..t })
}

/// The ribbon body (call first in the ribbon panel, whose frame insets its content by the
/// margins): the frame colour around a shaded, outlined body, open at the top under the selected
/// tab.
pub fn ribbon_body(ui: &Ui, margin: egui::Margin) {
    let Some(l) = look(ui.ctx()) else {
        return;
    };
    let inner = ui.max_rect();
    let panel = Rect::from_min_max(
        pos2(inner.min.x - f32::from(margin.left), inner.min.y - f32::from(margin.top)),
        pos2(inner.max.x + f32::from(margin.right), inner.max.y + f32::from(margin.bottom)),
    );
    let body = Rect::from_min_max(pos2(panel.min.x + 3.0, panel.min.y), pos2(panel.max.x - 3.0, panel.max.y - 1.0));
    let painter = ui.ctx().layer_painter(ui.layer_id());
    painter.rect_filled(panel, 0.0, l.tab_strip.bottom);
    let radius = CornerRadius { nw: 2, ne: 2, sw: 3, se: 3 };
    paint_gradient(&painter, body, radius, l.ribbon);
    painter.rect_stroke(body.shrink(1.0), 2, Stroke::new(1.0, l.ribbon_shine), egui::StrokeKind::Inside);
    painter.rect_stroke(body, radius, Stroke::new(1.0, l.ribbon_border), egui::StrokeKind::Inside);
    if let Some((x0, x1)) = ui.data(|d| d.get_temp::<(f32, f32)>(selected_tab_id())) {
        painter.rect_filled(Rect::from_min_max(pos2(x0 + 1.0, body.min.y), pos2(x1 - 1.0, body.min.y + 2.0)), 0.0, l.ribbon.top);
    }
}

/// A ribbon button's background: with Classic Blue, paints the gold highlight for its state and
/// returns true (the caller draws nothing more); otherwise false.
pub fn button_bg(ui: &Ui, r: Rect, resp: &Response, checked: bool) -> bool {
    let Some(l) = look(ui.ctx()) else {
        return false;
    };
    let h = if resp.is_pointer_button_down_on() {
        l.pressed
    } else if checked {
        l.checked
    } else if resp.hovered() {
        l.hover
    } else {
        return true;
    };
    paint_highlight(ui.painter(), r, 3, h);
    true
}

/// The light line beside a group divider drawn at `x` from `y0` to `y1`.
pub fn group_separator(ui: &Ui, x: f32, y0: f32, y1: f32) {
    if let Some(l) = look(ui.ctx()) {
        ui.painter().line_segment([pos2(x + 1.0, y0), pos2(x + 1.0, y1)], Stroke::new(1.0, l.group_sep_light));
    }
}

/// The colour of text and icons on the File page's side bar: white on the app-coloured bar, or
/// Classic Blue's navy on its light one.
pub fn nav_ink(ctx: &egui::Context) -> Color32 {
    look(ctx).map_or(Color32::WHITE, |l| l.nav_ink)
}

/// A side-bar page row `r`: with Classic Blue, paints the open page's fill or the hover highlight
/// and returns the row's ink; otherwise `None`.
pub fn nav_item(ui: &Ui, r: Rect, open: bool, enabled: bool, hovered: bool) -> Option<Color32> {
    let l = look(ui.ctx())?;
    let row = r.shrink2(egui::vec2(6.0, 1.0));
    if open {
        paint_gradient(ui.painter(), row, 3, l.nav_selected);
        return Some(l.on_nav_selected);
    }
    if enabled && hovered {
        paint_highlight(ui.painter(), row, 3, l.hover);
    }
    Some(if enabled { l.nav_ink } else { l.nav_ink.gamma_multiply(0.45) })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// WCAG contrast ratio between two opaque colours.
    fn contrast(a: Color32, b: Color32) -> f32 {
        let lum = |c: Color32| {
            let ch = |v: u8| {
                let v = f32::from(v) / 255.0;
                if v <= 0.03928 { v / 12.92 } else { ((v + 0.055) / 1.055).powf(2.4) }
            };
            0.2126 * ch(c.r()) + 0.7152 * ch(c.g()) + 0.0722 * ch(c.b())
        };
        let (x, y) = (lum(a), lum(b));
        (x.max(y) + 0.05) / (x.min(y) + 0.05)
    }

    #[test]
    fn classic_blue_is_an_interface_theme() {
        assert_eq!(Appearance::from_code("classic-blue"), Some(Appearance::ClassicBlue));
        assert!(Appearance::ALL.contains(&Appearance::ClassicBlue));
        assert!(!Appearance::ClassicBlue.is_dark(Some(egui::Theme::Dark)), "a light look whatever the OS says");
        assert_eq!(Appearance::ClassicBlue.preference(), egui::ThemePreference::Light);
    }

    /// Off, the hooks leave Light alone; on, every widget gets the Classic Blue palette.
    #[test]
    fn hooks_switch_with_the_theme() {
        let ctx = egui::Context::default();
        crate::theme::apply(&ctx, Appearance::Light);
        assert!(look(&ctx).is_none());
        assert_eq!(Tokens::get(&ctx).ribbon, Tokens::light().ribbon);
        assert_eq!(nav_ink(&ctx), Color32::WHITE);
        crate::theme::apply(&ctx, Appearance::ClassicBlue);
        assert!(look(&ctx).is_some());
        assert_eq!(Tokens::get(&ctx).ribbon, palette().ribbon);
        crate::theme::apply(&ctx, Appearance::Dark);
        assert!(look(&ctx).is_none());
    }

    /// The same bars Light and Dark meet (#655, #657, #1148): text on fills, field outlines, ruler.
    #[test]
    fn the_palette_is_readable() {
        let t = palette();
        for (name, fg, bg, min) in [
            ("on_accent on accent", t.on_accent, t.accent, 4.5),
            ("on_accent on its hover", t.on_accent, t.accent_text, 4.5),
            ("text on ribbon", t.text, LOOK.ribbon.bottom, 4.5),
            ("text_dim on status bar", t.text_dim, LOOK.status.bottom, 4.5),
            ("group labels", t.group_label, LOOK.ribbon.bottom, 4.5),
            ("ruler numbers", t.ruler_tick, t.ruler, 4.5),
            ("side bar text", LOOK.nav_ink, t.nav, 4.5),
            ("open page text (top)", LOOK.on_nav_selected, LOOK.nav_selected.top, 4.5),
            ("open page text (bottom)", LOOK.on_nav_selected, LOOK.nav_selected.bottom, 4.5),
            ("File tab", LOOK.on_file_tab, LOOK.file_tab.top, 4.5),
            ("File tab hovered", LOOK.on_file_tab, LOOK.file_tab_hover.top, 4.5),
            ("tab labels", LOOK.tab_text, LOOK.tab_strip.bottom, 4.5),
            ("tab labels hovered", LOOK.tab_text, LOOK.tab_hover.bottom, 4.5),
        ] {
            let r = contrast(fg, bg);
            assert!(r >= min, "{name} is {r:.2}:1");
        }
        for (name, bg) in [("input", t.input), ("ribbon", t.ribbon), ("panel", t.panel), ("menu", t.menu), ("title bar", t.title_bar)] {
            let r = contrast(t.input_border, bg);
            assert!(r >= 3.0, "input_border on {name} is {r:.2}:1");
        }
        assert!(contrast(t.ruler, Color32::BLACK) > contrast(t.ruler_margin, Color32::BLACK), "ruler text area lighter than its margins");
    }

    #[test]
    fn rounded_outlines_stay_inside_their_rect() {
        let r = Rect::from_min_max(pos2(10.0, 20.0), pos2(70.0, 44.0));
        for open in [false, true] {
            for p in rounded_outline(r, CornerRadius::same(3), open) {
                assert!(r.expand(0.01).contains(p), "{p:?} outside {r:?}");
            }
        }
        // A radius larger than the rect is clamped to half its height.
        assert!(
            rounded_outline(Rect::from_min_size(pos2(0.0, 0.0), egui::vec2(40.0, 4.0)), CornerRadius::same(200), false)
                .iter()
                .all(|p| p.y >= 0.0 && p.y <= 4.0)
        );
    }
}
