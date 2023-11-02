use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::{accept, Message};

use playback_io::read_file;

/// A WebSocket echo server
fn main () {
    match TcpListener::bind("127.0.0.1:9001")
    {
        Ok( server ) =>
        {
            for stream in server.incoming() {
                spawn (move || {
                    let mut websocket = accept(stream.unwrap()).unwrap();
                    loop {
                        match websocket.read_message()
                        {
                            Ok(val) =>
                            {
                                // String Recieved from client
                                let action = val.to_text().unwrap();

                                if action == "shutdown"
                                {
                                    websocket.close(None);
                                } else {
                                    let path = format!("../data/{}.wav", action); // path = ../data/unreleased_carti.wav

                                    let msg = read_file(&*path).unwrap();

                                    // ------------- This variable and the match function below streams out the wav header
                                    let wav_head = format!("{:?}", msg.0);
                                    match websocket.write_message(Message::Text(wav_head) )
                                    {
                                        Ok(()) =>
                                        {
                                            println!("Success");
                                        },
                                        Err(E) =>
                                        {
                                            println!("ERR: {}", E);
                                        }
                                    }

                                    // ------------- This variable and the match function below streams out the wav body
                                    
                                    let mut n = 0;

                                    let wav_body = msg.1.clone();
                                    let wav_body_length = wav_body.clone().len();
                                    let length_factor = wav_body_length / 16;
                                    while n < wav_body.clone().len()
                                    {
                                        let index = n + length_factor;
                                        let mut vec: Vec<f32> = Vec::new();
                                        vec = wav_body[n..index].to_vec();
                                        let _wav_body_0 = format!("{:?}", vec);
                                        match websocket.write_message(Message::Text(_wav_body_0) )
                                        {
                                            Ok(()) =>
                                            {
                                                let action = val.to_text().unwrap();

                                                if action == "shutdown"
                                                {
                                                    websocket.close(None);
                                                }
                                            },
                                            Err(E) =>
                                            {
                                                println!("ERR: {0} on index {1}", E, n.clone());
                                            }
                                        }
                                        n = n + index;
                                    }
                                }
                            }
                            Err(E) =>
                            {
                                println!("ERR: {}", E);
                            }
                        }
                    }
                });
            }
        },
        Err(E) =>
        {
            println!("ERR: {}", E);
        }
    }
}
