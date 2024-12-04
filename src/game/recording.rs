use crate::{cache::CachedCall, CONFIG};
use axum::{extract::Request, response::IntoResponse, Extension};
use reqwest::{header::CONTENT_TYPE, Client as ReqwestClient};
use std::time::Duration;
use tokio::{
    process::Command,
    time::{sleep, timeout},
};
use twilio::{twiml::Twiml, Client as TwilioClient, Recording};

pub async fn recording_handler(
    twilio: Extension<TwilioClient>,
    request: Request,
) -> impl IntoResponse {
    twilio
        .clone()
        .respond_to_webhook_async(request, |recording: Recording| async move {
            log::debug!(
                "Received recording for call {} with id {}",
                recording.call_sid,
                recording.sid
            );

            // Download the recording
            let mp3 = twilio
                .download_recording(&recording.sid)
                .await
                .expect("Failed to download recording");

            let audio_path = format!("cache/recordings/{}/audio.mp3", recording.call_sid);

            // Ensure the necessary directory exists
            let _ =
                tokio::fs::create_dir_all(format!("cache/recordings/{}", recording.call_sid)).await;

            // Write the audio to disk
            tokio::fs::write(&audio_path, mp3)
                .await
                .expect("Failed to write recording");

            Twiml::new()
        })
        .await
}

pub async fn render_video(reqwest: ReqwestClient, call_sid: String, cached_call: CachedCall) {
    let audio_path = format!("cache/recordings/{call_sid}/audio.mp3");

    // Wait at most N seconds for the recording to be downloaded
    let duration = Duration::from_secs(CONFIG.settings.recording_timeout as u64);
    if timeout(duration, wait_for_recording(&audio_path))
        .await
        .is_err()
    {
        log::error!("Recording for call {call_sid} timed out, video rendering aborted");
        return;
    }

    // Download the sponsor's background image
    let response = reqwest
        .get(cached_call.sponsor.background_url)
        .send()
        .await
        .expect("Failed to download background image");

    let content_type = response
        .headers()
        .get(CONTENT_TYPE)
        .expect("Failed to get content type")
        .to_str()
        .expect("Failed to convert content type to string");

    let extension = match content_type {
        "image/jpeg" => "jpg",
        "image/png" => "png",
        _ => panic!("Unsupported content type"),
    };

    let background = response
        .bytes()
        .await
        .expect("Failed to download background image");

    // Ensure the necessary directories exist
    let _ = tokio::fs::create_dir_all(format!("cache/recordings/{call_sid}")).await;
    let _ = tokio::fs::create_dir_all("cache/drafts").await;

    let background_path = format!("cache/recordings/{call_sid}/background.{extension}",);
    let subtitles_path = format!("cache/recordings/{call_sid}/subtitles.srt");
    let comment_path = format!("cache/recordings/{call_sid}/comment.txt");
    let output_path = format!("cache/drafts/{call_sid}.mp4");

    tokio::fs::write(&background_path, background)
        .await
        .expect("Failed to write background image");

    let comment = format!("Sponsored by {}", cached_call.sponsor.name);
    tokio::fs::write(&comment_path, comment)
        .await
        .expect("Failed to write comment");

    let showwaves = "[0:a]showwaves=size=400x400:colors=white:draw=full:mode=cline[v]";
    let rounding = "[v]format=rgba,geq='p(mod((2*W/(2*PI))*(PI+atan2(0.5*H-Y,X-W/2)),W),H-2*hypot(0.5*H-Y,X-W/2))':a='1*alpha(mod((2*W/(2*PI))*(PI+atan2(0.5*H-Y,X-W/2)),W),H-2*hypot(0.5*H-Y,X-W/2))'[vout]";
    let overlay = "[1:v][vout]overlay=(W-w)/2:(H-h)/2";
    let subtitles = format!("subtitles={subtitles_path}");
    let pad = "pad=ceil(iw/2)*2:ceil(ih/2)*2[outv]";
    let filter_complex = format!("{showwaves};{rounding};{overlay},{subtitles},{pad}");

    match Command::new("ffmpeg")
        .args(&["-i", &audio_path])
        .args(&["-i", &background_path])
        .args(&["-filter_complex", &filter_complex])
        .args(&["-map", "[outv]"])
        .args(&["-map", "0:a"])
        .args(&["-c:v", "libx264"])
        .args(&["-profile:v", "high"])
        .args(&["-level", "4.1"])
        .args(&["-pix_fmt", "yuv420p"])
        .args(&["-c:a", "aac"])
        .args(&["-r", "30"])
        .arg("-shortest")
        .arg(&output_path)
        .output()
        .await
    {
        Err(e) => log::error!("Failed to create video: {}", e),
        Ok(output) => match output.status.success() {
            true => {
                log::debug!("Successfully created video");
            }
            false => {
                log::error!(
                    "Failed to create video: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        },
    }
}

async fn wait_for_recording(path: &str) {
    loop {
        match tokio::fs::try_exists(path).await {
            Ok(true) => break,
            _ => sleep(Duration::from_secs(1)).await,
        }
    }
}
