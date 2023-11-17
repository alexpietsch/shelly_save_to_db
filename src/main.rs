use std::env;
use std::time::Duration;
use chrono::{DateTime, Local, NaiveDateTime};
use self::models::*;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool};

mod models;

async fn establish_connection() -> Pool<MySql>{
    let db_host = env::var("DB_HOST").expect("DB_HOST is not defined");
    let db_user = env::var("DB_USER").expect("DB_USER is not defined");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD is not defined");
    let db_url = format!("mysql://{}:{}@{}:3306/shellyData", db_user, db_password, db_host);

    MySqlPoolOptions::new().max_connections(1).connect(&db_url).await.unwrap()
}

async fn add_to_db(shelly_ip: &str, shelly_name: &str, pool: &Pool<MySql>) -> Result<(), Box<dyn std::error::Error>>{
    let shelly_meter_data = reqwest::get(format!("http://{}/meter/0", shelly_ip))
        .await?
        .json::<ShellyMeter>()
        .await?;

    let power_level = shelly_meter_data.power;
    let unix_timestamp = shelly_meter_data.timestamp;

    let datetime = DateTime::<Local>::from_naive_utc_and_offset(
        NaiveDateTime::from_timestamp_opt(
            unix_timestamp as i64, 0).unwrap(), *Local::now().offset()
    ).format("%Y-%m-%d %H:%M:%S");


    print!("Adding to {} values {} and {}... ", shelly_name, power_level, datetime);
    sqlx::query_as::<_, ShellyDataRow>(
        format!("INSERT INTO {} (timestamp, powerLevel) VALUES ('{}', {});",
                shelly_name,
                datetime,
                power_level
        ).as_str()
    ).fetch_all(pool).await?;
    println!("Done");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    let shelly_one = Vec::from_iter(env::var("SHELLY_ONE").expect("SHELLY_ONE is not defined")
        .split("|").map(String::from));
    let shelly_one_ip = shelly_one.get(0).expect("Error when getting IP");
    let shelly_one_table = shelly_one.get(1).expect("Error when getting table name");

    let shelly_two = Vec::from_iter(env::var("SHELLY_TWO").expect("SHELLY_TWO is not defined")
        .split("|").map(String::from));
    let shelly_two_ip = shelly_two.get(0).expect("Error when getting IP");
    let shelly_two_table = shelly_two.get(1).expect("Error when getting table name");

    let shelly_three = Vec::from_iter(env::var("SHELLY_THREE").expect("SHELLY_THREE is not defined")
        .split("|").map(String::from));
    let shelly_three_ip = shelly_three.get(0).expect("Error when getting IP");
    let shelly_three_table = shelly_three.get(1).expect("Error when getting table name");

    let save_interval = env::var("SAVE_INTERVAL").expect("SAVE_INTERVAL is not defined");

    let pool = establish_connection().await;

    loop {
        println!("------------------------");
        add_to_db(&shelly_one_ip, &shelly_one_table, &pool)
            .await.expect(format!("Error when saving {}", shelly_one_table).as_str());

        add_to_db(shelly_two_ip, &shelly_two_table, &pool)
            .await.expect(format!("Error when saving {}", shelly_two_table).as_str());

        add_to_db(shelly_three_ip, &shelly_three_table, &pool)
            .await.expect(format!("Error when saving {}", &shelly_three_table).as_str());

        tokio::time::sleep(Duration::from_millis(save_interval.parse::<u64>().unwrap())).await
    }
}
