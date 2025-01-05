use toml::value::Datetime;

pub struct PingResult {
    pub id:Option<i64>,
    pub host: Option<String>,
    pub latency: Option<f64>,
    pub iface: Option<String>,
    pub timeout: Option<bool>,
    pub created_at: Option<Datetime>,
}
impl PingResult {
    pub fn new() -> Self {
        Self {
            id: None,
            host: None,
            latency: None,
            iface: None,
            timeout: Some(false),
            created_at: None,
        }
    }
    pub fn set_host(&mut self, host: String) {
        self.host = Some(host);
    }
    pub fn set_latency(&mut self, latency: f64) {
        println!("latency: {}", latency);
        self.latency = Some(latency);
    }
    pub fn set_iface(&mut self, iface: String) {
        self.iface = Some(iface);
    }
    pub fn set_timeout(&mut self, timeout: bool) {
        self.timeout = Some(timeout);
    }
    pub fn set_created_at(&mut self, created_at: Datetime) {
        self.created_at = Some(created_at);
    }
}