use rocket::fs::{relative, FileServer};
use rocket_ws::WebSocket;

#[macro_use]
extern crate rocket;

#[get("/task")]
async fn echo_compose(ws: WebSocket) -> rocket_ws::Channel<'static> {
    use rocket::futures::{SinkExt, StreamExt};

    ws.channel(move |mut stream| {
        Box::pin(async move {
            while let Some(Ok(message)) = stream.next().await {
                let msg = message
                    .to_string()
                    .split(" ")
                    .map(|val| val.parse::<isize>())
                    .filter(|val| val.is_ok())
                    .map(|val| val.unwrap())
                    .filter(|&val| val > 0)
                    .map(|val| val.to_string())
                    .collect::<Vec<_>>()
                    .join(" ");
                stream.send(msg.into()).await.ok();
            }
            Ok(())
        })
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/websocket", routes![echo_compose])
        .mount("/", FileServer::from(relative!("static")))
}
