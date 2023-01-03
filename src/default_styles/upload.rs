use rusty_css::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[derive(Reflect, PartialEq)]
pub struct UploadStyle {
    position: String,
    width: String,
    box_sizing: String,
    padding: String,
    border_radius: String,
    border: String,
    box_shadow: String,
    height: String,
}

impl Style for UploadStyle {
    fn create() -> Self {
        append_to_string!( 
            Self {
                position: "relative",
                box_sizing: "border-box",
                padding: "0",
                border_radius: "10px",
                border: "1px solid #3f3f3f3f",
                width: "100%",
                height: "100%",
                box_shadow: "0px 0px 5px #3f3f3f3f"
            }
        )
    }
}
