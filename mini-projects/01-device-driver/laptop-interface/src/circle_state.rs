pub struct CircleState {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

pub fn normalize_adc_value(adc_value: u16, step: u16) -> u16 {
    (adc_value + step / 2) / step * step
}

impl CircleState {
    pub fn new() -> Self {
        Self {
            x: 200.0,
            y: 200.0,
            radius: 50.0,
        }
    }

    pub fn update(&mut self, buttons: u8, adc_value: u16) {
        let step = 10.0; // Adjusted step size for smoother movement
        if buttons & 0b10000000 != 0 {
            self.y -= step;
            self.x += step;
        } // NE
        if buttons & 0b01000000 != 0 {
            self.y += step;
            self.x += step;
        } // SE
        if buttons & 0b00100000 != 0 {
            self.y += step;
            self.x -= step;
        } // SW
        if buttons & 0b00010000 != 0 {
            self.y -= step;
            self.x -= step;
        } // NW
        if buttons & 0b00001000 != 0 {
            self.x += step;
        } // Right
        if buttons & 0b00000100 != 0 {
            self.x -= step;
        } // Left
        if buttons & 0b00000010 != 0 {
            self.y += step;
        } // Bottom
        if buttons & 0b00000001 != 0 {
            self.y -= step;
        } // Top

        let rounded_adc = normalize_adc_value(adc_value, 30);
        self.radius = (rounded_adc as f32) / 10.0;
    }
}
