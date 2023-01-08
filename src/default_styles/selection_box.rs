use rusty_css::*;
use bevy_reflect::{ Reflect };
use append_to_string::*;

#[derive(Reflect, PartialEq)]
pub struct SelectionBoxStyle {
    width: String,
    height: String,
    overflow: String,
    display: String,
    grid_template_columns: String,
    gap: String,
    align_items: String,
    justify_items: String,
    box_shadow: String,
    padding: String,
    border_radius: String,
    margin_top: String,
}

impl Style for SelectionBoxStyle {
    fn create() -> Self {
        append_to_string!( 
            Self {
                width: "400%",
                height: "50vh",
                overflow: "auto",
                display: "grid",
                grid_template_columns: "1fr 1fr 1fr 1fr", 
                gap: "5% 5%",
                border_radius: "10px",
                justify_items: "center", 
                align_items: "center",
                box_shadow: "0px 0px 15px #3f3f3f3f",
                padding: "5%",
                margin_top: "5px",
            }
        )
    }
}
