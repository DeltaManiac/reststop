use eframe::{egui,epi};
use reqwest;

#[derive(Debug, Clone,Eq,PartialEq )]
pub enum HTTPMethod {
    PUT,
    GET,
    POST,
    DELETE
}
impl Default for HTTPMethod{
    fn default()-> HTTPMethod {
       HTTPMethod::GET
    }
}
#[derive(Debug, Clone, Default)]
pub struct UI {
    url: String,
    response: String,
    headers: Option<reqwest::header::HeaderMap>,
    http_method: HTTPMethod,
}

impl epi::App for UI{
    fn name(&self)-> &str{
       "RestStop"
    }
    fn update(&mut self, ctx:&egui::CtxRef, _frame: &mut epi::Frame<'_>){
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.heading("Rest Stop");
            ui.separator();
            ui.horizontal(|ui| {
                egui::ComboBox::from_label("").selected_text(format!("{:?}",&mut self.http_method)).show_ui(ui,|ui| {
                    ui.selectable_value(&mut self.http_method ,HTTPMethod::GET,"GET");
                    ui.selectable_value(&mut self.http_method ,HTTPMethod::PUT,"PUT");
                    ui.selectable_value(&mut self.http_method ,HTTPMethod::POST,"POST");
                    ui.selectable_value(&mut self.http_method ,HTTPMethod::DELETE,"DELETE");
                });
                ui.text_edit_singleline(&mut self.url.clone());
                ui.button("Go")
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
