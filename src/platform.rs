//! Platform-specific space reserved at the top of the window/screen: the custom
//! title bar on macOS, or the device safe area (status bar, notch, Dynamic Island)
//! on iOS. Also exposes `ui_scale()`, the factor UI sizes should be multiplied by
//! so they render at a consistent physical size across platforms.

#[cfg(target_os = "ios")]
fn content_scale() -> f32 {
    use macroquad::miniquad::window::apple_view;
    use objc2::{msg_send, runtime::AnyObject};

    unsafe {
        let view = apple_view() as *mut AnyObject;
        if view.is_null() {
            return 1.0;
        }
        let scale: f64 = msg_send![view, contentScaleFactor];
        scale as f32
    }
}

#[cfg(target_os = "macos")]
pub fn top_inset() -> f32 {
    crate::constants::ui::CHROME_HEIGHT
}

#[cfg(target_os = "ios")]
pub fn top_inset() -> f32 {
    use macroquad::miniquad::window::apple_view;
    use objc2::{msg_send, runtime::AnyObject};
    use objc2_ui_kit::UIEdgeInsets;

    unsafe {
        let view = apple_view() as *mut AnyObject;
        if view.is_null() {
            return 0.0;
        }
        let insets: UIEdgeInsets = msg_send![view, safeAreaInsets];
        // safeAreaInsets is in points, but miniquad's iOS backend reports
        // screen_width()/screen_height() (and expects draw coordinates) in the
        // view's backing pixel space, so scale the inset up to match.
        insets.top as f32 * content_scale()
    }
}

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
pub fn top_inset() -> f32 {
    0.0
}

/// Factor to multiply UI sizes (fonts, padding, gaps, corner radii) by.
///
/// On macOS, miniquad reports `screen_width()`/`screen_height()` in the same
/// logical units the UI constants were tuned for, so no scaling is needed. On
/// iOS, those functions report the view's backing *pixel* space rather than
/// points, so a "24px" font constant would render at a fraction of its
/// intended physical size on a high-density display — scaling by the same
/// `contentScaleFactor` used for the safe-area inset corrects that.
#[cfg(target_os = "ios")]
pub fn ui_scale() -> f32 {
    content_scale()
}

#[cfg(not(target_os = "ios"))]
pub fn ui_scale() -> f32 {
    1.0
}

/// Scale a UI size (padding, gap, corner radius, ...) by `ui_scale()`.
pub fn scale(value: f32) -> f32 {
    value * ui_scale()
}

/// Scale a font size by `ui_scale()`.
pub fn scale_font(base: u16) -> u16 {
    ((base as f32) * ui_scale()).round().max(1.0) as u16
}
