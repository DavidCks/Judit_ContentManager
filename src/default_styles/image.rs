use rusty_css::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[derive(Reflect, PartialEq)]
pub struct ImageStyle {
    max_width: String,
}

impl Style for ImageStyle {
    fn create() -> Self {
        append_to_string!( 
            Self {
                max_width: "100%",
            }
        )
    }
}
