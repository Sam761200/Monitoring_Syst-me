use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub timestamp: DateTime<Utc>,
    pub cpu: CpuMetrics,
    pub memory: MemoryMetrics,
    pub disk: DiskMetrics,
    pub network: NetworkMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuMetrics {
    pub usage_percent: f32,
    pub cores: Vec<CoreMetrics>,
    pub load_average: LoadAverage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreMetrics {
    pub core_id: usize,
    pub usage_percent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadAverage {
    pub one_min: f64,
    pub five_min: f64,
    pub fifteen_min: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetrics {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f64,
    pub swap_total_bytes: u64,
    pub swap_used_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskMetrics {
    pub disks: Vec<DiskInfo>,
    pub total_read_bytes: u64,
    pub total_written_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total_space_bytes: u64,
    pub available_space_bytes: u64,
    pub used_space_bytes: u64,
    pub usage_percent: f64,
    pub file_system: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    pub interfaces: Vec<NetworkInterface>,
    pub total_bytes_received: u64,
    pub total_bytes_transmitted: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub bytes_received: u64,
    pub bytes_transmitted: u64,
    pub packets_received: u64,
    pub packets_transmitted: u64,
    pub errors_on_received: u64,
    pub errors_on_transmitted: u64,
}

impl SystemMetrics {
    pub fn new() -> Self {
        Self {
            timestamp: Utc::now(),
            cpu: CpuMetrics {
                usage_percent: 0.0,
                cores: Vec::new(),
                load_average: LoadAverage {
                    one_min: 0.0,
                    five_min: 0.0,
                    fifteen_min: 0.0,
                },
            },
            memory: MemoryMetrics {
                total_bytes: 0,
                used_bytes: 0,
                available_bytes: 0,
                usage_percent: 0.0,
                swap_total_bytes: 0,
                swap_used_bytes: 0,
            },
            disk: DiskMetrics {
                disks: Vec::new(),
                total_read_bytes: 0,
                total_written_bytes: 0,
            },
            network: NetworkMetrics {
                interfaces: Vec::new(),
                total_bytes_received: 0,
                total_bytes_transmitted: 0,
            },
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn display_summary(&self) {
        println!("\n📊 === MÉTRIQUES SYSTÈME ({}) ===", self.timestamp.format("%Y-%m-%d %H:%M:%S UTC"));
        
        // CPU
        println!("🖥️  CPU:");
        println!("   Usage global: {:.1}%", self.cpu.usage_percent);
        println!("   Nombre de cœurs: {}", self.cpu.cores.len());
        
        // Mémoire
        println!("💾 Mémoire:");
        println!("   Total: {:.2} GB", self.memory.total_bytes as f64 / 1_073_741_824.0);
        println!("   Utilisée: {:.2} GB ({:.1}%)", 
                 self.memory.used_bytes as f64 / 1_073_741_824.0, 
                 self.memory.usage_percent);
        println!("   Disponible: {:.2} GB", self.memory.available_bytes as f64 / 1_073_741_824.0);
        
        // Disques
        println!("💿 Disques:");
        for disk in &self.disk.disks {
            println!("   {}: {:.2} GB utilisés / {:.2} GB total ({:.1}%)",
                     disk.name,
                     disk.used_space_bytes as f64 / 1_073_741_824.0,
                     disk.total_space_bytes as f64 / 1_073_741_824.0,
                     disk.usage_percent);
        }
        
        // Réseau
        println!("🌐 Réseau:");
        println!("   Total reçu: {:.2} MB", self.network.total_bytes_received as f64 / 1_048_576.0);
        println!("   Total envoyé: {:.2} MB", self.network.total_bytes_transmitted as f64 / 1_048_576.0);
        
        println!("{}", "=".repeat(50));
    }
} 