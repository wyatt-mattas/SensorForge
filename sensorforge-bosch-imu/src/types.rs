use core::fmt::Debug;

/// Possible errors that can occur when interacting with the BMI323
#[derive(Debug)]
pub enum Error<E> {
    /// Communication error
    Comm(E),
    /// Invalid device (wrong chip ID)
    InvalidDevice,
    /// Invalid configuration
    InvalidConfig,
    /// Timeout error
    Timeout,
}

/// Accelerometer power modes
#[cfg(feature = "bmi323")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccelerometerPowerMode {
    /// Accelerometer disabled
    Disable = 0x00,
    /// Low power mode
    LowPower = 0x03,
    /// Normal power mode
    Normal = 0x04,
    /// High performance mode
    HighPerf = 0x07,
}

#[cfg(feature = "bmi160")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccelerometerPowerMode {
    Suspend = 0x10,
    LowPower = 0x12,
    Normal = 0x11,
}

#[cfg(feature = "bmi323")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccelerometerRange {
    G2 = 0x00,
    G4 = 0x01,
    G8 = 0x02,
    G16 = 0x03,
}

#[cfg(feature = "bmi160")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AccelerometerRange {
    G2 = 0x03,
    G4 = 0x05,
    G8 = 0x08,
    G16 = 0x0C,
}

impl AccelerometerRange {
    pub fn to_g(self) -> f32 {
        match self {
            AccelerometerRange::G2 => 2.0,
            AccelerometerRange::G4 => 4.0,
            AccelerometerRange::G8 => 8.0,
            AccelerometerRange::G16 => 16.0,
        }
    }
}

impl Default for AccelerometerRange {
    fn default() -> Self {
        AccelerometerRange::G8
    }
}

/// Gyroscope power mode
#[cfg(feature = "bmi323")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GyroscopePowerMode {
    /// Gyroscope disabled
    Disable = 0x00,
    /// Supend mode
    Suspend = 0x01,
    /// Low power mode
    LowPower = 0x03,
    /// Normal power mode
    Normal = 0x04,
    /// High perfomance mode
    HighPerf = 0x07,
}

/// Gyroscope power mode
#[cfg(feature = "bmi160")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GyroscopePowerMode {
    Suspend = 0x14,
    Normal = 0x15,
    FastStartup = 0x17,
}

/// Gyroscope measurement ranges
#[cfg(feature = "bmi323")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GyroscopeRange {
    /// ±125 degrees per second
    DPS125 = 0x00,
    /// ±250 degrees per second
    DPS250 = 0x01,
    /// ±500 degrees per second
    DPS500 = 0x02,
    /// ±1000 degrees per second
    DPS1000 = 0x03,
    /// ±2000 degrees per second
    DPS2000 = 0x04,
}

/// Gyroscope measurement ranges
#[cfg(feature = "bmi160")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GyroscopeRange {
    /// ±125 degrees per second
    DPS125 = 0x04,
    /// ±250 degrees per second
    DPS250 = 0x03,
    /// ±500 degrees per second
    DPS500 = 0x02,
    /// ±1000 degrees per second
    DPS1000 = 0x01,
    /// ±2000 degrees per second
    DPS2000 = 0x00,
}

impl GyroscopeRange {
    pub fn to_dps(self) -> f32 {
        match self {
            GyroscopeRange::DPS125 => 125.0,
            GyroscopeRange::DPS250 => 250.0,
            GyroscopeRange::DPS500 => 500.0,
            GyroscopeRange::DPS1000 => 1000.0,
            GyroscopeRange::DPS2000 => 2000.0,
        }
    }
}

impl Default for GyroscopeRange {
    fn default() -> Self {
        GyroscopeRange::DPS2000
    }
}

/// 3D sensor data (raw values)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sensor3DData {
    /// X-axis value
    pub x: i16,
    /// Y-axis value
    pub y: i16,
    /// Z-axis value
    pub z: i16,
}

/// Scaled 3D sensor data
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sensor3DDataScaled {
    /// X-axis scaled value
    pub x: f32,
    /// Y-axis scaled value
    pub y: f32,
    /// Z-axis scaled value
    pub z: f32,
}

/// Output data rates for sensors
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputDataRate {
    /// 0.78 Hz
    #[cfg(not(feature = "bmi160-gyro"))]
    Odr0_78hz = 0x01,
    /// 1.56 Hz
    #[cfg(not(feature = "bmi160-gyro"))]
    Odr1_56hz = 0x02,
    /// 3.125 Hz
    #[cfg(not(feature = "bmi160-gyro"))]
    Odr3_125hz = 0x03,
    /// 6.25 Hz
    #[cfg(not(feature = "bmi160-gyro"))]
    Odr6_25hz = 0x04,
    /// 12.5 Hz
    #[cfg(not(feature = "bmi160-gyro"))]
    Odr12_5hz = 0x05,
    /// 25 Hz
    Odr25hz = 0x06,
    /// 50 Hz
    Odr50hz = 0x07,
    /// 100 Hz
    Odr100hz = 0x08,
    /// 200 Hz
    Odr200hz = 0x09,
    /// 400 Hz
    Odr400hz = 0x0A,
    /// 800 Hz
    Odr800hz = 0x0B,
    /// 1600 Hz
    Odr1600hz = 0x0C,
    /// 3200 Hz
    #[cfg(feature = "bmi323")]
    Odr3200hz = 0x0D,
    /// 6400 Hz
    #[cfg(feature = "bmi323")]
    Odr6400hz = 0x0E,
}

/// Number of samples to average
#[cfg(feature = "bmi323")]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AverageNum {
    /// No averaging
    Avg1 = 0x00,
    /// Average 2 samples
    Avg2 = 0x01,
    /// Average 4 samples
    Avg4 = 0x02,
    /// Average 8 samples
    Avg8 = 0x03,
    /// Average 16 samples
    Avg16 = 0x04,
    /// Average 32 samples
    Avg32 = 0x05,
    /// Average 64 samples
    Avg64 = 0x06,
}

/// Sensor bandwidth settings
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bandwidth {
    /// Half of the output data rate
    OdrHalf = 0,
    /// Quarter of the output data rate
    OdrQuarter = 1,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SensorType {
    Accelerometer,
    Gyroscope,
}
