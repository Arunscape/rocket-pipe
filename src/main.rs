#[macro_use]
extern crate rocket;

use rocket::response::stream::{ReaderStream};
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

    ReaderStream::one(stdout)
}

#[get("/video")]
fn video() -> ReaderStream![ChildStdout] {
    // Start the process to generate the video file

    let mut process = Command::new("yt-dlp")
        .arg("https://youtu.be/dQw4w9WgXcQ")
        .arg("-o")
        .arg("-")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start process");

    let stdout = process.stdout.take().expect("Failed to get stdout");

    ReaderStream::one(stdout)
}



// #[get("/download")]
// fn download<'r>() -> Response<'_>{
//     let mut process = Command::new("yt-dlp")
//         .arg("https://youtu.be/dQw4w9WgXcQ")
//         .arg("-o")
//         .arg("-")
//         .stdout(Stdio::piped())
//         .spawn()
//         .expect("Failed to start process");
//     let stdout = process.stdout.take().expect("Failed to get stdout");

//     Response::build()
//         .streamed_body(stdout)
//         .header(ContentType::MP4)
//         .header(Header::new("Content-Type", "video/mp4"))
//         .header(Header::new("Content-Disposition", "attachment; filename=\"video.mp4\""))
//         .finalize()
// } 

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, video])
}
