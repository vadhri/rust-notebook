use std::process;
use dotenv::dotenv;
use std::env;
use rand::Rng;

extern crate querystring;
extern crate paho_mqtt as mqtt;

fn main() {
	dotenv().ok();
	let mut rng = rand::thread_rng();

	let mqtt_server = env::var("MQTT_SERVER").unwrap();
	let mqtt_publish = env::var("MQTT_PUBLISH").unwrap();

    let cli = mqtt::Client::new(mqtt_server).unwrap_or_else(|err| {
        println!("Error creating the client: {:?}", err);
        process::exit(1);
    });

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(std::time::Duration::from_secs(20))
        .clean_session(true)
        .finalize();

    if let Err(e) = cli.connect(conn_opts) {
        println!("Unable to connect:\n\t{:?}", e);
        process::exit(1);
    }

	let temperature: u8 = rng.gen();
	let temperature_string = temperature.to_string();
	let mut sensor_values_vector = vec![
		("field1", temperature_string.as_str())
	];

	let color: u8 = rng.gen();
	let color_string = color.to_string();

	sensor_values_vector.push(
		("field2", color_string.as_str())
	);

	let humidity: u8 = rng.gen();
	let humidity_string = humidity.to_string();
	sensor_values_vector.push(
		("field3", humidity_string.as_str())
	);

	let pressure: u8 = rng.gen();
	let pressure_string = pressure.to_string();

	sensor_values_vector.push(
		("field4", pressure_string.as_str())
	);

	sensor_values_vector.push(
		("status", "MQTTPUBLISH")
	);

	let qs: String = querystring::stringify(sensor_values_vector);

	let msg = mqtt::Message::new(mqtt_publish, qs, 0);
    let tok = cli.publish(msg);

    if let Err(e) = tok {
        println!("Error sending message: {:?}", e);
    }

    // Disconnect from the broker
    let _tok = cli.disconnect(None);
}
