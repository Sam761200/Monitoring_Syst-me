use std::time::Duration;
use std::env;

mod metrics;
mod collector;
mod monitor;
mod examples;

use monitor::SystemMonitor;
use examples::{run_demo, run_short_monitoring, print_usage_help};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 === SYSTÈME DE MONITORING SYSTÈME ===");
    println!("📊 Collecte des métriques: CPU, Mémoire, Disque, Réseau\n");
    
    // Lire les variables d'environnement
    let mode = env::var("MONITOR_MODE").unwrap_or_else(|_| "normal".to_string());
    let interval_seconds: u64 = env::var("MONITOR_INTERVAL")
        .unwrap_or_else(|_| "5".to_string())
        .parse()
        .unwrap_or(5);
    
    // Traiter les arguments de ligne de commande
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "help" | "--help" | "-h" => {
                print_usage_help();
                return Ok(());
            }
            "demo" => {
                return run_demo().await;
            }
            "short" => {
                return run_short_monitoring().await;
            }
            _ => {
                println!("❌ Argument inconnu: {}", args[1]);
                println!("💡 Utilisez 'cargo run help' pour voir l'aide");
                return Ok(());
            }
        }
    }
    
    // Exécuter selon le mode défini
    match mode.as_str() {
        "demo" => {
            run_demo().await?;
        }
        "short" => {
            run_short_monitoring().await?;
        }
        "normal" => {
            let mut monitor = SystemMonitor::new();
            let interval = Duration::from_secs(interval_seconds);
            
            println!("🎯 Mode: Monitoring continu");
            monitor.start_monitoring(interval).await?;
        }
        _ => {
            println!("❌ Mode inconnu: {}", mode);
            println!("💡 Modes supportés: normal, demo, short");
            return Ok(());
        }
    }
    
    Ok(())
}
