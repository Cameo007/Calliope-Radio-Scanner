use std::time::Duration;
use std::env;
use std::process;
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Action {
	value: String,
}

fn main() {
	let args: Vec<String> = env::args().collect();

	let com = "";
	let mut channel = String::from("");

	if args.len() == 2 {
		//detect
		if args[1] == "detect"{
			let ports = serialport::available_ports().expect("No ports found!");
			if ports.is_empty() {
				println!("No Calliopes found.");
			} else {
				for port in ports {
					let com = port.port_name;
					let mode = get_mode(&com);
					println!("{} ({})", com, mode);
				}
			}
			process::exit(0);
		} else {
			help()
		}
	} else if args.len() == 3 {
		//scanner_restart
		let com = &args[2];
		send(com, "SCANNER:RESTART");
	} else if args.len() == 4 {
		if args[1] == "communicator" {
			//Communicator
			let com = &args[2];
			channel = args[3].to_owned();

			api_activate(&channel);

			send(&com, format!("COMMUNICATOR:{channel}").as_str());
		} else if args[1] == "scanner" {
			//Scanner
			let com = &args[2];
			let frequency = &args[3];

			send(com, format!("SCANNER:{frequency}").as_str());
		} else if args[1] == "tester" {
			//Tester
			let com = &args[2];
			channel = args[3].to_owned();

			send(com, format!("TESTER:{channel}").as_str());
		} else {
			help();
		}
	} else {
		help();
	}

	//CTRL+C interrupt handler
	let used_channel = channel.clone();
	ctrlc::set_handler(move || {
		if !used_channel.is_empty() {
        	api_loose(&used_channel);
		}
		process::exit(0);

    })
    .expect("Error setting Ctrl-C handler");

	let mut serial = serialport::new(com, 115_200)
		.timeout(Duration::from_millis(10))
		.open().expect("Failed to open port");

	println!("Set mode");

	let mut last_timestamp = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_secs();

	loop {
		let mut line = String::from("");
		serial.read_to_string(&mut line).ok();

		if line != "" {
			println!("{}", line);
			let log = line.strip_suffix("\n").unwrap();
			println!("{}", log);
			log_to_server(log);
		}

		//Run pending actions
		if !channel.is_empty() {
			let new_timestamp = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap()
			.as_secs();

			if last_timestamp + 15 < new_timestamp {
				last_timestamp = new_timestamp;
				let action = api_get_next_pending_action(&channel);
				send(com, &action.value);
			}
		}
	}
}

fn get_mode(com: &str) -> String {
	let mode = ask(com, "MODE");
	if mode.starts_with("MODE:") {
		mode.split(":").nth(1).unwrap().to_owned()
	} else {
		//return String::from("")
		mode.split(":").nth(1).unwrap().to_owned()
	}
}

fn send(com: &str, message: &str) {
	let mut serial = serialport::new(com, 115_200)
		.timeout(Duration::from_millis(10))
		.open().expect("Failed to open port");
	serial.write((message.to_owned() + "\n").as_bytes())
		.expect("Write failed!");
}

fn ask(com: &str, question: &str) -> String {
	send(com, question);

	let mut serial = serialport::new(com, 115_200)
		.timeout(Duration::from_millis(10))
		.open().expect("Failed to open port");

	loop {
		let mut line = String::from("");
		serial.read_to_string(&mut line).ok();

		if line != "" {
			break line.strip_suffix("\n").unwrap().to_owned();
		}
	}
}

fn log_to_server(log: &str) {
	if log.starts_with("CHANNEL_FOUND") {
		api_log_channel_found(log.split(":").nth(1).unwrap());
	} else if log.starts_with("LOG") {
		//TRAFFIC (Communicator)
		let channel = log.split(":").nth(1).unwrap().split(";").nth(0).unwrap();
		let content = log.split_once(":").unwrap().1.split_once(";").unwrap().1;
		api_log(channel, content);
	}

}


fn api_activate(channel: &str) {
	let response = minreq::get(get_api_endpoint() + "activate/" + channel)
		.with_timeout(3)
		.send()
		.unwrap();
	if response.status_code == 200 {
		let result: Value = serde_json::from_str(response.as_str().unwrap())
			.expect(&format!("Unable to parse json: {:?}", response));
		if result["error_id"].is_null() {
			println!("{}", result["success"].as_str().unwrap());
		} else {
			println!("{}", result["error"].as_str().unwrap());
		}
	}
}

fn api_loose(channel: &str) {
	let response = minreq::get(get_api_endpoint() + "loose/" + channel)
		.with_timeout(3)
		.send()
		.unwrap();
	if response.status_code == 200 {
		let result: Value = serde_json::from_str(response.as_str().unwrap())
			.expect(&format!("Unable to parse json: {:?}", response));
		if result["error_id"].is_null() {
			println!("{}", result["success"].as_str().unwrap());
		} else {
			println!("{}", result["error"].as_str().unwrap());
		}
	}
}

fn api_log(channel: &str, content: &str) {
	println!("{}", get_api_endpoint() + "log/" + channel + "?value=" + content);
	let response = minreq::get(get_api_endpoint() + "log/" + channel + "?value=" + content)
		.with_timeout(3)
		.send()
		.unwrap();
	if response.status_code == 200 {
		let result: Value = serde_json::from_str(response.as_str().unwrap())
			.expect(&format!("Unable to parse json: {:?}", response));
		if !result["error_id"].is_null() {
			println!("{}", result["error"].as_str().unwrap());
		}
	}
}

fn api_log_channel_found(channel: &str) {
	api_log("-1", channel);
}

fn api_get_next_pending_action(channel: &str) -> Action {
	let response = minreq::get(get_api_endpoint() + "pop/" + channel)
		.with_timeout(3)
		.send()
		.unwrap();
	if response.status_code == 200 {
		let result: Value = serde_json::from_str(response.as_str().unwrap())
			.expect(&format!("Unable to parse json: {:?}", response));
		if result["next_action"] != "idle" {
			let action: Action = serde_json::from_value(result["next_action"].to_owned())
				.expect(&format!("Unable to parse json: {:?}", result));
			return action;
		}
	}
	Action{value: String::from("idle")}
}


fn help() {
	println!("
		Calliope Radio Scanner - Debug and manipulate the radio traffic between Calliope minis!\n
		Usage: crs [COMMAND]\n
		Some Commands:\n
		 detect                                    List all connected Calliope minis\n
		 communicator <COM-Port> <CHANNEL>         Use Calliope to listen on a specific channel
		 scanner <COM-Port> <SCANNER-FREQUENCY>    Use Calliope mini to search for channels
		 tester <COM-Port> <CHANNEL>               Run the test mode\n
		 scanner_restart <COM-Port>                Restart Scanner mode
	");
	process::exit(0);
}

fn get_api_endpoint() -> String {
	env::var("CRS_ENDPOINT")
		.unwrap_or(String::from("http://localhost:8123"))
}