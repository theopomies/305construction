#[derive(Debug)]
pub struct Task {
    pub duration: u16,
    pub deps: Vec<String>,
    pub dependants: Vec<String>,
    pub earlier_start: Option<u16>,
    pub latest_start: Option<u16>,
}

impl Task {
    pub fn new(duration: u16, deps: &[&str]) -> Self {
        Self {
            duration,
            deps: deps.iter().cloned().map(|s| s.to_owned()).collect(),
            dependants: vec![],
            earlier_start: None,
            latest_start: None,
        }
    }
}
