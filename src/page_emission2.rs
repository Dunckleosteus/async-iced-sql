use crate::app::App;
use crate::messages::Messages;
use iced::{
    widget::{button, column, pick_list, row, text, text_input, PickList, TextInput},
    Renderer,
};
pub fn emmission2_page<'a>(app: &'a App) -> iced::Element<'a, Messages> {
    let mut col = column![];
    if app.connection.is_some() {
        col = col.push(text("connected"));
        col = col.push(
            button("Get Assise Materials from database").on_press(Messages::TryGetAssiseList),
        );
        col = col.push(
            button("Get Roulement Materials from database").on_press(Messages::TryGetRoulementList),
        );
        col = col.push(button("Get traffic list from assise").on_press(Messages::TryGetList));
        match &app.assise_material_list {
            Some(materials) => {
                let drop_down: PickList<'_, String, Messages, Renderer> = pick_list(
                    materials,
                    app.chosen_assise_material.clone(),
                    Messages::SelectAssiseMaterial,
                );
                let thickness_input: TextInput<'_, Messages, Renderer> =
                    text_input("Input Thickness", &app.assise_thickness_input)
                        .on_input(Messages::AssiseThicknessInputChanged)
                        .on_submit(Messages::ChangeAssiseThickness(
                            app.assise_thickness_input.clone(),
                        ));
                col = col.push(
                    row![text(String::from("Assise"))
                        .horizontal_alignment(iced::alignment::Horizontal::Center)]
                    .padding(20.)
                    .width(iced::Length::Fill),
                );
                col = col.push(row![
                    drop_down,
                    thickness_input,
                    text(match app.assise_thickness {
                        Some(val) => format!("{val}"),
                        None => String::from("No value chosen"),
                    })
                ]);
            }
            None => {}
        };
        match &app.roulement_material_list {
            Some(materials) => {
                let thickness_input: TextInput<'_, Messages, Renderer> =
                    text_input("Input Thickness", &app.roulement_thickness_input)
                        .on_input(Messages::RoulementThicknessInputChanged)
                        .on_submit(Messages::ChangeRoulementThickness(
                            app.roulement_thickness_input.clone(),
                        ));
                let drop_down: PickList<'_, String, Messages, Renderer> = pick_list(
                    materials,
                    app.chosen_roulement_material.clone(),
                    Messages::SelectRoulementMaterial,
                );
                col = col.push(row![
                    drop_down,
                    thickness_input,
                    text(match app.roulement_thickness {
                        Some(val) => format!("{val}"),
                        None => String::from("No value chosen"),
                    })
                ]);
            }
            None => {}
        };
        match &app.traffic_list {
            Some(traffic) => {
                let string_values: Vec<String> =
                    traffic.clone().iter().map(|x| x.name.clone()).collect();
                let drop_down: iced::widget::PickList<'_, std::string::String, Messages, Renderer> =
                    pick_list(string_values, app.chosen_traffic.clone(), Messages::Select);
                col = col.push(drop_down);
            }
            None => {}
        };
    } else {
        col = col.push(text("Not connected to database"))
    }
    col.into()
}
