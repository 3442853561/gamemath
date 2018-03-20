pub struct Factor(f32);

impl Factor {
    pub fn new(new_factor: f32) -> Self {
        Factor(new_factor - (new_factor as u32) as f32)
    }
    pub fn get(&self) -> f32 {
        self.0
    }
    fn check(&mut self) {
        self.0 = 0.0_f32.max(1.0_f32.min(self.0));
    }
}

impl From<f32> for Factor {
    fn from(new_factor: f32) -> Self {
        Factor(0.0_f32.max(1.0_f32.min(new_factor)))
    }
}

impl From<f64> for Factor {
    fn from(new_factor: f64) -> Self {
        Self::from(new_factor as f32)
    }
}

impl Into<f32> for Factor {
    fn into(self) -> f32 {
        self.0
    }
}

impl Into<f64> for Factor {
    fn into(self) -> f64 {
        self.0 as f64
    }
}

pub trait IntoFactor {
    fn into(self) -> Factor;
}

impl IntoFactor for f32 {
    fn into(self) -> Factor {
        Factor::from(self)
    }
}

impl IntoFactor for f64 {
    fn into(self) -> Factor {
        Factor::from(self)
    }
}
