use std::sync::Once;

use egui::{Context, FontData, FontDefinitions, FontFamily, FontTweak, RichText};

pub fn ui_main(ctx: &Context, _i: &mut i32) {
    static ONCE: Once = Once::new();

    ONCE.call_once(|| {
        let mut fonts = FontDefinitions::default();
        let tweak = FontTweak::default();
        fonts.font_data.insert(
            "inter".to_owned(),
            FontData::from_static(include_bytes!("../res/Inter-Regular.ttf")).tweak(tweak),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "inter".to_owned());
        ctx.set_fonts(fonts);
        // egui_extras::install_image_loaders(ctx);
    });

    egui::containers::Window::new("Cranberry City").show(ctx, |ui| {
        ui.label(RichText::new("at least it's not Quest3DTamperer™").italics());
    });
}
