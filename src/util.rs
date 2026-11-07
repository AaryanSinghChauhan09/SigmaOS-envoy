//! envoy model — cloud-native proxy and service mesh data plane.

#[derive(Debug, Clone)]
pub struct Listener {
    pub name: String,
    pub address: String,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct Cluster {
    pub name: String,
    pub type_: String,
    pub endpoints: Vec<String>,
    pub lb_policy: String,
}

#[derive(Debug, Clone)]
pub struct Route {
    pub name: String,
    pub domains: Vec<String>,
    pub cluster: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub listeners: Vec<Listener>,
    pub clusters: Vec<Cluster>,
    pub routes: Vec<Route>,
}

impl Default for Model {
    fn default() -> Self { Self { listeners: vec![], clusters: vec![], routes: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_listener(&mut self, l: Listener) { self.listeners.push(l); }
    pub fn add_cluster(&mut self, c: Cluster) { self.clusters.push(c); }
    pub fn add_route(&mut self, r: Route) { self.routes.push(r); }

    pub fn stats(&self) -> (usize, usize, usize) {
        (self.listeners.len(), self.clusters.len(), self.routes.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_all() {
        let mut e = Model::new();
        e.add_listener(Listener { name: "http".into(), address: "0.0.0.0".into(), port: 8080 });
        e.add_cluster(Cluster { name: "web".into(), type_: "STATIC".into(), endpoints: vec!["10.0.0.1:80".into()], lb_policy: "ROUND_ROBIN".into() });
        e.add_route(Route { name: "r1".into(), domains: vec!["*".into()], cluster: "web".into() });
        assert_eq!(e.stats(), (1, 1, 1));
    }
}
