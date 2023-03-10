use rusty_css::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[derive(Reflect, PartialEq)]
pub struct InputStyle {
    width: String,
    box_sizing: String,
    padding: String,
    border_radius: String,
    border: String,
    margin_bottom: String,
    box_shadow: String,
}

impl Style for InputStyle {
    fn create() -> Self {
        append_to_string!( 
            Self {
                box_sizing: "border-box",
                padding: "0",
                border_radius: "10px",
                border: "1px solid #3f3f3f3f",
                width: "100%",
                margin_bottom: "5px",
                box_shadow: "0px 0px 5px #3f3f3f3f"
            }
        )
    }
}
