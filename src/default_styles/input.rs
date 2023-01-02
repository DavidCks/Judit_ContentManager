use rusty_css::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[derive(Reflect)]
pub struct InputStyle {
    pub width: String,
    pub height: String,
}

impl Style for InputStyle {
    fn create() -> Self {
        append_to_string!( 
            Self {
                width: "50px",
                height: "10px",
            }
        )
    }
}
