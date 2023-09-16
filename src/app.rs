#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
enum SelectedLabel {
    First,
    Second,
    Third,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    selected_label: SelectedLabel,
    #[serde(skip)]
    wallpaper: egui_extras::RetainedImage,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            selected_label: SelectedLabel::First,
            wallpaper: egui_extras::RetainedImage::from_image_bytes(
                "../assets/wallpaper.png",
                include_bytes!("../assets/wallpaper.png"),
            )
            .unwrap(),
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            selected_label: _,
            wallpaper: _,
        } = self;

        egui::Area::new("wallpaper")
            .order(egui::Order::Background)
            .show(ctx, |ui| {
                /*let wallpaper = egui_extras::RetainedImage::from_image_bytes(
                    "../assets/wallpaper.png",
                    include_bytes!("../assets/wallpaper.png"),
                );*/
                ui.add(egui::Image::new(
                    self.wallpaper.texture_id(ctx),
                    ctx.screen_rect().size(),
                ));
            });

        egui::Window::new("Select Bar")
            .anchor(egui::Align2::LEFT_BOTTOM, [10.0, -10.0])
            .fixed_size([250.0, ctx.screen_rect().height() / 1.3])
            .movable(false)
            .resizable(false)
            .title_bar(false)
            .open(&mut true)
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    if ui
                        .add_sized(
                            [ui.available_width(), ui.available_height() / 8.0],
                            egui::SelectableLabel::new(
                                self.selected_label == SelectedLabel::First,
                                "About Me!",
                            ),
                        )
                        .clicked()
                    {
                        self.selected_label = SelectedLabel::First;
                    }
                    if ui
                        .add_sized(
                            [ui.available_width(), ui.available_height() / 8.0],
                            egui::SelectableLabel::new(
                                self.selected_label == SelectedLabel::Second,
                                "Skills!",
                            ),
                        )
                        .clicked()
                    {
                        self.selected_label = SelectedLabel::Second;
                    }
                });
                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                    ui.label("Powered by egui and NovaAI with ♥");
                    ui.separator();
                });
            });

        egui::Window::new("Window")
            .anchor(egui::Align2::RIGHT_BOTTOM, [-10.0, -10.0])
            .fixed_size([
                ctx.screen_rect().width() - 310.0,
                ctx.screen_rect().height() / 1.3,
            ])
            .movable(false)
            .resizable(false)
            .title_bar(false)
            .open(&mut true)
            .show(ctx, |ui| {
                if self.selected_label == SelectedLabel::First {
                    ui.add_sized(ui.available_size(), egui::Label::new("test"));
                } else if self.selected_label == SelectedLabel::Second {
                    ui.add_sized(ui.available_size(), egui::Label::new("test 2"));
                }
            });
    }
}
