mod diagnostic;
pub mod diagnostics;
pub use diagnostic::*;

use crate::app::{AppPlugin, AppBuilder};
use std::time::Duration;
use diagnostics::{print_diagnostics_system, frame_time_diagnostic_system};

pub struct DiagnosticsPlugin {
    pub print_wait_duration: Duration,
    pub print_diagnostics: bool,
    pub add_defaults: bool,
}

impl Default for DiagnosticsPlugin {
    fn default() -> Self {
        DiagnosticsPlugin {
            print_wait_duration: Duration::from_secs_f64(1.0),
            print_diagnostics: false,
            add_defaults: true,
        }
    }
}

impl AppPlugin for DiagnosticsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Diagnostics::default());
        if self.add_defaults {
            let frame_time_diagnostic_system =
                { frame_time_diagnostic_system(app.resources_mut(), 10) };
            app.add_system(frame_time_diagnostic_system);
        }

        if self.print_diagnostics {
            app.add_system(print_diagnostics_system(self.print_wait_duration));
        }
    }
}
