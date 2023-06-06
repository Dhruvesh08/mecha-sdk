use std::process::Command;

fn main() {
    let output = Command::new("gst-launch-1.0")
        .args(&[
            "autovideosrc",
            "!",
            "queue",
            "!",
            "videoconvert",
            "!",
            "jpegenc",
            "!",
            "filesink",
            "location=captured_image.jpg",
        ])
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Image captured successfully!");
    } else {
        println!("Command failed with error: {:?}", output.status);
        if let Ok(stderr) = String::from_utf8(output.stderr) {
            println!("Error message: {}", stderr);
        }
    }
}