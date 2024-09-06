use crate::AccessSharedData;
use actix_web::{http::header::ContentType, web, HttpResponse};
use time::format_description;

use super::relay_control::change_fridge_status;

fn color_status(status: bool) -> String {
    if status {
        "on".to_owned()
    } else {
        "off".to_owned()
    }
}

pub async fn index(common_data: web::Data<AccessSharedData>) -> HttpResponse {
    let format =
        format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();

    let html_res: String = format!(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Atmospheric Control System</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            max-width: 800px;
            margin: 0 auto;
            padding: 20px;
        }}
        h1, h2 {{
            color: #2c3e50;
        }}
        .data-section {{
            background-color: #f9f9f9;
            border: 1px solid #ddd;
            border-radius: 5px;
            padding: 15px;
            margin-bottom: 20px;
        }}
        .data-item {{
            margin-bottom: 10px;
        }}
        .label {{
            font-weight: bold;
        }}
        .value {{
            color: #2980b9;
        }}
        .status {{
            display: inline-block;
            padding: 3px 8px;
            border-radius: 3px;
            text-transform: uppercase;
            font-size: 0.8em;
            font-weight: bold;
        }}
        .on {{
            background-color: #2ecc71;
            color: white;
        }}
        .off {{
            background-color: #e74c3c;
            color: white;
        }}
        footer {{
            margin-top: 40px;
            text-align: center;
            font-size: 0.9em;
            color: #7f8c8d;
        }}
    </style>
</head>
<body>
    <h1>Atmospheric Control System</h1>
    
    <div class="data-section">
        <h2>Current Atmospheric Data</h2>
        <div class="data-item">
            <span class="label">Average Temperature:</span>
            <span class="value">{:.1}°C</span>
        </div>
        <div class="data-item">
            <span class="label">Average Humidity:</span>
            <span class="value">{:.1}%RH</span>
        </div>
        <div class="data-item">
            <span class="label">Last Reading Time:</span>
            <span class="value">{}</span>
        </div>
    </div>

    <div class="data-section">
        <h2>Sensor Readings</h2>
        <div class="data-item">
            <span class="label">Sensor 1:</span>
            <span class="value">{:.1}°C, {:.1}%RH</span>
        </div>
        <div class="data-item">
            <span class="label">Sensor 2:</span>
            <span class="value">{:.1}°C, {:.1}%RH</span>
        </div>
    </div>

    <div class="data-section">
        <h2>Equipment Status</h2>
        <div class="data-item">
            <span class="label">Fridge:</span>
            <span class="status {}">{}</span>
        </div>
        <div class="data-item">
            <span class="label">Last ON:</span>
            <span class="value">{}</span>
        </div>
        <div class="data-item">
            <span class="label">Last OFF:</span>
            <span class="value">{}</span>
        </div>
    </div>

    <div class="data-section">
        <h2>Humidity Control</h2>
        <div class="data-item">
            <span class="label">Humidifier:</span>
            <span class="status {}">{}</span>
        </div>
        <div class="data-item">
            <span class="label">Last ON:</span>
            <span class="value">{}</span>
        </div>
        <div class="data-item">
            <span class="label">Last OFF:</span>
            <span class="value">{}</span>
        </div>
        <div class="data-item">
            <span class="label">Dehumidifier:</span>
            <span class="status {}">{}</span>
        </div>
        <div class="data-item">
            <span class="label">Last ON:</span>
            <span class="value">{}</span>
        </div>
        <div class="data-item">
            <span class="label">Last OFF:</span>
            <span class="value">{}</span>
        </div>
    </div>

    <footer>
        <p>&copy; DIVIGA 2023-Eternity. All rights reserved.</p>
    </footer>
</body>
</html>
        "#,
        common_data.average_temp(),
        common_data.average_humidity(),
        common_data.last_reading_datetime().format(&format).unwrap(),
        common_data.temp_one(),
        common_data.humidity_one(),
        common_data.temp_two(),
        common_data.humidity_two(),
        color_status(common_data.fridge_status()),
        color_status(common_data.fridge_status()),
        common_data
            .fridge_turn_on_datetime()
            .format(&format)
            .unwrap(),
        common_data
            .fridge_turn_off_datetime()
            .format(&format)
            .unwrap(),
        color_status(common_data.humidifier_status()),
        color_status(common_data.humidifier_status()),
        common_data
            .humidifier_turn_on_datetime()
            .format(&format)
            .unwrap(),
        common_data
            .humidifier_turn_off_datetime()
            .format(&format)
            .unwrap(),
        color_status(common_data.dehumidifier_status()),
        color_status(common_data.dehumidifier_status()),
        common_data
            .dehumidifier_turn_on_datetime()
            .format(&format)
            .unwrap(),
        common_data
            .dehumidifier_turn_off_datetime()
            .format(&format)
            .unwrap()
    );

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(html_res)
}

// Commented out form code preserved for potential future use
/*
<form action=\"/change_fridge_status\" method=\"post\">
        <button name=\"fridge_status\" value=\"fridge_status\">Change fridge status</button>
    </form>
    <form action=\"/change_dehumidifier_status\" method=\"post\">
        <button name=\"dehumidifier_status\" value=\"dehumidifier_status\">Change dehumidifier status</button>
    </form>
*/
