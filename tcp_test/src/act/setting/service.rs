use std::io;
use std::sync::Mutex;

use calamine::{Sheets, DataType};

use futures::{BoxFuture, Future};
use futures_cpupool::CpuPool;

use tokio_service::Service;
use serde_json;

lazy_static!{
    static ref EXCELLDB: Mutex<Sheets> = {
        Mutex::new(Sheets::open("./test.xlsx").unwrap())
    };
}
#[derive(Serialize,Deserialize,Debug)]
pub enum State {
    Error(usize),
    ReqSearch { content: String },
    ReqBorrow { content: String },
    RespSearch { find: bool, list: Vec<String> },
    RespBorrow { success: bool },
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Message {
    id: usize,
    messag: State,
}
impl Message {
    pub fn new(id: usize, messag: State) -> Self {
        Message { id, messag }
    }
}
pub struct DBserver;
impl Service for DBserver {
    // These types must match the corresponding protocol types:
    type Request = String;
    type Response = String;

    // For non-streaming protocols, service errors are always io::Error
    type Error = io::Error;
    // The future ting the response; box it for simplicity.
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let mut db = EXCELLDB
            .lock()
            .unwrap()
            .worksheet_range_by_index(0)
            .unwrap();
        println!("start {:?},end {:?}", db.start(), db.end());
        let msg = CpuPool::new(10).spawn_fn(move || match serde_json::from_str::<Message>(&req) {
                                                Ok(ref result) => {
            let respond = match *result {
                Message { id, ref messag } => {
                    match *messag {
                        State::ReqSearch { ref content } => {
                            let result = db.rows()
                                .fold(Vec::new(), |mut acc, x| {
                                    if let DataType::String(inner) = x[1].clone() {
                                        if inner.contains(content) {
                                            acc.push(inner);
                                        }
                                    }
                                    acc
                                });
                            Message {
                                id,
                                messag: State::RespSearch {
                                    find: !result.is_empty(),
                                    list: result,
                                },
                            }
                        }
                        State::ReqBorrow { ref content } => {
                            let mut available: Vec<(u32, u32)> = Vec::new();
                            for (index, item) in db.rows().enumerate() {
                                if let DataType::String(inner) = item[1].clone() {
                                    println!("inner: {}", inner);
                                    if inner == *content {
                                        if let DataType::String(current) = item[2].clone() {
                                            println!("bool {}", current);
                                            if current == "unborrow" {
                                                available.push((index as u32, 2));
                                            }
                                        }
                                    }
                                }
                            }
                            for h in available.iter_mut() {
                                db.set_value(*h, DataType::String("borrow".to_string()))
                                    .expect("fail to set value");
                            }
                            Message {
                                id,
                                messag: State::RespBorrow { success: !available.is_empty() },
                            }
                        }
                        _ => {
                            panic!("wrong request");
                        }
                    }
                }
            };
            Ok(respond)
        }
                                                Err(_) => {
                                                    Ok(Message {
                                                           id: 0,
                                                           messag: State::Error(0),
                                                       })
                                                }
                                            });

        msg.map(|msg| serde_json::to_string(&msg).unwrap())
            .boxed()
    }
}
