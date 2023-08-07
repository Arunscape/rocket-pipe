#[macro_use]
extern crate rocket;

use rocket::response::stream::ReaderStream;
use std::process::Stdio;
use tokio::process::ChildStdout;
use tokio::process::Command;

#[get("/")]
fn index() -> ReaderStream![ChildStdout] {
    // Start the process to generate the video file

    let mut process = Command::new("ls")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start process");

    let stdout = process.stdout.take().expect("Failed to get stdout");

    ReaderStream! {
        yield stdout
    }
}

#[get("/video")]
fn video() -> ReaderStream![ChildStdout] {
    let mut process = Command::new("yt-dlp")
        .arg("https://youtu.be/dQw4w9WgXcQ")
        .arg("-o")
        .arg("-")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start process");
    let stdout = process.stdout.take().expect("Failed to get stdout");

    ReaderStream! {
        yield stdout
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, video])
}
