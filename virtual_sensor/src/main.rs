use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::{accept, Message};

enum Prior {
    Status,
}

fn main () {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        spawn (move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            let mut prior: Option<Prior> = None;
            loop {
                let msg = websocket.read().unwrap();

                if msg.is_text() {
                    let dat = msg.into_text().unwrap();
                    let dat = dat.trim();
                    println!("{:?}", dat);
                    match prior {
                        Some(Prior::Status) => {
                            match {
                                match &*dat {
                                    "000" => get_status_overall(),
                                    component => get_status_of(component),
                                }
                            } {
                                Some(fault_code) => {
                                    websocket.send(
                                        Message::Text("110".to_owned())
                                    ).unwrap();
                                    websocket.send(
                                        Message::Text(fault_code)
                                    ).unwrap();
                                },
                                None => {
                                    websocket.send(
                                        Message::Text("101".to_owned())
                                    ).unwrap();
                                },
                            }
                            prior = None;
                            continue;
                        },
                        None => (),
                    }
                    match &*dat {
                        "001" => websocket.send(
                            Message::Text("001".to_owned())
                        ).unwrap(),
                        "010" => prior = Some(Prior::Status),
                        "011" => {
                            websocket.send(
                                Message::Text("011".to_owned())
                            ).unwrap();
                            websocket.send(
                                Message::Text(get_fuel_level())
                            ).unwrap();
                        },
                        "100" => {
                            websocket.send(
                                Message::Text("100".to_owned())
                            ).unwrap();
                        },
                        _ => panic!("unknown command"),
                    }
                }
            }
        });
    }
}

fn get_fuel_level() -> String {
    "100110".to_owned()
}

fn get_status_overall() -> Option<String> {
    Some("111".to_owned())
}

fn get_status_of(component: &str) -> Option<String> {
    match component {
        "000001" => None,
        "000011" => Some("111".to_owned()),
        _ => panic!("unknown component"),
    }
}
