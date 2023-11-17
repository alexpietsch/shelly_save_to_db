# Save Shelly Data to DB

This is a fairly simple application which retrieves the current usage data from a [Shelly Plug S](https://www.shelly.com/de/products/shop/shelly-plus-plug-s-1) or [Shelly Plug](https://www.shelly.com/de/products/shop/shelly-plug) and saves it to a MySQL DB for later analysis.

## Database setup

I use three Shelly Plug S. I created a Database "shellyData". For each plug I created one table:

```mysql
CREATE TABLE `shellyNAME` (
  `entryID` INT NOT NULL AUTO_INCREMENT,
  `timestamp` DATETIME,
  `powerLevel` INT,
  PRIMARY KEY (`entryID`)
);
```

## Adding more Plugs

The application is built in such a way that it is easy to add/remove Plugs.
To add a new plug, just add a new enviorment variable like this:

```rust
let shelly_<NUMBER> = Vec::from_iter(env::var("SHELLY_<NUMBER>").expect("SHELLY_<NUMBER> is not defined")
        .split("|").map(String::from));
let shelly_<NUMBER>_ip = shelly_<NUMBER>.get(0).expect("Error when getting IP");
let shelly_<NUMBER>_table = shelly_<NUMBER>.get(1).expect("Error when getting table name");
```

Then, inside the loop in the main function, add the following:

```rust
add_to_db(shelly_<NUMBER>_ip, &shelly_<NUMBER>_table, &pool)
    .await.expect(format!("Error when saving {}", &shelly_<NUMBER>_table).as_str());
```

## Build and run
### Build from source

To build from source, run:

```shell
cargo build --release
```

Then, start the application:

```shell
DB_HOST='DB_IP' \
DB_PASSWORD='DB_PASSWORD' \
DB_USER='DB_USERNAME' \
SAVE_INTERVAL='SAVE_INTERVAL_IN_MS' \
SHELLY_ONE='SHELLY_IP|DB_TABLE_NAME' \
SHELLY_THREE='SHELLY_IP|DB_TABLE_NAME' \
SHELLY_TWO='SHELLY_IP|DB_TABLE_NAME' \
./target/release/shelly-save-to-db 
```

### Docker

Build the Docker image with:

```shell
docker build -t shelly-save-to-db:0.1.0 .
```

Then, run it with:

```shell
docker run \
    -e DB_HOST='DB_IP' \
    -e DB_PASSWORD='DB_PASSWORD' \
    -e DB_USER='DB_USERNAME' \
    -e SAVE_INTERVAL='SAVE_INTERVAL_IN_MS' \
    -e SHELLY_ONE='SHELLY_IP|DB_TABLE_NAME' \
    -e SHELLY_THREE='SHELLY_IP|DB_TABLE_NAME' \
    -e SHELLY_TWO='SHELLY_IP|DB_TABLE_NAME' \
shelly-save-to-db:latest
```


