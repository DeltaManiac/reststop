use eframe::{
    egui::{widgets, CentralPanel, Color32, ComboBox, CtxRef, Key},
    epi,
};
use reqwest;
use reqwest::Url;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum HTTPMethod {
    PUT,
    GET,
    POST,
    DELETE,
}
impl Default for HTTPMethod {
    fn default() -> HTTPMethod {
        HTTPMethod::GET
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Request {
    url: String,
    parsed_url: Option<Url>,
    color: Color32,
    http_method: HTTPMethod,
}

impl Default for Request {
    fn default() -> Self {
        Request {
            url: String::from(""),
            parsed_url: None,
            color: Color32::GREEN,
            http_method: HTTPMethod::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct UI {
    response: String,
    headers: Option<reqwest::header::HeaderMap>,
    request: Request,
}

impl epi::App for UI {
    fn name(&self) -> &str {
        "RestStop"
    }

    fn update(&mut self, ctx: &CtxRef, _frame: &mut epi::Frame<'_>) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rest Stop");
            ui.separator();
            ui.horizontal(|ui| {
                ComboBox::from_label("")
                    .selected_text(format!("{:?}", &mut self.request.http_method))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.request.http_method, HTTPMethod::GET, "GET");
                        ui.selectable_value(&mut self.request.http_method, HTTPMethod::PUT, "PUT");
                        ui.selectable_value(
                            &mut self.request.http_method,
                            HTTPMethod::POST,
                            "POST",
                        );
                        ui.selectable_value(
                            &mut self.request.http_method,
                            HTTPMethod::DELETE,
                            "DELETE",
                        );
                    });
                // ui.text_edit_singleline(&mut self.url.clone()).text_color(egui::Color32::from_rgb(256,0,0));
                let textedit_response = ui.add(
                    widgets::TextEdit::singleline(&mut self.request.url)
                        .text_color(self.request.color),
                );
                if textedit_response.lost_focus() {
                    match Url::parse(&self.request.url) {
                        Ok(url) => {
                            dbg!("Parsing Url Pass");
                            self.request.parsed_url = Some(url);
                            self.request.color = Color32::GREEN;
                        }
                        Err(e) => {
                            dbg!("Parsing Url Fail");
                            dbg!(e);
                            self.request.parsed_url = None;
                            self.request.color = Color32::RED;
                        }
                    }
                }
                ui.button("Go");
            });
            ui.separator();
        });
    }
}

fn main() {
    let app = UI::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
