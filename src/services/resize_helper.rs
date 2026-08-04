use crate::services::models::resize_props::ResizeProperties;

pub fn get_resize_properties(value: Option<String>, width: Option<String>, height: Option<String>, original_image_width: u32, original_image_height: u32) -> ResizeProperties {
    let mut resize_props = ResizeProperties {
        width: original_image_width,
        height: original_image_height,
    };

    if value.is_none() && width.is_none() && height.is_none() {
        return resize_props;
    }

    if value.is_some() {
        let mut current_value = value.unwrap().trim().to_string();

        if current_value.ends_with("px") {
            current_value.truncate(current_value.len() - 2);

            let new_value = current_value.parse::<u32>().unwrap_or_else(|_| original_image_width as u32);

            resize_props.width = new_value;
            resize_props.height = new_value;
        } else if current_value.ends_with("%") {
            current_value.truncate(current_value.len() - 1);
            //let percent = current_value.parse::<f32>().unwrap();

            resize_props.width = calculate_value_by_percentage(current_value.clone(), original_image_width);
            resize_props.height = calculate_value_by_percentage(current_value.clone(), original_image_height);
        } else {
            let new_value = current_value.parse::<u32>().unwrap_or_else(|_| original_image_width as u32);

            resize_props.width = new_value;
            resize_props.height = new_value;
        }
    } else if width.is_some() && height.is_some() {
        let mut current_width = width.unwrap().trim().to_string();
        let mut current_height = height.unwrap().trim().to_string();
        let mut new_width = original_image_width;
        let mut new_height = original_image_height;

        // DEFAULT TO PIXELS
        if (!current_width.ends_with("px"))
            && (!current_height.ends_with("px"))
            && (!current_width.ends_with("%"))
            && (!current_height.ends_with("%")) {
                new_width = current_width.parse::<u32>().unwrap_or_else(|_| original_image_width as u32);
                new_height = current_height.parse::<u32>().unwrap_or_else(|_| original_image_height as u32);
        } else {
            // BY PIXELS
            if current_width.ends_with("px") {
                current_width.truncate(current_width.len() - 2);
                new_width = current_width.parse::<u32>().unwrap_or_else(|_| original_image_width as u32);
            }

            if current_height.ends_with("px") {
                current_height.truncate(current_height.len() - 2);
                new_height = current_height.parse::<u32>().unwrap_or_else(|_| original_image_height as u32);
            }

            // BY PERCENT
            if current_width.ends_with("%") {
                current_width.truncate(current_width.len() - 1);
                new_width = calculate_value_by_percentage(current_width.clone(), original_image_width);
            }

            if current_height.ends_with("%") {
                current_height.truncate(current_height.len() - 1);
                new_height = calculate_value_by_percentage(current_height.clone(), original_image_height);
            }
        }

        resize_props.width = new_width;
        resize_props.height = new_height;
    }

    resize_props
}

pub fn calculate_value_by_percentage(percent: String, original_size: u32) -> u32 {
    let percent_val = percent.parse::<f32>().unwrap();
    (original_size as f32 * (percent_val / 100.0)).floor() as u32
}