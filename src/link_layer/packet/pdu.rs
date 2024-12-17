mod advertising;
mod data;
mod isochronous;

pub enum Address {
    Public([u8; 6]),
    Random([u8; 6])
}

impl Address {
    pub fn bytes(&self) -> &[u8; 6] {
        match self {
            Self::Public(bytes) => bytes,
            Self::Random(bytes) => bytes,
        }
    }
    
}

