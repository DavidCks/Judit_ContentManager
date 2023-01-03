use super::input::InputStyle;
use super::selection_box::SelectionBoxStyle;
use super::image::ImageStyle;
use super::upload::UploadStyle;
use rusty_css::Style;

pub fn set_if_empty(input_style: &mut String, selection_box_style: &mut String, image_style: &mut String, upload_style: &mut String) {
    if input_style.is_empty() {
        *input_style = InputStyle::create().inline();
    }
    if selection_box_style.is_empty() {
        *selection_box_style = SelectionBoxStyle::create().inline();
    }
    if image_style.is_empty() {
        *image_style = ImageStyle::create().inline();
    }
    if upload_style.is_empty() {
        *upload_style = UploadStyle::create().inline();
    }
}