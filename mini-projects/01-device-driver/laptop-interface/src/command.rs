#[derive(Debug)]
pub enum Command {
    Init,
    SetReadyLed,
    SetSetLed,
    SetGoLed,
    SetAllLeds,
    Start,
    Stop,
    Reset,
}

impl Command {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Command::Init => b"init controller\n",
            Command::SetReadyLed => b"set ready led\n",
            Command::SetSetLed => b"set set led\n",
            Command::SetGoLed => b"set go led\n",
            Command::SetAllLeds => b"set all leds\n",
            Command::Start => b"start controller\n",
            Command::Stop => b"stop controller\n",
            Command::Reset => b"reset\n",
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Command::Init => "Init".to_string(),
            Command::SetReadyLed => "SetReadyLed".to_string(),
            Command::SetSetLed => "SetSetLed".to_string(),
            Command::SetGoLed => "SetGoLed".to_string(),
            Command::SetAllLeds => "SetAllLeds".to_string(),
            Command::Start => "Start".to_string(),
            Command::Stop => "Stop".to_string(),
            Command::Reset => "Reset".to_string(),
        }
    }
    pub fn from_string(s: &str) -> Option<Command> {
        match s {
            "Init" => Some(Command::Init),
            "SetReadyLed" => Some(Command::SetReadyLed),
            "SetSetLed" => Some(Command::SetSetLed),
            "SetGoLed" => Some(Command::SetGoLed),
            "SetAllLeds" => Some(Command::SetAllLeds),
            "Start" => Some(Command::Start),
            "Stop" => Some(Command::Stop),
            "Reset" => Some(Command::Reset),
            _ => None,
        }
    }
}
