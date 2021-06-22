use eframe::{
    egui::{widgets, CentralPanel, Color32, ComboBox, CtxRef, Key},
    epi,
};
use reqwest;
use reqwest::StatusCode;
use reqwest::Url;
use std::io::Read;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum HTTPMethod {
    Put,
    Get,
    Post,
    Delete,
}
impl Default for HTTPMethod {
    fn default() -> HTTPMethod {
        HTTPMethod::Get
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct Response {
    body: Option<String>,
    status: Option<StatusCode>,
    headers: Option<reqwest::header::HeaderMap>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Request {
    url: String,
    parsed_url: Option<Url>,
    color: Color32,
    parse_error: Option<String>,
    http_method: HTTPMethod,
}

impl Default for Request {
    fn default() -> Self {
        Request {
            url: String::from(""),
            parsed_url: None,
            parse_error: None,
            color: Color32::GREEN,
            http_method: HTTPMethod::default(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct UI {
    response: String,
    request: Request,
    req_tx: Option<std::sync::mpsc::Sender<Request>>,
    res_rx: Option<Arc<std::sync::mpsc::Receiver<Response>>>,
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
                        ui.selectable_value(&mut self.request.http_method, HTTPMethod::Get, "GET");
                        ui.selectable_value(&mut self.request.http_method, HTTPMethod::Put, "PUT");
                        ui.selectable_value(
                            &mut self.request.http_method,
                            HTTPMethod::Post,
                            "POST",
                        );
                        ui.selectable_value(
                            &mut self.request.http_method,
                            HTTPMethod::Delete,
                            "DELETE",
                        );
                    });
                // ui.text_edit_singleline(&mut self.url.clone()).text_color(egui::Color32::from_rgb(256,0,0));
                let mut textedit_response = ui.add(
                    widgets::TextEdit::singleline(&mut self.request.url)
                        .text_color(self.request.color)
                        .hint_text("Enter URL"),
                );
                if self.request.parse_error.is_some() {
                    textedit_response =
                        textedit_response.on_hover_text(&self.request.parse_error.clone().unwrap());
                }
                if textedit_response.lost_focus() {
                    match Url::parse(&self.request.url) {
                        Ok(url) => {
                            dbg!("Parsing Url Pass");
                            self.request.parsed_url = Some(url);
                            self.request.color = Color32::GREEN;
                            self.request.parse_error = None;
                        }
                        Err(e) => {
                            self.request.parsed_url = None;
                            self.request.parse_error = Some(e.to_string());
                            self.request.color = Color32::RED;
                        }
                    }
                }
                let a = ui.button("Go");
                if a.clicked() {
                    dbg!(self.req_tx.as_ref().unwrap().send(self.request.clone()));
                }
            });
            ui.separator();
            ui.horizontal(|ui| {});
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    let (req_tx, req_rx) = channel::<Request>();
    let (res_tx, res_rx) = channel::<Response>();
    let mut app = UI::default();
    app.req_tx = Some(req_tx);
    app.res_rx = Some(Arc::new(res_rx));
    thread::spawn(move || loop {
        let c = req_rx.recv().unwrap();
        let mut res = reqwest::blocking::get(c.parsed_url.unwrap()).unwrap();
        let mut body = String::new();
        dbg!(res.read_to_string(&mut body).unwrap());
        let r = Response {
            body: Some(body),
            status: Some(res.status()),
            headers: Some(res.headers().clone()),
        };
        dbg!(&r);
        dbg!(res_tx.send(r).unwrap());
    });
    eframe::run_native(Box::new(app), native_options);
    // let res = child.join();
}
