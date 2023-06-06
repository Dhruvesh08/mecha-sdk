use std::process::Command;

fn main() {
    let args = [
        "v4l2src",
        "device=/dev/video0",
        "!",
        "video/x-raw, width=1280, height=720, framerate=30/1",
        "vpuenc_h264",
        "!",
        "filesink",
        "location=captured_video.h264",
    ];

    let output = Command::new("gst-launch-1.0")
        .args(&args)
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
