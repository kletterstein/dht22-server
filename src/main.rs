use std::io::Result;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use actix_web::{http, web, App, HttpResponse, HttpServer};
use once_cell::sync::Lazy;

use dht22_pi::Reading;


const DHT22_GPIO: u8 = 4;
static DHT22_MEASUREMENT: Lazy<Mutex<Reading>> = Lazy::new(|| {
    Mutex::new(Reading {temperature: 0.0, humidity: 0.0})
});

fn start_dht22(rx: Receiver<()>) -> Result<thread::JoinHandle<()>>  {
    Ok(
        thread::spawn( move ||
            loop {
                {
                    let mut meas = DHT22_MEASUREMENT.lock().unwrap();
                    *meas = dht22_pi::read(DHT22_GPIO).unwrap_or(*meas);
                }
                thread::sleep(Duration::from_secs(10));
                match rx.try_recv() {
                    Ok(_) | Err(TryRecvError::Disconnected) => {
                        println!("Measurement thread stopped");
                        break;
                    }
                    Err(TryRecvError::Empty) => {}
                }
            }
        )
    )
}

async fn temperature() -> HttpResponse {
    let meas = DHT22_MEASUREMENT.lock().unwrap();
    HttpResponse::Ok()
    .set_header(http::header::CONTENT_TYPE, "text/plain;charset=utf-8")
    .set_header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
    .body(format!("{:.1}", meas.temperature))
}

async fn humidity() -> HttpResponse  {
    let meas = DHT22_MEASUREMENT.lock().unwrap();
    HttpResponse::Ok()
    .set_header(http::header::CONTENT_TYPE, "text/plain;charset=utf-8")
    .set_header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
    .body(format!("{:.1}", meas.humidity))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let (tx, rx) = mpsc::channel::<()>();  // for sending termination message to measurement thread

    let thread_handle = start_dht22(rx)?;

    let result = HttpServer::new( || {
        App::new()
            .route("/temperature_C", web::get().to(temperature))
            .route("/humidity_pct", web::get().to(humidity))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await;

    println!("Terminating measurement thread...");
    let _ = tx.send(());  // Terminate thread
    thread_handle.join().unwrap();

    result
}
