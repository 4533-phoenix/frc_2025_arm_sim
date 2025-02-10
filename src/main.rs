use std::env;

mod simulations;

enum SimulationType {
    Main,
    Grid,
}

impl SimulationType {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "main" => Some(Self::Main),
            "grid" => Some(Self::Grid),
            _ => None,
        }
    }
}

fn main() {
    let sim_type = env::args()
        .nth(1)
        .and_then(|arg| SimulationType::from_str(&arg))
        .unwrap_or(SimulationType::Main);

    let mut app = match sim_type {
        SimulationType::Main => simulations::main::run(),
        SimulationType::Grid => simulations::grid::run(),
    };

    app.run();
}
