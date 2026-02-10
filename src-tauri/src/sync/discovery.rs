use mdns_sd::{Receiver, ServiceDaemon, ServiceEvent, ServiceInfo};
use std::collections::HashMap;
use std::time::Duration;

const SERVICE_TYPE: &str = "_cacao._tcp.local.";

pub struct PeerInfo {
    pub ip: String,
    pub port: u16,
    pub device_name: String,
    #[allow(dead_code)]
    pub family_uuid: String,
}

pub struct MdnsBroadcaster {
    daemon: ServiceDaemon,
    fullname: String,
}

impl MdnsBroadcaster {
    pub fn start(family_uuid: &str, device_name: &str, port: u16) -> Result<Self, String> {
        let daemon = ServiceDaemon::new().map_err(|e| e.to_string())?;

        let hostname = format!("cacao-{}.local.", &family_uuid[..8.min(family_uuid.len())]);
        let instance_name = format!("cacao-{}", device_name);

        let mut properties = HashMap::new();
        properties.insert("family_uuid".to_string(), family_uuid.to_string());
        properties.insert("device_name".to_string(), device_name.to_string());

        let service = ServiceInfo::new(
            SERVICE_TYPE,
            &instance_name,
            &hostname,
            "",
            port,
            properties,
        )
        .map_err(|e| e.to_string())?;

        let fullname = service.get_fullname().to_string();
        daemon.register(service).map_err(|e| e.to_string())?;

        log::info!("mDNS broadcast started: {} on port {}", fullname, port);

        Ok(Self { daemon, fullname })
    }

    pub fn stop(&self) {
        if let Err(e) = self.daemon.unregister(&self.fullname) {
            log::warn!("Failed to unregister mDNS service: {}", e);
        }
        if let Err(e) = self.daemon.shutdown() {
            log::warn!("Failed to shutdown mDNS daemon: {}", e);
        }
    }
}

pub struct MdnsDiscovery;

impl MdnsDiscovery {
    pub fn search(family_uuid: &str, timeout_secs: u64) -> Result<Vec<PeerInfo>, String> {
        let daemon = ServiceDaemon::new().map_err(|e| e.to_string())?;
        let receiver: Receiver<ServiceEvent> =
            daemon.browse(SERVICE_TYPE).map_err(|e| e.to_string())?;

        let mut peers = Vec::new();
        let deadline = std::time::Instant::now() + Duration::from_secs(timeout_secs);

        while std::time::Instant::now() < deadline {
            match receiver.recv_timeout(Duration::from_millis(500)) {
                Ok(ServiceEvent::ServiceResolved(info)) => {
                    let props = info.get_properties();
                    let found_family = props
                        .get_property_val_str("family_uuid")
                        .unwrap_or_default();

                    if found_family == family_uuid {
                        let device_name = props
                            .get_property_val_str("device_name")
                            .unwrap_or_default()
                            .to_string();

                        if let Some(addr) = info.get_addresses().iter().next() {
                            peers.push(PeerInfo {
                                ip: addr.to_string(),
                                port: info.get_port(),
                                device_name,
                                family_uuid: found_family.to_string(),
                            });
                            break; // Found our peer
                        }
                    }
                }
                Ok(_) => {}  // Ignore other events
                Err(_) => {} // Timeout on recv, continue loop
            }
        }

        daemon.shutdown().ok();
        Ok(peers)
    }
}
