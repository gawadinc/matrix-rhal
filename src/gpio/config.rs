/// Specifies if a pin is being used for Output or Input signal.
pub enum Mode {
    Output,
    Input,
}

/// Specifies which function a pin is using.
pub enum Function {
    Digital,
    Pwm,
}

// Specifies if a pin will output a signal.
pub enum State {
    On,
    Off,
}
