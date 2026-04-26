use macroquad::{
    input::{
        MouseButton, is_mouse_button_down, is_mouse_button_pressed, is_mouse_button_released,
        mouse_position,
    },
    math::{Vec2, vec2},
    miniquad::{
        CursorIcon,
        window::{apple_view, set_mouse_cursor},
    },
    shapes::draw_rectangle,
    text::{Font, TextParams, draw_text_ex, measure_text},
    window::{screen_height, screen_width},
};

use crate::{
    constants::{
        style::BACKGROUND_COLOR,
        ui::{
            CHROME_BUTTON_INACTIVE_COLOR, CHROME_CLOSE_COLOR, CHROME_HEIGHT, CHROME_MINIMIZE_COLOR,
            CHROME_TITLE_TEXT_SIZE, CHROME_TRAFFIC_LIGHT_PADDING, CHROME_TRAFFIC_LIGHT_SIZE,
            CHROME_TRAFFIC_LIGHT_SPACING, CHROME_ZOOM_COLOR, CONTAINER_INNER_PADDING,
            LABEL_TEXT_COLOR, WINDOW_PADDING,
        },
    },
    drawing::draw_circle_pixelated,
};

#[cfg(target_os = "macos")]
use objc2::{ClassType, msg_send, runtime::AnyObject};
#[cfg(target_os = "macos")]
use objc2_app_kit::NSEvent;
#[cfg(target_os = "macos")]
use objc2_foundation::{NSPoint, NSRect, NSSize};

const RESIZE_ZONE: f32 = 8.0;
const MIN_WINDOW_WIDTH: f32 = 300.0;
const TRAFFIC_LIGHT_HIT_RADIUS: f32 = CHROME_TRAFFIC_LIGHT_SIZE / 2.0 + 3.0;

#[derive(Clone, Copy, PartialEq)]
enum ResizeEdge {
    Right,
    Bottom,
    BottomRight,
}

struct ResizeState {
    edge: ResizeEdge,
    start_mouse: Vec2,
    start_size: Vec2,
}

struct DragState {
    start_screen_x: f64,
    start_screen_y: f64,
    start_window_x: f64,
    start_window_y: f64,
}

#[cfg(target_os = "macos")]
unsafe fn get_ns_window() -> *mut AnyObject {
    let view = apple_view() as *mut AnyObject;
    msg_send![view, window]
}

pub struct WindowChrome {
    initialized: bool,
    resize_state: Option<ResizeState>,
    drag_state: Option<DragState>,
    grid_rows: u32,
    grid_cols: u32,
    panel_h: f32,
    close_hovered: bool,
    minimize_hovered: bool,
    zoom_hovered: bool,
    #[cfg(target_os = "macos")]
    saved_origin: Option<NSPoint>,
    #[cfg(target_os = "macos")]
    saved_content_size: Option<NSSize>,
}

impl WindowChrome {
    pub fn new(grid_rows: u32, grid_cols: u32, panel_h: f32) -> Self {
        Self {
            initialized: false,
            resize_state: None,
            drag_state: None,
            grid_rows,
            grid_cols,
            panel_h,
            close_hovered: false,
            minimize_hovered: false,
            zoom_hovered: false,
            #[cfg(target_os = "macos")]
            saved_origin: None,
            #[cfg(target_os = "macos")]
            saved_content_size: None,
        }
    }

    /// Given a window width, returns the height that makes the grid fit exactly.
    fn width_to_height(&self, w: f32) -> f32 {
        let avail_w = (w - WINDOW_PADDING.x * 2.0 - CONTAINER_INNER_PADDING * 2.0).max(0.0);
        let block = avail_w / self.grid_cols as f32;
        CHROME_HEIGHT
            + WINDOW_PADDING.y * 2.0
            + CONTAINER_INNER_PADDING * 2.0
            + self.grid_rows as f32 * block
            + self.panel_h
    }

    /// Given a window height, returns the width that makes the grid fit exactly.
    fn height_to_width(&self, h: f32) -> f32 {
        let avail_h = (h
            - CHROME_HEIGHT
            - WINDOW_PADDING.y * 2.0
            - CONTAINER_INNER_PADDING * 2.0
            - self.panel_h)
            .max(0.0);
        let block = avail_h / self.grid_rows as f32;
        WINDOW_PADDING.x * 2.0 + CONTAINER_INNER_PADDING * 2.0 + self.grid_cols as f32 * block
    }

    fn traffic_light_centers() -> [(f32, f32); 3] {
        let cy = CHROME_HEIGHT / 2.0;
        [
            (CHROME_TRAFFIC_LIGHT_PADDING, cy),
            (
                CHROME_TRAFFIC_LIGHT_PADDING + CHROME_TRAFFIC_LIGHT_SPACING,
                cy,
            ),
            (
                CHROME_TRAFFIC_LIGHT_PADDING + 2.0 * CHROME_TRAFFIC_LIGHT_SPACING,
                cy,
            ),
        ]
    }

    fn point_in_circle(mx: f32, my: f32, cx: f32, cy: f32) -> bool {
        let dx = mx - cx;
        let dy = my - cy;
        dx * dx + dy * dy < TRAFFIC_LIGHT_HIT_RADIUS * TRAFFIC_LIGHT_HIT_RADIUS
    }

    fn get_resize_edge(sw: f32, sh: f32, mx: f32, my: f32) -> Option<ResizeEdge> {
        if my < CHROME_HEIGHT {
            return None;
        }
        let near_right = mx > sw - RESIZE_ZONE;
        let near_bottom = my > sh - RESIZE_ZONE;
        match (near_right, near_bottom) {
            (true, true) => Some(ResizeEdge::BottomRight),
            (true, false) => Some(ResizeEdge::Right),
            (false, true) => Some(ResizeEdge::Bottom),
            (false, false) => None,
        }
    }

    pub fn handle_input(&mut self, constrain_resize: bool) {
        if !self.initialized {
            self.setup_platform_window();
            self.initialized = true;
        }

        let (mx, my) = mouse_position();
        let sw = screen_width();
        let sh = screen_height();

        let centers = Self::traffic_light_centers();
        self.close_hovered = Self::point_in_circle(mx, my, centers[0].0, centers[0].1);
        self.minimize_hovered = Self::point_in_circle(mx, my, centers[1].0, centers[1].1);
        self.zoom_hovered = Self::point_in_circle(mx, my, centers[2].0, centers[2].1);

        if is_mouse_button_released(MouseButton::Left) {
            self.drag_state = None;
            self.resize_state = None;
        }

        if self.drag_state.is_some() {
            self.update_drag_platform();
        }

        if let Some(ref state) = self.resize_state {
            let dx = mx - state.start_mouse.x;
            let dy = my - state.start_mouse.y;
            let (new_w, new_h) = if constrain_resize {
                let min_h = self.width_to_height(MIN_WINDOW_WIDTH);
                match state.edge {
                    ResizeEdge::Right => {
                        let nw = (state.start_size.x + dx).max(MIN_WINDOW_WIDTH);
                        (nw, self.width_to_height(nw))
                    }
                    ResizeEdge::Bottom => {
                        let nh = (state.start_size.y + dy).max(min_h);
                        (self.height_to_width(nh), nh)
                    }
                    ResizeEdge::BottomRight => {
                        if dx.abs() >= dy.abs() {
                            let nw = (state.start_size.x + dx).max(MIN_WINDOW_WIDTH);
                            (nw, self.width_to_height(nw))
                        } else {
                            let nh = (state.start_size.y + dy).max(min_h);
                            (self.height_to_width(nh), nh)
                        }
                    }
                }
            } else {
                let min_h = self.width_to_height(MIN_WINDOW_WIDTH);
                match state.edge {
                    ResizeEdge::Right => {
                        let nw = (state.start_size.x + dx).max(MIN_WINDOW_WIDTH);
                        (nw, state.start_size.y)
                    }
                    ResizeEdge::Bottom => {
                        let nh = (state.start_size.y + dy).max(min_h);
                        (state.start_size.x, nh)
                    }
                    ResizeEdge::BottomRight => {
                        let nw = (state.start_size.x + dx).max(MIN_WINDOW_WIDTH);
                        let nh = (state.start_size.y + dy).max(min_h);
                        (nw, nh)
                    }
                }
            };
            self.set_content_size_platform(new_w, new_h);
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            if self.close_hovered {
                macroquad::miniquad::window::request_quit();
                return;
            }
            if self.minimize_hovered {
                self.minimize_window_platform();
                return;
            }
            if self.zoom_hovered {
                self.zoom_window_platform();
                return;
            }

            if let Some(edge) = Self::get_resize_edge(sw, sh, mx, my) {
                self.resize_state = Some(ResizeState {
                    edge,
                    start_mouse: vec2(mx, my),
                    start_size: vec2(sw, sh),
                });
            } else if my < CHROME_HEIGHT {
                self.start_drag_platform();
            }
        }
    }

    pub fn render(&self, body_font: &Font) {
        let sw = screen_width();
        let sh = screen_height();
        let (mx, my) = mouse_position();

        // Set resize cursor (overrides the default set by game UI earlier in render)
        let cursor_edge = self.resize_state.as_ref().map(|s| s.edge).or_else(|| {
            if is_mouse_button_down(MouseButton::Left) {
                None
            } else {
                Self::get_resize_edge(sw, sh, mx, my)
            }
        });
        match cursor_edge {
            Some(ResizeEdge::BottomRight) => set_mouse_cursor(CursorIcon::NWSEResize),
            Some(ResizeEdge::Right) => set_mouse_cursor(CursorIcon::EWResize),
            Some(ResizeEdge::Bottom) => set_mouse_cursor(CursorIcon::NSResize),
            None => {}
        }

        // Chrome background strip
        draw_rectangle(0.0, 0.0, sw, CHROME_HEIGHT, BACKGROUND_COLOR);

        // Traffic light dots
        let centers = Self::traffic_light_centers();
        let colors = [CHROME_CLOSE_COLOR, CHROME_MINIMIZE_COLOR, CHROME_ZOOM_COLOR];
        let hovered = [self.close_hovered, self.minimize_hovered, self.zoom_hovered];
        let r = CHROME_TRAFFIC_LIGHT_SIZE / 2.0;
        for i in 0..3 {
            let (cx, cy) = centers[i];
            let color = if hovered[i] {
                colors[i]
            } else {
                CHROME_BUTTON_INACTIVE_COLOR
            };
            draw_circle_pixelated(cx, cy, r, color);
        }

        // Window title
        let title = "Bleak Blocks";
        let dims = measure_text(title, Some(body_font), CHROME_TITLE_TEXT_SIZE, 1.0);
        draw_text_ex(
            title,
            (sw - dims.width) / 2.0,
            (CHROME_HEIGHT + dims.height) / 2.0,
            TextParams {
                font_size: CHROME_TITLE_TEXT_SIZE,
                color: LABEL_TEXT_COLOR,
                font: Some(body_font),
                ..Default::default()
            },
        );
    }

    fn setup_platform_window(&self) {
        #[cfg(target_os = "macos")]
        unsafe {
            let window = get_ns_window();

            // Add NSWindowStyleMaskFullSizeContentView (1 << 15)
            let current_mask: u64 = msg_send![window, styleMask];
            let (): () = msg_send![window, setStyleMask: current_mask | (1u64 << 15)];

            // Transparent title bar
            let (): () = msg_send![window, setTitlebarAppearsTransparent: true];

            // Hide title text (NSWindowTitleHidden = 1)
            let (): () = msg_send![window, setTitleVisibility: 1i64];

            // Hide native traffic light buttons (close=0, miniaturize=1, zoom=2)
            for i in 0u64..=2 {
                let btn: *mut AnyObject = msg_send![window, standardWindowButton: i];
                if !btn.is_null() {
                    let (): () = msg_send![btn, setHidden: true];
                }
            }
        }
    }

    fn start_drag_platform(&mut self) {
        #[cfg(target_os = "macos")]
        unsafe {
            let window = get_ns_window();
            let frame: NSRect = msg_send![window, frame];
            let mouse: NSPoint = msg_send![NSEvent::class(), mouseLocation];
            self.drag_state = Some(DragState {
                start_screen_x: mouse.x,
                start_screen_y: mouse.y,
                start_window_x: frame.origin.x,
                start_window_y: frame.origin.y,
            });
        }
    }

    fn update_drag_platform(&self) {
        #[cfg(target_os = "macos")]
        if let Some(ref state) = self.drag_state {
            unsafe {
                let window = get_ns_window();
                let current: NSPoint = msg_send![NSEvent::class(), mouseLocation];
                let new_origin = NSPoint {
                    x: state.start_window_x + (current.x - state.start_screen_x),
                    y: state.start_window_y + (current.y - state.start_screen_y),
                };
                let (): () = msg_send![window, setFrameOrigin: new_origin];
            }
        }
    }

    fn minimize_window_platform(&self) {
        #[cfg(target_os = "macos")]
        unsafe {
            let window = get_ns_window();
            let null: *mut AnyObject = std::ptr::null_mut();
            let (): () = msg_send![window, miniaturize: null];
        }
    }

    fn zoom_window_platform(&mut self) {
        #[cfg(target_os = "macos")]
        unsafe {
            let window = get_ns_window();
            // Restore if previously maximized
            if let (Some(origin), Some(size)) =
                (self.saved_origin.take(), self.saved_content_size.take())
            {
                let (): () = msg_send![window, setFrameOrigin: origin];
                let (): () = msg_send![window, setContentSize: size];
                return;
            }
            // Save current state then maximize to visible screen area
            let frame: NSRect = msg_send![window, frame];
            let screen: *mut AnyObject = msg_send![window, screen];
            if screen.is_null() {
                return;
            }
            let visible: NSRect = msg_send![screen, visibleFrame];
            self.saved_origin = Some(frame.origin);
            self.saved_content_size = Some(NSSize {
                width: screen_width() as f64,
                height: screen_height() as f64,
            });
            let (): () = msg_send![window, setFrameOrigin: visible.origin];
            let (): () = msg_send![window, setContentSize: visible.size];
        }
    }

    /// Resize the window so the current grid fills the available area exactly.
    /// Fixes one dimension (width in portrait, height in landscape) and solves the other.
    pub fn fit_to_grid(
        &mut self,
        screen_w: f32,
        screen_h: f32,
        rows: u32,
        cols: u32,
        panel_h: f32,
    ) {
        self.grid_rows = rows;
        self.grid_cols = cols;
        self.panel_h = panel_h;

        let is_landscape = screen_w >= screen_h;
        let (new_w, new_h) = if is_landscape {
            let new_w = self.height_to_width(screen_h);
            (new_w, screen_h)
        } else {
            let new_h = self.width_to_height(screen_w);
            (screen_w, new_h)
        };
        self.set_content_size_platform(new_w, new_h);
    }

    fn set_content_size_platform(&self, logical_w: f32, logical_h: f32) {
        #[cfg(target_os = "macos")]
        unsafe {
            let window = get_ns_window();
            let size = NSSize {
                width: logical_w as f64,
                height: logical_h as f64,
            };
            let (): () = msg_send![window, setContentSize: size];
        }
    }
}
