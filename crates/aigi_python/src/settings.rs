use uv::settings::NetworkSettings;
use uv_client::Connectivity;

pub fn uv_network_settings() -> NetworkSettings {
    NetworkSettings {
        connectivity: Connectivity::Online,
        native_tls: false,
        allow_insecure_host: Vec::new(),
    }
}
