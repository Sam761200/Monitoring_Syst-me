use sysinfo::{System, Disks, Networks};
use chrono::Utc;
use crate::metrics::*;

pub struct MetricsCollector {
    system: System,
    disks: Disks,
    networks: Networks,
}

impl MetricsCollector {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        
        let disks = Disks::new_with_refreshed_list();
        let networks = Networks::new_with_refreshed_list();
        
        Self { system, disks, networks }
    }

    pub fn collect(&mut self) -> SystemMetrics {
        // Rafraîchir toutes les informations système
        self.system.refresh_all();
        self.disks.refresh(true);
        self.networks.refresh(true);
        
        let timestamp = Utc::now();
        
        SystemMetrics {
            timestamp,
            cpu: self.collect_cpu_metrics(),
            memory: self.collect_memory_metrics(),
            disk: self.collect_disk_metrics(),
            network: self.collect_network_metrics(),
        }
    }

    fn collect_cpu_metrics(&self) -> CpuMetrics {
        let cpus = self.system.cpus();
        
        // Calculer l'usage global du CPU
        let total_usage: f32 = cpus.iter().map(|cpu| cpu.cpu_usage()).sum();
        let usage_percent = if !cpus.is_empty() {
            total_usage / cpus.len() as f32
        } else {
            0.0
        };

        // Collecter les métriques par cœur
        let cores: Vec<CoreMetrics> = cpus
            .iter()
            .enumerate()
            .map(|(id, cpu)| CoreMetrics {
                core_id: id,
                usage_percent: cpu.cpu_usage(),
            })
            .collect();

        // Load average (sur certains systèmes)
        let load_avg = System::load_average();
        let load_average = LoadAverage {
            one_min: load_avg.one,
            five_min: load_avg.five,
            fifteen_min: load_avg.fifteen,
        };

        CpuMetrics {
            usage_percent,
            cores,
            load_average,
        }
    }

    fn collect_memory_metrics(&self) -> MemoryMetrics {
        let total_memory = self.system.total_memory();
        let used_memory = self.system.used_memory();
        let available_memory = self.system.available_memory();
        
        let usage_percent = if total_memory > 0 {
            (used_memory as f64 / total_memory as f64) * 100.0
        } else {
            0.0
        };

        MemoryMetrics {
            total_bytes: total_memory,
            used_bytes: used_memory,
            available_bytes: available_memory,
            usage_percent,
            swap_total_bytes: self.system.total_swap(),
            swap_used_bytes: self.system.used_swap(),
        }
    }

    fn collect_disk_metrics(&self) -> DiskMetrics {
        let disks: Vec<DiskInfo> = self
            .disks
            .iter()
            .map(|disk| {
                let total_space = disk.total_space();
                let available_space = disk.available_space();
                let used_space = total_space - available_space;
                
                let usage_percent = if total_space > 0 {
                    (used_space as f64 / total_space as f64) * 100.0
                } else {
                    0.0
                };

                DiskInfo {
                    name: disk.name().to_string_lossy().to_string(),
                    mount_point: disk.mount_point().to_string_lossy().to_string(),
                    total_space_bytes: total_space,
                    available_space_bytes: available_space,
                    used_space_bytes: used_space,
                    usage_percent,
                    file_system: disk.file_system().to_string_lossy().to_string(),
                }
            })
            .collect();

        DiskMetrics {
            disks,
            total_read_bytes: 0, // Non disponible facilement dans cette API
            total_written_bytes: 0,
        }
    }

    fn collect_network_metrics(&self) -> NetworkMetrics {
        let mut total_bytes_received = 0;
        let mut total_bytes_transmitted = 0;

        let interfaces: Vec<NetworkInterface> = self
            .networks
            .iter()
            .map(|(interface_name, network)| {
                total_bytes_received += network.total_received();
                total_bytes_transmitted += network.total_transmitted();

                NetworkInterface {
                    name: interface_name.clone(),
                    bytes_received: network.total_received(),
                    bytes_transmitted: network.total_transmitted(),
                    packets_received: network.packets_received(),
                    packets_transmitted: network.packets_transmitted(),
                    errors_on_received: network.errors_on_received(),
                    errors_on_transmitted: network.errors_on_transmitted(),
                }
            })
            .collect();

        NetworkMetrics {
            interfaces,
            total_bytes_received,
            total_bytes_transmitted,
        }
    }
} 