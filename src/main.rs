#[macro_use]
extern crate rocket;

use rocket::futures::stream::{Stream, StreamExt};
use rocket::http::ContentType;
use rocket::response::{self, stream::ReaderStream, Responder};
use rocket::{Request, Response};
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::ChildStdout;
use tokio::process::Command;

#[get("/")]
fn index() -> ReaderStream![ChildStdout] {
    // Start the process to generate the video file
    // let process = Command::new("yt-dlp")
    //     .arg("https://youtu.be/dQw4w9WgXcQ")
    //     .arg("-o")
    //     .arg("-")
    //     .stdout(Stdio::piped())
    //     .spawn()
    //     .expect("Failed to start process");
    let mut process = Command::new("ls")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start process");

    let mut stdout = process.stdout.take().expect("Failed to get stdout");

    ReaderStream!{
        yield stdout
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
