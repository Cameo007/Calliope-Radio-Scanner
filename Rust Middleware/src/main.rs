use std::time::Duration;
use std::env;
use std::process;
use serde_json::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Action {
	cmd: String,
	text: String,
}

fn main() {
	let args: Vec<String> = env::args().collect();

	/*
	for action in get_pending_actions("0") {
		println!("{}, {}", action.cmd, action.text);
	}
	*/

	if args.len() == 2 {
		//detect
		if args[1] == "detect"{
			let ports = serialport::available_ports().expect("No ports found!");
			if ports.is_empty() {
				println!("No Calliopes found.");
			} else {
				for port in ports {
					let com = port.port_name;
					println!("{} ({})", com, get_mode(&com));
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
			let channel = &args[3];

			send(com, format!("COMMUNICATOR:{channel}").as_str());
		} else if args[1] == "scanner" {
			//Scanner
			let com = &args[2];
			let frequency = &args[3];

			send(com, format!("SCANNER:{frequency}").as_str());
		} else if args[1] == "tester" {
			//Tester
			let com = &args[2];
			let channel = &args[3];

			send(com, format!("TESTER:{channel}").as_str());
		} else {
			help();
		}
	} else {
		help();
	}

	let com = &args[2];

	let mut serial = serialport::new(com, 115_200)
		.timeout(Duration::from_millis(10))
		.open().expect("Failed to open port");

	println!("Set mode");

	loop {
		let mut line = String::from("");
		serial.read_to_string(&mut line).ok();

		if line != "" {
			let log = line.strip_suffix("\n").unwrap();
			println!("{}", log);
			log_to_server(log);
		}
	}
}

fn get_mode(com: &str) -> String {
	let mode = ask(com, "MODE");
	if mode.starts_with("MODE:") {
		return mode.split(":").nth(1).unwrap().to_owned()
	} else {
		return String::from("")
	}
}

fn send(com: &str, message: &str) {
	let mut serial = serialport::new(com, 115_200)
		.timeout(Duration::from_millis(10))
		.open().expect("Failed to open port");

	let message = String::from(message) + "\n";
	serial.write(message.as_bytes()).expect("Write failed!");
}

fn ask(com: &str, question: &str) -> String {
	let mut serial = serialport::new(com, 115_200) 
		.timeout(Duration::from_millis(10))
		.open().expect("Failed to open port");

	serial.write(question.as_bytes()).expect("Write failed!");

	loop {
		let mut line = String::from("");
		serial.read_to_string(&mut line).ok();

		if line != "" {
			break line.strip_suffix("\n").unwrap().to_owned();
		}
	}
}

fn log_to_server(log: &str) {
	println!("{:?}", log);
}

fn get_pending_actions(channel: &str) -> Vec<Action> {
	let mut vector_actions: Vec<Action> = Vec::new();

	let response = minreq::get("http://212.227.8.25:8123/pending/".to_owned() + channel).with_timeout(3).send().unwrap();
	if response.status_code == 200 {
		let actions: Value = serde_json::from_str(response.as_str().unwrap()).expect(&format!("Unable to parse json: {:?}", response));
		if actions["error_id"] != "null" {
			for action in actions.as_array().expect(&format!("Unable to parse to array: {:?}", actions)).iter() {
				vector_actions.push(serde_json::from_value(action.to_owned()).unwrap());
			}
		}
	}
	vector_actions
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