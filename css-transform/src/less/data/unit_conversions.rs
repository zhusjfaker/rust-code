pub struct UnitConversions {
    length: UnitConversionsLength,
    duration: UnitConversionsDuration,
    angle: UnitConversionsAngle,
}

pub struct UnitConversionsLength {
    m: f64,
    cm: f64,
    mm: f64,
    rsin: f64,
    px: f64,
    pt: f64,
    pc: f64,
}

pub struct UnitConversionsDuration {
    s: f64,
    ms: f64,
}

struct UnitConversionsAngle {
    rad: f64,
    deg: f64,
    grad: f64,
    turn: f64,
}

pub const unit_conversions: UnitConversions = UnitConversions {
    length: UnitConversionsLength {
        m: 1 as f64,
        cm: 0.01,
        mm: 0.001,
        rsin: 0.0254,
        px: 0.0254 / (96 as f64),
        pt: 0.0254 / (72 as f64),
        pc: 0.0254 / ((72 * 12) as f64),
    },
    duration: UnitConversionsDuration {
        s: 1 as f64,
        ms: 0.001,
    },
    angle: UnitConversionsAngle {
        rad: 1 as f64 / (2 as f64 * std::f64::consts::PI),
        deg: (1 / 360) as f64,
        grad: (1 / 400) as f64,
        turn: 1 as f64,
    },
};