# 🚀 Système de Monitoring Système

Un système de monitoring en temps réel écrit en Rust pour collecter et surveiller les métriques système.

## 📋 Fonctionnalités

- **CPU** : Usage global et par cœur, load average
- **Mémoire** : Utilisation RAM et swap
- **Disque** : Espace utilisé/disponible par disque
- **Réseau** : Statistiques par interface réseau
- **Alertes** : Notifications automatiques pour les seuils critiques
- **Sauvegarde** : Export des métriques au format JSON
- **Temps réel** : Collecte périodique configurable

## 🛠️ Installation

### Prérequis

- Rust 1.70+
- Cargo

### Compilation

```bash
git clone https://github.com/Sam761200/Monitoring_Syst-me.git
cd Monitoring
cargo build --release
```

## 🚀 Utilisation

### Monitoring continu (mode par défaut)

```bash
cargo run
```

### Mode démonstration (snapshot unique)

```bash
cargo run demo
# ou
MONITOR_MODE=demo cargo run
```

### Monitoring court (30 secondes)

```bash
cargo run short
# ou
MONITOR_MODE=short cargo run
```

### Aide

```bash
cargo run help
```

## ⚙️ Configuration

### Variables d'environnement

| Variable           | Description                          | Défaut |
| ------------------ | ------------------------------------ | ------ |
| `MONITOR_MODE`     | Mode d'exécution (normal/demo/short) | normal |
| `MONITOR_INTERVAL` | Intervalle de collecte en secondes   | 5      |

### Exemples de configuration

```bash
# Intervalle de 10 secondes
MONITOR_INTERVAL=10 cargo run

# Mode démo
MONITOR_MODE=demo cargo run

# Monitoring court avec intervalle de 2 secondes
MONITOR_MODE=short MONITOR_INTERVAL=2 cargo run
```

## 📊 Format des métriques

Les métriques sont exportées au format JSON avec la structure suivante :

```json
{
  "timestamp": "2024-01-15T10:30:45.123Z",
  "cpu": {
    "usage_percent": 25.4,
    "cores": [
      { "core_id": 0, "usage_percent": 22.1 },
      { "core_id": 1, "usage_percent": 28.7 }
    ],
    "load_average": {
      "one_min": 1.2,
      "five_min": 1.1,
      "fifteen_min": 0.9
    }
  },
  "memory": {
    "total_bytes": 17179869184,
    "used_bytes": 8589934592,
    "available_bytes": 8589934592,
    "usage_percent": 50.0,
    "swap_total_bytes": 2147483648,
    "swap_used_bytes": 0
  },
  "disk": {
    "disks": [
      {
        "name": "C:",
        "mount_point": "C:\\",
        "total_space_bytes": 1000000000000,
        "used_space_bytes": 500000000000,
        "available_space_bytes": 500000000000,
        "usage_percent": 50.0,
        "file_system": "NTFS"
      }
    ]
  },
  "network": {
    "interfaces": [
      {
        "name": "eth0",
        "bytes_received": 1048576,
        "bytes_transmitted": 524288,
        "packets_received": 1000,
        "packets_transmitted": 800,
        "errors_on_received": 0,
        "errors_on_transmitted": 0
      }
    ],
    "total_bytes_received": 1048576,
    "total_bytes_transmitted": 524288
  }
}
```

## 🚨 Système d'alertes

Le système génère automatiquement des alertes pour :

- **CPU** : Usage > 80%
- **Mémoire** : Usage > 85%
- **Disque** : Usage > 90%

## 📁 Structure du projet

```
src/
├── main.rs          # Point d'entrée principal
├── metrics.rs       # Structures de données des métriques
├── collector.rs     # Collecteur de métriques système
├── monitor.rs       # Orchestrateur du monitoring
└── examples.rs      # Modes de démonstration
```

## 🎯 Roadmap

- [ ] Interface web en temps réel
- [ ] Base de données pour l'historique
- [ ] Notifications par email/webhook
- [ ] Monitoring de processus spécifiques
- [ ] Graphiques et tableaux de bord
- [ ] API REST
- [ ] Configuration par fichier YAML

## 🤝 Contribution

Les contributions sont les bienvenues ! N'hésitez pas à :

1. Fork le projet
2. Créer une branche feature
3. Commiter vos changements
4. Pousser vers la branche
5. Ouvrir une Pull Request

## 📜 Licence

Ce projet est sous licence MIT. Voir le fichier `LICENSE` pour plus de détails.

## 🙏 Remerciements

- [sysinfo](https://crates.io/crates/sysinfo) - Collecte d'informations système
- [tokio](https://crates.io/crates/tokio) - Runtime asynchrone
- [serde](https://crates.io/crates/serde) - Sérialisation/désérialisation
- [chrono](https://crates.io/crates/chrono) - Gestion des dates et heures
