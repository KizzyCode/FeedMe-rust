//! Get file information

use feedme_shared::{error, Entry, Error, UuidBuilder};
use std::{
    path::Path,
    process::Command,
    time::{Duration, UNIX_EPOCH},
};

/// Collects the metadata for a video file
pub fn read_metadata(file: &str) -> Result<Entry, Error> {
    let file = Path::new(file);
    let metadata = file.metadata()?;

    // Get the filename
    let video_name = file.file_name().and_then(|name| name.to_str())
        // Reject non-UTF-8 filenames
        .ok_or(error!("Failed to get filename from entry"))?;

    // Get duration
    eprintln!("[feedme-manual] Using ffprobe to get duration for: {video_name}");
    let duration = ffprobe_duration(file)?.as_secs();

    // Compute UUID
    eprintln!("[feedme-manual] Computing UUID for: {video_name}");
    let uuid = UuidBuilder::new().context(b"feedme.manual").finalize(video_name)?;

    // Get file size and birth
    let size = metadata.len();
    let date = metadata.created()?.duration_since(UNIX_EPOCH)?.as_secs();

    // Get type
    let type_ = match file.extension() {
        Some(ext) if ext == "mp4" => "video/mp4".to_string(),
        Some(ext) if ext == "m4v" => "video/mp4".to_string(),
        _ => return Err(error!("Unknown file type")),
    };

    // Create entry
    let file = video_name.to_string();
    let title = video_name.to_string();
    let description = None;
    Ok(Entry { file, uuid, size, type_, duration, date, title, description })
}

/// Uses ffprobe to get the file duration
fn ffprobe_duration(file: &Path) -> Result<Duration, Error> {
    // Call ffprobe to get duration:
    //  `ffprobe -i <file> -show_entries format=duration -of csv="p=0"`
    let result = Command::new("ffprobe")
        // Input file
        .arg("-i").arg(file.as_os_str())
        // Output format
        .arg("-show_entries").arg("format=duration").arg("-of").arg("csv=p=0")
        // Run the command
        .output()?;

    // Check for success
    let true = result.status.success() else {
        // Fail with stderr
        let stderr = String::from_utf8_lossy(&result.stderr);
        return Err(error!("Failed to run ffprobe: {stderr}"));
    };

    // Get and parse stdout
    let stdout = String::from_utf8(result.stdout)?;
    let duration_secs: f64 = stdout.parse()?;
    Ok(Duration::from_secs_f64(duration_secs))
}
