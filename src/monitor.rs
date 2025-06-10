use std::time::Duration;
use tokio::{time, signal};
use crate::collector::MetricsCollector;
use crate::metrics::SystemMetrics;

pub struct SystemMonitor {
    collector: MetricsCollector,
}

impl SystemMonitor {
    pub fn new() -> Self {
        Self {
            collector: MetricsCollector::new(),
        }
    }

    pub async fn start_monitoring(&mut self, interval: Duration) -> Result<(), Box<dyn std::error::Error>> {
        println!("⏰ Intervalle de collecte: {:?}", interval);
        println!("⚡ Appuyez sur Ctrl+C pour arrêter le monitoring\n");

        let mut interval_timer = time::interval(interval);
        let mut iteration = 1;

        loop {
            tokio::select! {
                _ = interval_timer.tick() => {
                    println!("🔄 Collecte #{}", iteration);
                    let metrics = self.collect_metrics();
                    self.process_metrics(&metrics).await?;
                    iteration += 1;
                }
                _ = signal::ctrl_c() => {
                    println!("\n🛑 Arrêt du monitoring demandé...");
                    break;
                }
            }
        }

        println!("✅ Monitoring arrêté.");
        Ok(())
    }

    fn collect_metrics(&mut self) -> SystemMetrics {
        self.collector.collect()
    }

    async fn process_metrics(&self, metrics: &SystemMetrics) -> Result<(), Box<dyn std::error::Error>> {
        // Afficher un résumé des métriques
        metrics.display_summary();
        
        // Optionnel: sauvegarder en JSON
        self.save_metrics_to_file(metrics).await?;
        
        // Optionnel: détecter des alertes
        self.check_alerts(metrics);
        
        Ok(())
    }

    async fn save_metrics_to_file(&self, metrics: &SystemMetrics) -> Result<(), Box<dyn std::error::Error>> {
        let json_data = metrics.to_json()?;
        let filename = format!("metrics_{}.json", metrics.timestamp.format("%Y%m%d_%H%M%S"));
        
        tokio::fs::write(&filename, json_data).await?;
        println!("💾 Métriques sauvegardées dans: {}", filename);
        
        Ok(())
    }

    fn check_alerts(&self, metrics: &SystemMetrics) {
        let mut alerts = Vec::new();

        // Alertes CPU
        if metrics.cpu.usage_percent > 80.0 {
            alerts.push(format!("⚠️  CPU usage élevé: {:.1}%", metrics.cpu.usage_percent));
        }

        // Alertes mémoire
        if metrics.memory.usage_percent > 85.0 {
            alerts.push(format!("⚠️  Utilisation mémoire élevée: {:.1}%", metrics.memory.usage_percent));
        }

        // Alertes disque
        for disk in &metrics.disk.disks {
            if disk.usage_percent > 90.0 {
                alerts.push(format!("⚠️  Disque {} presque plein: {:.1}%", disk.name, disk.usage_percent));
            }
        }

        // Afficher les alertes
        if !alerts.is_empty() {
            println!("\n🚨 ALERTES:");
            for alert in alerts {
                println!("   {}", alert);
            }
        }
    }

    pub async fn collect_single_snapshot(&mut self) -> SystemMetrics {
        self.collector.collect()
    }

    pub fn get_system_info(&self) -> String {
        format!(
            "System Monitor v1.0\n\
             Collecte des métriques: CPU, Mémoire, Disque, Réseau\n\
             Plateforme: {}\n\
             Architecture: {}",
            std::env::consts::OS,
            std::env::consts::ARCH
        )
    }
} 