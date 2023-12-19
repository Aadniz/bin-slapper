use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};
use rocket_dyn_templates::handlebars::{Handlebars, Context, RenderContext, Helper, Output, RenderError};

use crate::utils;


fn lowercase(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> Result<(), RenderError> {
    // Get the parameter from the helper, which should be a number
    let param = h.param(0).unwrap().value();
    let value = param.as_str().unwrap();

    // Multiply the number by 1000 and round it
    let result = value.to_lowercase();

    // Write the result to the output
    out.write(&result.to_string())?;
    Ok(())
}

fn hbs_round(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> Result<(), RenderError> {
    // Get the parameter from the helper, which should be a number
    let param = h.param(0).unwrap().value();
    let number = param.as_f64().unwrap();

    // Multiply the number by 1000 and round it
    let result = number.round();

    // Write the result to the output
    out.write(&result.to_string())?;
    Ok(())
}

fn multiply(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> Result<(), RenderError> {
    // Get the parameter from the helper, which should be a number
    let number1 = h.param(0).unwrap().value().as_f64().unwrap();
    let number2 = h.param(1).unwrap().value().as_f64().unwrap();
    let round = h.hash_get("round").and_then(|v| v.value().as_bool());

    // Multiply the number by 1000 and round it
    let result = if round.is_some() && round.unwrap() == true {(number1 * number2).round()} else {number1 * number2};

    // Write the result to the output
    out.write(&result.to_string())?;
    Ok(())
}

fn load_time(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> Result<(), RenderError> {
    // Get the start_load_time from the helper
    let start_load_time = h.param(0).unwrap().value().as_f64().unwrap();

    // Calculate the elapsed time in milliseconds
    let elapsed_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64() - start_load_time;
    let elapsed_ms = (elapsed_time * 1000.0 * 10000.0).round() / 10000.0;

    // Write the elapsed time to the output
    out.write(&elapsed_ms.to_string())?;
    Ok(())
}

fn updated_time(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> Result<(), RenderError> {
    // Get the timestamp from the helper
    let timestamp = h.param(0).unwrap().value().as_str().unwrap();
    let last_checked = DateTime::parse_from_rfc3339(timestamp).unwrap().with_timezone(&Utc);

    // Calculate the elapsed time in minutes
    let now = Utc::now();
    let duration = now.signed_duration_since(last_checked);
    let updated_mins = duration.num_minutes();

    // Format the updated title and description
    let updated_title = if updated_mins <= 0 {
        "Updated now".to_string()
    } else {
        format!("Updated {}m ago", updated_mins)
    };

    // Write the updated title and description to the output
    out.write(&updated_title)?;
    Ok(())
}

fn pretty_date(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> Result<(), RenderError> {
    // Get the timestamp from the helper
    let timestamp = h.param(0).unwrap().value().as_str().unwrap();
    let date = DateTime::parse_from_rfc3339(timestamp).unwrap_or(DateTime::from_timestamp(0,0).unwrap().into()).with_timezone(&Utc);

    // Format the date
    let formatted_date = date.format("%Y-%m-%d %H:%M:%S UTC+0").to_string();

    // Write the formatted date to the output
    out.write(&formatted_date)?;
    Ok(())
}

fn name_to_safe_clickable(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> Result<(), RenderError> {
    // Get the parameter from the helper, which should be a number
    let param = h.param(0).unwrap().value();
    let name = param.as_str().unwrap();

    let result = name.to_lowercase().replace(|c: char| !c.is_ascii_alphanumeric(), "-");

    // Write the result to the output
    out.write(&result)?;
    Ok(())
}

fn colors_between(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> Result<(), RenderError> {
    // Get the parameters from the helper
    let color1 = h.param(0).unwrap().value().as_str().unwrap();
    let color2 = h.param(1).unwrap().value().as_str().unwrap();
    let value = match h.param(2) {
        Some(v) => match v.value().as_str() {
            Some(s) => match DateTime::parse_from_rfc3339(s) {
                Ok(dt) => (Utc::now().timestamp() - dt.timestamp()) as f64,
                Err(_) => v.value().as_f64().unwrap_or(0.5),
            },
            None => v.value().as_f64().unwrap_or(0.5),
        },
        None => 0.5,
    };
    let min_value = h.param(3).map(|v| v.value().as_f64().unwrap()).unwrap_or(0.0);
    let max_value = h.param(4).map(|v| v.value().as_f64().unwrap()).unwrap_or(1.0);
    let steepness = h.param(5).and_then(|v| v.value().as_f64());

    // Parse the colors
    let r1 = u8::from_str_radix(&color1[1..3], 16).unwrap();
    let g1 = u8::from_str_radix(&color1[3..5], 16).unwrap();
    let b1 = u8::from_str_radix(&color1[5..7], 16).unwrap();

    let r2 = u8::from_str_radix(&color2[1..3], 16).unwrap();
    let g2 = u8::from_str_radix(&color2[3..5], 16).unwrap();
    let b2 = u8::from_str_radix(&color2[5..7], 16).unwrap();

    // Calculate the ratio
    let ratio = if value * 1000.0 - min_value * 1000.0 > 0.0 {
        // Too fancy, just commented this out
        // match steepness {
        //     Some(s) => utils::mafs::ease_out((value * 1000.0 - min_value * 1000.0) / (max_value * 1000.0 - min_value * 1000.0), s),
        //     None => (value * 1000.0 - min_value * 1000.0) / (max_value * 1000.0 - min_value * 1000.0)
        // }
        (value * 1000.0 - min_value * 1000.0) / (max_value * 1000.0 - min_value * 1000.0)
    } else { 0.0 };

    // Interpolate the colors
    let r = r1 as f64 * (1.0 - ratio) + r2 as f64 * ratio;
    let g = g1 as f64 * (1.0 - ratio) + g2 as f64 * ratio;
    let b = b1 as f64 * (1.0 - ratio) + b2 as f64 * ratio;

    // Write the color to the output in hexadecimal format
    out.write(&format!("#{:02X}{:02X}{:02X}", r.round() as u8, g.round() as u8, b.round() as u8))?;

    Ok(())
}

fn if_contains(h: &Helper, _: &Handlebars, _: &Context, _: &mut RenderContext, out: &mut dyn Output) -> Result<(), RenderError> {
    // Get the parameters from the helper
    let value1 = h.param(0).unwrap().value().as_str().unwrap();
    let value2 = h.param(1).unwrap().value().as_str().unwrap();

    if value1.contains(value2) {
        out.write(&"1")?;
    }

    Ok(())
}

// Then you can register your helper like this:
pub fn register_helpers(registry: &mut Handlebars) {
    registry.register_helper("multiply", Box::new(multiply));
    registry.register_helper("round", Box::new(hbs_round));
    registry.register_helper("colors_between", Box::new(colors_between));
    registry.register_helper("name_to_safe_clickable", Box::new(name_to_safe_clickable));
    registry.register_helper("load_time", Box::new(load_time));
    registry.register_helper("updated_time", Box::new(updated_time));
    registry.register_helper("pretty_date", Box::new(pretty_date));
    registry.register_helper("contains", Box::new(if_contains));
    registry.register_helper("lowercase", Box::new(lowercase));
}
