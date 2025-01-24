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
        
        // Utilisation de wf-recorder pour Wayland
        let _status = Command::new("wf-recorder")
            .arg("--file")
            .arg(&output_file)
            .arg("--codec")
            .arg("h264")
            .arg("--pixel-format")
            .arg("yuv420p")  // Format compatible avec la plupart des lecteurs
            .spawn()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    "wf-recorder n'est pas installé. Veuillez l'installer avec : sudo apt install wf-recorder".to_string()
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

        // Arrêter wf-recorder
        let _status = Command::new("pkill")
            .arg("wf-recorder")
            .status()
            .map_err(|e| format!("Erreur lors de l'arrêt de l'enregistrement: {}", e))?;

        self.is_recording.store(false, Ordering::SeqCst);
        Ok(())
    }
} 