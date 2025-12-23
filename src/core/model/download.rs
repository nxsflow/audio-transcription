use std::path::PathBuf;
use crate::error::{Result, AudioTranscriptionError};
use crate::core::model::ModelSize;
use reqwest;
use futures_util::StreamExt;
use std::io::Write;
use chrono;

/// Get the full path to a whisper model file
fn get_whisper_model_path(cache_dir: &PathBuf, size: &ModelSize) -> PathBuf {
    cache_dir.join("whisper").join(size.to_string()).join(format!("ggml-{}.bin", size))
}

/// Get the pyannote model directory
fn get_pyannote_model_dir(cache_dir: &PathBuf) -> PathBuf {
    cache_dir.join("pyannote")
}

/// Get the full path to the pyannote setup marker file
fn get_pyannote_model_path(cache_dir: &PathBuf) -> PathBuf {
    get_pyannote_model_dir(cache_dir).join("setup_complete.txt")
}

/// Get the full path to the pyannote segmentation model
fn get_pyannote_segmentation_model_path(cache_dir: &PathBuf) -> PathBuf {
    // The segmentation model extracts to a subdirectory with the same name as the archive
    get_pyannote_model_dir(cache_dir)
        .join("sherpa-onnx-pyannote-segmentation-3-0")
        .join("model.onnx")
}

/// Get the full path to the speaker embedding model
fn get_speaker_embedding_model_path(cache_dir: &PathBuf) -> PathBuf {
    get_pyannote_model_dir(cache_dir).join("3dspeaker_speech_eres2net_base_sv_zh-cn_3dspeaker_16k.onnx")
}

/// Download a model file from the given URL to the specified path
pub async fn download_model(url: &str, destination: &PathBuf) -> Result<()> {
    // Create parent directory if it doesn't exist
    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| AudioTranscriptionError::Io(e))?;
    }

    // Create HTTP client and start the download
    let client = reqwest::Client::new();
    let request = client.get(url);
    let response = request.send().await?;
    
    if !response.status().is_success() {
        return Err(AudioTranscriptionError::Network(
            reqwest::Error::from(response.error_for_status().unwrap_err())
        ));
    }

    // Create the destination file
    let mut file = std::fs::File::create(destination)
        .map_err(|e| AudioTranscriptionError::Io(e))?;

    // Stream the response body to the file
    let mut stream = response.bytes_stream();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk)
            .map_err(|e| AudioTranscriptionError::Io(e))?;
    }

    // Validate the downloaded file exists and has content
    let metadata = std::fs::metadata(destination)
        .map_err(|e| AudioTranscriptionError::Io(e))?;
    
    if metadata.len() == 0 {
        return Err(AudioTranscriptionError::Model(
            "Downloaded model file is empty".to_string()
        ));
    }

    Ok(())
}

/// Download the Whisper transcription model for the specified size
pub async fn download_transcription_model(cache_dir: &PathBuf, model_size: &ModelSize) -> Result<()> {
    let model_path = get_whisper_model_path(cache_dir, model_size);
    
    println!("Downloading Whisper {} model...", model_size);
    
    // Construct the download URL for whisper model
    // Using the official whisper.cpp model repository
    let whisper_url = format!(
        "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-{}.bin",
        model_size
    );
    
    match download_model(&whisper_url, &model_path).await {
        Ok(_) => {
            println!("✅ Whisper {} model downloaded successfully", model_size);
            Ok(())
        }
        Err(e) => {
            println!("❌ Failed to download Whisper model: {}", e);
            Err(e)
        }
    }
}

/// Download and setup the sherpa-onnx diarization models
/// Downloads ONNX models for speaker segmentation and embedding extraction
pub async fn download_diarization_model(cache_dir: &PathBuf, _unused_token: &str) -> Result<()> {
    println!("Setting up sherpa-onnx speaker diarization models...");
    
    // Download pyannote segmentation model (sherpa-onnx format)
    let segmentation_model_path = get_pyannote_segmentation_model_path(cache_dir);
    let segmentation_url = "https://github.com/k2-fsa/sherpa-onnx/releases/download/speaker-segmentation-models/sherpa-onnx-pyannote-segmentation-3-0.tar.bz2";
    
    println!("  📥 Downloading pyannote segmentation model...");
    
    // Create a temporary file for the compressed model
    let temp_dir = std::env::temp_dir();
    let temp_file = temp_dir.join("pyannote-segmentation.tar.bz2");
    
    // Download the compressed model
    match download_model(segmentation_url, &temp_file).await {
        Ok(_) => {
            println!("  ✅ Segmentation model downloaded");
            
            // Extract the model
            println!("  📦 Extracting segmentation model...");
            if let Err(e) = extract_tar_bz2(&temp_file, &get_pyannote_model_dir(cache_dir)).await {
                return Err(AudioTranscriptionError::Model(
                    format!("Failed to extract segmentation model: {}", e)
                ));
            }
            
            // Clean up temp file
            let _ = std::fs::remove_file(&temp_file);
            println!("  ✅ Segmentation model extracted successfully");
        }
        Err(e) => {
            println!("  ❌ Failed to download segmentation model: {}", e);
            return Err(e);
        }
    }
    
    // Download speaker embedding model (3D-Speaker)
    let embedding_model_path = get_speaker_embedding_model_path(cache_dir);
    let embedding_url = "https://github.com/k2-fsa/sherpa-onnx/releases/download/speaker-recongition-models/3dspeaker_speech_eres2net_base_sv_zh-cn_3dspeaker_16k.onnx";
    
    println!("  📥 Downloading speaker embedding model...");
    
    match download_model(embedding_url, &embedding_model_path).await {
        Ok(_) => {
            println!("  ✅ Speaker embedding model downloaded successfully");
        }
        Err(e) => {
            println!("  ❌ Failed to download embedding model: {}", e);
            return Err(e);
        }
    }
    
    // Create a marker file to indicate setup is complete
    let marker_path = get_pyannote_model_path(cache_dir);
    std::fs::write(&marker_path, format!(
        "Sherpa-ONNX diarization setup completed at: {}\n\
        Segmentation model: sherpa-onnx-pyannote-segmentation-3-0\n\
        Embedding model: 3dspeaker_speech_eres2net_base_sv_zh-cn_3dspeaker_16k\n\
        \n\
        Models are ready for speaker diarization using sherpa-onnx.\n\
        Segmentation model: {}\n\
        Embedding model: {}\n",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
        segmentation_model_path.display(),
        embedding_model_path.display()
    )).map_err(|e| AudioTranscriptionError::Io(e))?;
    
    println!("✅ Sherpa-ONNX diarization models setup completed successfully");
    
    Ok(())
}

/// Extract a tar.bz2 file to the specified directory
async fn extract_tar_bz2(archive_path: &PathBuf, extract_to: &PathBuf) -> Result<()> {
    use std::process::Command;
    
    // Create the extraction directory
    std::fs::create_dir_all(extract_to)
        .map_err(|e| AudioTranscriptionError::Io(e))?;
    
    // Use tar command to extract
    let output = Command::new("tar")
        .args(&["-xjf", &archive_path.to_string_lossy(), "-C", &extract_to.to_string_lossy()])
        .output()
        .map_err(|e| AudioTranscriptionError::Io(e))?;
    
    if !output.status.success() {
        return Err(AudioTranscriptionError::Model(
            format!("Failed to extract archive: {}", String::from_utf8_lossy(&output.stderr))
        ));
    }
    
    Ok(())
}

/// Check if a transcription model is available
pub fn is_transcription_model_available(cache_dir: &PathBuf, model_size: &ModelSize) -> bool {
    let model_path = get_whisper_model_path(cache_dir, model_size);
    model_path.exists() && 
    std::fs::metadata(&model_path)
        .map(|m| m.len() > 0)
        .unwrap_or(false)
}

/// Check if diarization model setup is complete
pub fn is_diarization_model_available(cache_dir: &PathBuf) -> bool {
    // Check if both required ONNX model files exist
    let segmentation_model = get_pyannote_segmentation_model_path(cache_dir);
    let embedding_model = get_speaker_embedding_model_path(cache_dir);
    
    segmentation_model.exists() && embedding_model.exists() &&
    std::fs::metadata(&segmentation_model)
        .map(|m| m.len() > 0)
        .unwrap_or(false) &&
    std::fs::metadata(&embedding_model)
        .map(|m| m.len() > 0)
        .unwrap_or(false)
}