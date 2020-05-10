use super::*;

impl<T : InspectRenderDefault<T>> InspectRenderDefault<Vec<T>> for Vec<T> {
    fn render(
        data: &[&Vec<T>],
        label: &'static str,
        ui: &imgui::Ui,
        _args: &InspectArgsDefault,
    ) {
        ui.text(&imgui::im_str!("{}:", label));
        match data.first() {
            None => {}
            Some(data) => {
                ui.indent();
                for v in data.iter() {
                    <T as InspectRenderDefault<T>>::render(
                        &[v],
                        &"",
                        ui,
                        &InspectArgsDefault::default()
                    );
                }
                ui.unindent();
            }
        }
    }

    fn render_mut(
        data: &mut [&mut Vec<T>],
        label: &'static str,
        ui: &imgui::Ui,
        _args: &InspectArgsDefault,
    ) -> bool {

        ui.text(&imgui::im_str!("{}:", label));
        let mut changed = false;
        match data.first_mut() {
            None => {}
            Some(data) => {
                ui.indent();
                for d in data.iter_mut() {
                    <T as InspectRenderDefault<T>>::render_mut(
                        &mut [d],
                        &"",
                        ui,
                        &InspectArgsDefault::default()
                    );
                }
                ui.unindent();
            }
        };
        changed
    }
}
