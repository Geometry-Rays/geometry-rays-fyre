use crate::types::ObjectStruct;
use macroquad::prelude::Color;

pub fn level_to_string(
    obj_grid: &Vec<ObjectStruct>,
    level_version: &str,

    cc_1001: Color,
    cc_1002: Color,

    current_song: String,
    current_mode: String
) -> String {
    let mut level_string: String = format!(
        "version:{};song:{};mode:{};cc_1001:{},{},{};cc_1002:{},{},{};;;",

        level_version,
        current_song,
        current_mode,

        cc_1001.r,
        cc_1001.g,
        cc_1001.b,

        cc_1002.r,
        cc_1002.g,
        cc_1002.b
    );

    for object in obj_grid {
        if object.id == 23 {
            level_string.push_str(&format!(
                "x:{};y:{};rot:{};id:{};props:{},{},{},{};;",

                object.x,
                object.y,
                object.rotation,
                object.id,
                object.properties.clone().unwrap()[0],
                object.properties.clone().unwrap()[1],
                object.properties.clone().unwrap()[2],
                object.properties.clone().unwrap()[3]
            ));
        } else {
            level_string.push_str(&format!(
                "x:{};y:{};rot:{};id:{};;",

                object.x,
                object.y,
                object.rotation,
                object.id
            ));
        }
    }

    if level_string.ends_with(";;;") {
        level_string.pop();
    }

    level_string.pop();
    level_string.pop();

    return level_string;
}