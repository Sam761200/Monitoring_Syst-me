use std::time::Duration;
use crate::monitor::SystemMonitor;

pub async fn run_demo() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 === DÉMONSTRATION DU SYSTÈME DE MONITORING ===\n");
    
    let mut monitor = SystemMonitor::new();
    
    // Afficher les informations du système
    println!("{}\n", monitor.get_system_info());
    
    // Collecter un snapshot unique
    println!("📸 Collecte d'un snapshot unique...");
    let snapshot = monitor.collect_single_snapshot().await;
    snapshot.display_summary();
    
    // Sauvegarder le snapshot
    let json_data = snapshot.to_json()?;
    tokio::fs::write("demo_snapshot.json", json_data).await?;
    println!("💾 Snapshot sauvegardé dans: demo_snapshot.json");
    
    Ok(())
}

pub async fn run_short_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 === MONITORING COURT (30 secondes) ===\n");
    
    let mut monitor = SystemMonitor::new();
    
    // Surveiller pendant 30 secondes avec des intervalles de 5 secondes
    let monitoring_duration = Duration::from_secs(30);
    let collection_interval = Duration::from_secs(5);
    
    println!("⏱️  Monitoring pendant {:?} avec des intervalles de {:?}", 
             monitoring_duration, collection_interval);
    
    let start_time = std::time::Instant::now();
    let mut interval_timer = tokio::time::interval(collection_interval);
    let mut iteration = 1;
    
    while start_time.elapsed() < monitoring_duration {
        interval_timer.tick().await;
        
        println!("🔄 Collecte #{} ({}s écoulées)", iteration, start_time.elapsed().as_secs());
        let metrics = monitor.collect_single_snapshot().await;
        metrics.display_summary();
        
        iteration += 1;
    }
    
    println!("✅ Monitoring de démonstration terminé.");
    Ok(())
}

pub fn print_usage_help() {
    println!("📋 === AIDE D'UTILISATION ===\n");
    println!("Options disponibles:");
    println!("  - Monitoring continu : lance le programme normalement");
    println!("  - Mode démo         : collecte un snapshot unique");
    println!("  - Monitoring court  : surveille pendant 30 secondes");
    println!();
    println!("Variables d'environnement:");
    println!("  MONITOR_INTERVAL    : intervalle de collecte en secondes (défaut: 5)");
    println!("  MONITOR_NO_SAVE     : désactive la sauvegarde automatique (true/false)");
    println!("  MONITOR_MODE        : mode d'exécution (demo/short/normal)");
    println!();
    println!("Exemples:");
    println!("  cargo run");
    println!("  MONITOR_MODE=demo cargo run");
    println!("  MONITOR_MODE=short cargo run");
    println!("  MONITOR_INTERVAL=10 cargo run");
} 