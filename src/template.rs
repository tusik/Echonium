use askama::Template;
use crate::config::{PingConfig, PingTarget};

#[derive(Template)]
#[template(path = "ping.html")]
pub struct PingTemplate{
    pub source: Option<PingConfig>,
}
impl PingTemplate {
    pub fn new() -> Self {
        Self {
            source: None,
        }
    }
    pub fn set_source(&mut self, source: PingConfig) {
        self.source = Some(source);
    }
}