use nannou::prelude::*;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::runtime::Runtime;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create multithreaded executor
    let rt  = Runtime::new()?;

    // Spawn the main tokio loop, here it functions as a tcp echo server
    rt.spawn(async {
        // setup here for main loop
        let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

        loop {
            let (mut socket, _) = listener.accept().await.unwrap();

            tokio::spawn(async move {
                let mut buf = [0; 1024];

                // inner loop for a specific socket read & write
                loop {
                    let n = match socket.read(&mut buf).await {
                        // socket closed
                        Ok(n) if n == 0 => return,
                        Ok(n) => n,
                        Err(e) => {
                            println!("failed to read from socket; err = {:?}", e);
                            return;
                        }
                    };

                    // Write the data back
                    if let Err(e) = socket.write_all(&buf[0..n]).await {
                        println!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
            });
        }
    });

    nannou::app(model).update(update).simple_window(view).run();
    Ok(())
}

/// custom model definition
struct SomeModel {}

/// model updates are legit here
fn update(_app: &App, _model: &mut SomeModel, _update: Update) {}

/// render the model to the frame
fn view(_app: &App, _model: &SomeModel, _frame: Frame) {}

/// initial model construction
fn model(_app: &App) -> SomeModel {
    SomeModel {}
}

