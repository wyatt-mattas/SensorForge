/// BMI323 register addresses and constant values
pub struct Register;
impl Register {
    /// Chip ID register address
    pub const CHIPID: u8 = 0x00;
    /// Error register address
    #[cfg(feature = "bmi160")]
    pub const ERR_REG: u8 = 0x01;
    #[cfg(feature = "bmi323")]
    pub const ERR_REG: u8 = 0x02;
    /// Status register address
    pub const STATUS: u8 = 0x02;
    /// Accelerometer X-axis data register address
    #[cfg(feature = "bmi323")]
    pub const ACC_DATA_X: u8 = 0x03;
    #[cfg(feature = "bmi160")]
    pub const ACC_DATA_X: u8 = 0x12;
    /// Gyroscope X-axis data register address
    #[cfg(feature = "bmi323")]
    pub const GYR_DATA_X: u8 = 0x06;
    #[cfg(feature = "bmi160")]
    pub const GYR_DATA_X: u8 = 0x0C;
    /// Accelerometer configuration register address
    pub const ACC_CONF: u8 = 0x20;
    /// Gyroscope configuration register address
    pub const GYR_CONF: u8 = 0x21;
    /// Command register address
    pub const CMD: u8 = 0x7E;
    /// Expected chip ID for BMI323
    pub const BMI323_CHIP_ID: u8 = 0x43;
    /// Soft reset command value
    #[cfg(feature = "bmi323")]
    pub const CMD_SOFT_RESET: u16 = 0xDEAF;
    #[cfg(feature = "bmi160")]
    pub const CMD_SOFT_RESET: u8 = 0xB6;
}
