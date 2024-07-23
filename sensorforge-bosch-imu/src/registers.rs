/// BMI323 register addresses and constant values
pub struct Register;
#[cfg(feature = "bmi323")]
impl Register {
    /// Chip ID register address
    pub const CHIPID: u8 = 0x00;
    /// Error register address
    pub const ERR_REG: u8 = 0x02;
    /// Status register address
    pub const STATUS: u8 = 0x02;
    /// Accelerometer X-axis data register address
    pub const ACC_DATA_X: u8 = 0x03;
    /// Gyroscope X-axis data register address
    pub const GYR_DATA_X: u8 = 0x06;
    /// Accelerometer configuration register address
    pub const ACC_CONF: u8 = 0x20;
    /// Gyroscope configuration register address
    pub const GYR_CONF: u8 = 0x21;
    /// Command register address
    pub const CMD: u8 = 0x7E;
    /// Expected chip ID for BMI323
    pub const CHIP_ID: u8 = 0x43;
    /// Soft reset command value
    pub const CMD_SOFT_RESET: u16 = 0xDEAF;
}

#[cfg(feature = "bmi160")]
impl Register {
    pub const CHIPID: u8 = 0x00;
    pub const CMD: u8 = 0x7E;
    pub const CHIP_ID: u8 = 0xD1;
    pub const CMD_SOFT_RESET: u8 = 0xB6;
    pub const ERR_REG: u8 = 0x01;
    pub const GYR_DATA_X: u8 = 0x0C;
    pub const ACC_DATA_X: u8 = 0x12;
    pub const STATUS: u8 = 0x1B;
    #[cfg(feature = "bmi160-accel")]
    pub const BMI160_ACCEL_CONFIG_ADDR: u8 = 0x40;
    #[cfg(feature = "bmi160-accel")]
    pub const BMI160_ACCEL_RANGE_ADDR: u8 = 0x41;
    #[cfg(feature = "bmi160-gyro")]
    pub const BMI160_GYRO_CONFIG_ADDR: u8 = 0x42;
    #[cfg(feature = "bmi160-gyro")]
    pub const BMI160_GYRO_RANGE_ADDR: u8 = 0x43;
}
