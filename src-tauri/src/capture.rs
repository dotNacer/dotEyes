use ffmpeg_next as ffmpeg;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::path::PathBuf;
use chrono::Local;
use std::process::Command;
use std::fs;

pub struct ScreenRecorder {
    is_recording: Arc<AtomicBool>,
    output_path: PathBuf,
}

impl ScreenRecorder {
    pub fn new() -> Self {
        ffmpeg::init().unwrap();
        
        // Créer le chemin vers le dossier Videos
        let home_dir = dirs::home_dir().expect("Impossible de trouver le dossier home");
        let videos_dir = home_dir.join("Videos");
        
        // Créer le dossier Videos s'il n'existe pas
        fs::create_dir_all(&videos_dir).expect("Impossible de créer le dossier Videos");
        
        Self {
            is_recording: Arc::new(AtomicBool::new(false)),
            output_path: videos_dir,
        }
    }

    pub fn is_recording(&self) -> bool {
        self.is_recording.load(Ordering::SeqCst)
    }

    pub fn start_recording(&self) -> Result<(), String> {
        if self.is_recording.load(Ordering::SeqCst) {
            return Err("Already recording".to_string());
        }

        // Générer un nom de fichier unique avec la date et l'heure
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let output_file = self.output_path.join(format!("screen_recording_{}.mp4", timestamp));
        
        // Utiliser ffmpeg pour X11
        let _status = Command::new("ffmpeg")
            .arg("-f")
            .arg("x11grab")  // Capture X11
            .arg("-framerate")
            .arg("30")       // 30 FPS
            .arg("-i")
            .arg(":0.0")     // Display X11
            .arg("-c:v")
            .arg("libx264")  // Codec vidéo
            .arg("-preset")
            .arg("ultrafast")
            .arg("-qp")
            .arg("0")        // Qualité maximale
            .arg("-pix_fmt")
            .arg("yuv420p")  // Format compatible
            .arg(output_file)
            .spawn()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    "ffmpeg n'est pas installé. Veuillez l'installer avec : sudo apt install ffmpeg".to_string()
                } else {
                    format!("Erreur lors du démarrage de l'enregistrement: {}", e)
                }
            })?;

        self.is_recording.store(true, Ordering::SeqCst);
        Ok(())
    }

    pub fn stop_recording(&self) -> Result<(), String> {
        if !self.is_recording.load(Ordering::SeqCst) {
            return Err("Not recording".to_string());
        }

        // Arrêter ffmpeg
        let _status = Command::new("pkill")
            .arg("-f")
            .arg("ffmpeg")
            .status()
            .map_err(|e| format!("Erreur lors de l'arrêt de l'enregistrement: {}", e))?;

        self.is_recording.store(false, Ordering::SeqCst);
        Ok(())
    }
} 