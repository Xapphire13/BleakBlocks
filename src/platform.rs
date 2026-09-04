//! Platform-specific space reserved at the top of the window/screen: the custom
//! title bar on macOS, or the device safe area (status bar, notch, Dynamic Island)
//! on iOS.

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
        let scale: f64 = msg_send![view, contentScaleFactor];
        insets.top as f32 * scale as f32
    }
}

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
pub fn top_inset() -> f32 {
    0.0
}
