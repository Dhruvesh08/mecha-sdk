use std::process::Command;

fn main() {
    let output = Command::new("gst-launch-1.0")
        .args(&[
            "v4l2src",
            "device=/dev/video0",
            "!",
            "video/x-raw, width=1280, height=720, framerate=30/1",
            "!",
            "vpuenc_h264",
            "!",
            "filesink",
            "location=capture_video.h264",
        ])
        .output()
        .expect("Failed to execute command.");

    if output.status.success() {
        println!("Command executed successfully.");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Command failed with error: {}", stderr);
    }
}
