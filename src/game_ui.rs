use macroquad::{
    miniquad::window::set_mouse_cursor,
    text::{Font, load_ttf_font_from_bytes},
};

use crate::{
    app::{AppState, UiContext},
    difficulty::Difficulty,
    grid_size::GridSize,
};

mod buttons;
mod layout;

pub use buttons::ButtonId;

pub use layout::compute_status_panel_height;
use layout::{GameOverLayout, MainMenuLayout, PlayingLayout, ScreenLayout, SettingsLayout};

#[derive(Copy, Clone)]
struct Fonts<'a> {
    title: &'a Font,
    body: &'a Font,
}

pub struct GameUi {
    title_font: Font,
    body_font: Font,
    screen: ScreenLayout,
}

impl GameUi {
    pub fn new() -> Self {
        let title_font =
            load_ttf_font_from_bytes(include_bytes!("../assets/Creepster-Regular.ttf")).unwrap();
        let body_font =
            load_ttf_font_from_bytes(include_bytes!("../assets/Jersey15-Regular.ttf")).unwrap();

        Self {
            title_font,
            body_font,
            screen: ScreenLayout::default(),
        }
    }

    pub fn title_font(&self) -> &Font {
        &self.title_font
    }

    pub fn body_font(&self) -> &Font {
        &self.body_font
    }

    pub fn status_panel_height(&self) -> f32 {
        self.screen.status_panel_height()
    }

    pub fn render(&self, ctx: UiContext) {
        set_mouse_cursor(macroquad::miniquad::CursorIcon::Default);

        let fonts = Fonts {
            title: &self.title_font,
            body: &self.body_font,
        };
        self.screen.render(fonts, ctx.blocks_remaining, ctx.score);

        for button in self.screen.buttons() {
            button.render(fonts);
        }
    }

    pub fn handle_input(&self) -> Option<ButtonId> {
        self.screen
            .buttons()
            .iter()
            .find(|button| button.is_pressed())
            .map(|button| button.id.clone())
    }

    pub fn update_buttons(
        &mut self,
        app_state: AppState,
        is_existing_game: bool,
        grid_size: GridSize,
        difficulty: Difficulty,
    ) {
        self.screen = match app_state {
            AppState::Playing => {
                ScreenLayout::Playing(PlayingLayout::compute(&self.title_font, &self.body_font))
            }
            AppState::GameOver => ScreenLayout::GameOver(GameOverLayout::compute(&self.title_font)),
            AppState::MainMenu => {
                ScreenLayout::MainMenu(MainMenuLayout::compute(&self.title_font, is_existing_game))
            }
            AppState::Settings => ScreenLayout::Settings(SettingsLayout::compute(
                &self.title_font,
                &self.body_font,
                grid_size,
                difficulty,
            )),
        };
    }
}
