use druid::widget::prelude::*;
use druid::widget::{Button, Flex, Label, TextBox};
use druid::{AppLauncher, Application, Data, Lens, UnitPoint, WidgetExt, WindowDesc, Menu, WindowId, LocalizedString};
use reqwest;
use std::io::Read;

#[derive(Debug, Data, Lens, Clone)]
pub struct UI {
    url: String,
    response: String,
    #[data(ignore)]
    headers: Option<reqwest::header::HeaderMap>,
}

impl Default for UI {
    fn default() -> Self {
        UI {
            url: String::from("http://httpbin.org/get"),
            response: String::from(""),
            headers: None,
        }
    }
}

fn main() {
    let main_window = WindowDesc::new(build_root_widget()).menu(make_menu).title("RestStop");
    let init = UI::default();
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(init)
        .expect("Phail");
}

fn build_root_widget() -> impl Widget<UI> {
    let label = Label::new(|data: &UI, _env: &Env| {
        if data.url.is_empty() {
            "Hello anybody!?".to_string()
        } else {
            format!("Hello {}!", data.url)
        }
    })
    .with_text_size(32.0);

    // a textbox that modifies `name`.
    let textbox = TextBox::new()
        .with_placeholder("Who are we greeting?")
        .with_text_size(18.0)
        // .fix_width(TEXT_BOX_WIDTH)
        .lens(UI::url);

    // arrange the two widgets vertically, with some padding
    Flex::column()
        .with_child(label)
        // .with_spacer(VERTICAL_WIDGET_SPACING)
        .with_child(textbox)
        .align_vertical(UnitPoint::LEFT)
}


#[allow(unused_assignments, unused_mut)]
fn make_menu<T: Data>(_window: Option<WindowId>, _data: &UI, _env: &Env) -> Menu<T> {
    let mut base = Menu::empty();
    #[cfg(target_os = "macos")]
    {
        base = base.entry(druid::platform_menus::mac::application::default())
    }
    #[cfg(any(target_os = "windows", target_os = "linux"))]
    {
        base = base.entry(druid::platform_menus::win::file::default());
    }
    base
}
