use crate::shared_data::AccessSharedData;
use serde_json::Value;
use std::process::Command;
use std::thread;
use std::time::Duration;
use time::macros::offset;
use time::OffsetDateTime;

fn get_atmosphere_from_sensor() -> String {
    let output = Command::new("python3")
        .arg("dht.py")
        .output()
        .expect("failed to execute process");

    let str_output: String = String::from_utf8_lossy(&output.stdout).to_string();
    println!("{}", str_output);

    return str_output;
}

pub fn read_atmosphere_from_sensors(sd: &AccessSharedData) {
    const MAX_RETRIES: u8 = 10;
    let mut current_tries: u8 = 0;
    let mut output: String = get_atmosphere_from_sensor();
    let mut v: Value = serde_json::from_str(&output).unwrap();

    loop {
        if v.get("error").is_some() {
            if current_tries < MAX_RETRIES {
                thread::sleep(Duration::from_secs(4));
                output = get_atmosphere_from_sensor();
                v = serde_json::from_str(&output).unwrap();
                current_tries += 1;
            } else {
                println!("MAX_RETRIES reached ! Couldn't get data from sensor ! EXITING.");
                std::process::exit(1);
            }
        } else {
            break;
        }
    }

    let t1: f32 = v["t1"].as_f64().unwrap() as f32;
    let h1: f32 = v["h1"].as_f64().unwrap() as f32;
    let t2: f32 = v["t2"].as_f64().unwrap() as f32;
    let h2: f32 = v["h2"].as_f64().unwrap() as f32;
    let now = OffsetDateTime::now_utc().to_offset(offset!(+1));

    println!("t1:{}, h1:{}, t2:{}, h2:{}", t1, h1, t2, h2);

    sd.increment_polling_iterations();
    sd.set_temp_one(t1);
    sd.set_humidity_one(h1);
    sd.set_temp_two(t2);
    sd.set_humidity_two(h2);
    sd.set_last_reading_datetime(now);
}
