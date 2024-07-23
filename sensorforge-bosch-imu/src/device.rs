use crate::{
    interface::{I2cInterface, ReadData, SpiInterface, WriteData},
    types::{AccelerometerRange, GyroscopeRange, Sensor3DData, Sensor3DDataScaled, SensorType},
    AccelConfig, Error, GyroConfig, Imu, Register,
};
use embedded_hal::delay::DelayNs;

impl<I2C, D> Imu<I2cInterface<I2C>, D>
where
    D: DelayNs,
{
    /// Create a new BMI323 device instance
    ///
    /// # Arguments
    ///
    /// * `iface` - The communication interface
    /// * `delay` - A delay provider
    pub fn new_with_i2c(i2c: I2C, address: u8, delay: D) -> Self {
        Imu {
            iface: I2cInterface { i2c, address },
            delay,
            accel_range: AccelerometerRange::default(),
            gyro_range: GyroscopeRange::default(),
        }
    }
}

impl<SPI, D> Imu<SpiInterface<SPI>, D>
where
    D: DelayNs,
{
    /// Create a new BMI323 device instance
    ///
    /// # Arguments
    ///
    /// * `iface` - The communication interface
    /// * `delay` - A delay provider
    pub fn new_with_spi(spi: SPI, delay: D) -> Self {
        Imu {
            iface: SpiInterface { spi },
            delay,
            accel_range: AccelerometerRange::default(),
            gyro_range: GyroscopeRange::default(),
        }
    }
}

// TODO split this into one for bmi323 and one for bmi160
impl<DI, D, E> Imu<DI, D>
where
    DI: ReadData<Error = Error<E>> + WriteData<Error = Error<E>>,
    D: DelayNs,
{
    /// Initialize the device
    #[cfg(feature = "bmi323")]
    pub fn init(&mut self) -> Result<(), Error<E>> {
        #[cfg(feature = "bmi323")]
        self.write_register_16bit(Register::CMD, Register::CMD_SOFT_RESET)?;

        self.delay.delay_us(2000);

        //let mut reg_data = [0u8; 3];
        //reg_data[0] = 0x01; // sensor error conditins register
        // TODO: Modify the below in this function to match the bmi160
        let status = self.read_register(Register::ERR_REG)?;
        if (status & 0b0000_0001) != 0 {
            return Err(Error::InvalidDevice);
        }

        let result = self.read_register(Register::CHIPID)?;
        if result != Register::CHIP_ID {
            return Err(Error::InvalidDevice);
        }

        Ok(())
    }

    #[cfg(feature = "bmi160")]
    pub fn init(&mut self) -> Result<(), Error<E>> {
        #[cfg(feature = "bmi160")]
        self.write_register(Register::CMD, Register::CMD_SOFT_RESET)?;

        self.delay.delay_us(2000);

        let status = self.read_register(Register::ERR_REG)?;
        if status != 0 {
            return Err(Error::InvalidDevice);
        }

        let result = self.read_register(Register::CHIPID)?;
        if result != Register::CHIP_ID {
            return Err(Error::InvalidDevice);
        }

        Ok(())
    }

    /// Set the accelerometer configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The accelerometer configuration
    #[cfg(feature = "bmi323-accel")]
    pub fn set_accel_config(&mut self, config: AccelConfig) -> Result<(), Error<E>> {
        let reg_data: u16 = config.into();
        self.write_register_16bit(Register::ACC_CONF, reg_data)?;
        self.accel_range = config.range;

        // Wait for accelerometer data to be ready
        self.wait_for_data_ready(SensorType::Accelerometer)?;

        Ok(())
    }

    #[cfg(feature = "bmi160-accel")]
    pub fn set_accel_config(&mut self, config: AccelConfig) -> Result<(), Error<E>> {
        let (config_data, range_data) = config.into();

        // Write output data rate and bandwidth
        self.write_register(Register::BMI160_ACCEL_CONFIG_ADDR, config_data)?;

        // Write accel range
        self.write_register(Register::BMI160_ACCEL_RANGE_ADDR, range_data)?;

        self.accel_range = config.range;

        self.set_accel_power_mode(config)?;

        // Wait for accelerometer data to be ready
        self.wait_for_data_ready(SensorType::Accelerometer)?;

        Ok(())
    }

    #[cfg(feature = "bmi160-accel")]
    fn set_accel_power_mode(&mut self, config: AccelConfig) -> Result<(), Error<E>> {
        let mode = config.mode;
        self.write_register(Register::CMD, mode as u8)?;
        self.delay.delay_ms(5);
        Ok(())
    }

    /// Set the gyroscope configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The gyroscope configuration
    #[cfg(feature = "bmi323-gyro")]
    pub fn set_gyro_config(&mut self, config: GyroConfig) -> Result<(), Error<E>> {
        let reg_data: u16 = config.into();
        self.write_register_16bit(Register::GYR_CONF, reg_data)?;
        self.gyro_range = config.range;

        // Wait for gyroscope data to be ready
        self.wait_for_data_ready(SensorType::Gyroscope)?;

        Ok(())
    }

    #[cfg(feature = "bmi160-gyro")]
    pub fn set_gyro_config(&mut self, config: GyroConfig) -> Result<(), Error<E>> {
        let (config_data, range_data) = config.into();
        // Write output data rate and bandwidth
        self.write_register(Register::BMI160_GYRO_CONFIG_ADDR, config_data)?;

        // Write gyro range
        self.write_register(Register::BMI160_GYRO_RANGE_ADDR, range_data)?;

        self.gyro_range = config.range;

        // Set gyro power mode
        self.set_gyro_power_mode(config)?;

        // Wait for gyroscope data to be ready
        self.wait_for_data_ready(SensorType::Gyroscope)?;

        Ok(())
    }

    #[cfg(feature = "bmi160-gyro")]
    fn set_gyro_power_mode(&mut self, config: GyroConfig) -> Result<(), Error<E>> {
        let mode = config.mode;
        self.write_register(Register::CMD, mode as u8)?;
        self.delay.delay_ms(5);
        Ok(())
    }

    fn read_sensor_data(&mut self, sensor_type: SensorType) -> Result<Sensor3DData, Error<E>> {
        let (base_reg, data_size) = match sensor_type {
            SensorType::Accelerometer => (Register::ACC_DATA_X, 21),
            SensorType::Gyroscope => (Register::GYR_DATA_X, 15),
        };

        let mut data = [0u8; 21]; // Use the larger size
        data[0] = base_reg;
        let sensor_data = self.read_data(&mut data[0..data_size])?;

        Ok(Sensor3DData {
            x: i16::from_le_bytes([sensor_data[0], sensor_data[1]]),
            y: i16::from_le_bytes([sensor_data[2], sensor_data[3]]),
            z: i16::from_le_bytes([sensor_data[4], sensor_data[5]]),
        })
    }

    /// Read the LSB for the accelerometer
    pub fn read_accel_data(&mut self) -> Result<Sensor3DData, Error<E>> {
        self.read_sensor_data(SensorType::Accelerometer)
    }

    /// Read the LSB for the gyroscope
    pub fn read_gyro_data(&mut self) -> Result<Sensor3DData, Error<E>> {
        self.read_sensor_data(SensorType::Gyroscope)
    }

    /// Read the LSB for the accelerometer and return the scaled value as mps2
    pub fn read_accel_data_scaled(&mut self) -> Result<Sensor3DDataScaled, Error<E>> {
        let raw_data = self.read_accel_data()?;
        Ok(raw_data.to_mps2(self.accel_range.to_g())) // Assuming 16-bit width
    }

    /// Read the LSB for the gyroscope and return the scaled value as dps
    pub fn read_gyro_data_scaled(&mut self) -> Result<Sensor3DDataScaled, Error<E>> {
        let raw_data = self.read_gyro_data()?;
        Ok(raw_data.to_dps(self.gyro_range.to_dps())) // Assuming 16-bit width
    }

    #[cfg(feature = "bmi323")]
    fn write_register_16bit(&mut self, reg: u8, value: u16) -> Result<(), Error<E>> {
        let bytes = value.to_le_bytes();
        self.iface.write_data(&[reg, bytes[0], bytes[1]])
    }

    #[cfg(feature = "bmi160")]
    fn write_register(&mut self, reg: u8, value: u8) -> Result<(), Error<E>> {
        self.iface.write_data(&[reg, value])
    }

    fn read_register(&mut self, reg: u8) -> Result<u8, Error<E>> {
        self.iface.read_register(reg)
    }

    fn read_data<'a>(&mut self, data: &'a mut [u8]) -> Result<&'a [u8], Error<E>> {
        self.iface.read_data(data)
    }

    fn wait_for_data_ready(&mut self, sensor_type: SensorType) -> Result<(), Error<E>> {
        const MAX_RETRIES: u8 = 100;
        let mut retries = 0;

        while !self.is_data_ready(sensor_type)? {
            if retries >= MAX_RETRIES {
                return Err(Error::Timeout);
            }
            self.delay.delay_ms(1);
            retries += 1;
        }

        Ok(())
    }

    fn is_data_ready(&mut self, sensor_type: SensorType) -> Result<bool, Error<E>> {
        let status = self.read_register(Register::STATUS)?;
        match sensor_type {
            SensorType::Accelerometer => Ok((status & 0b1000_0000) != 0), // Check bit 7 (drdy_acc)
            SensorType::Gyroscope => Ok((status & 0b0100_0000) != 0),     // Check bit 6 (drdy_gyr)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_sensor3d_data(data: &[u8]) -> Sensor3DData {
        Sensor3DData {
            x: i16::from_le_bytes([data[0], data[1]]),
            y: i16::from_le_bytes([data[2], data[3]]),
            z: i16::from_le_bytes([data[4], data[5]]),
        }
    }

    mod sensor3d_data {
        use super::*;

        #[test]
        fn can_decode_positive_array() {
            let result = get_sensor3d_data(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06]);
            assert_eq!(
                result,
                Sensor3DData {
                    x: 0x0201,
                    y: 0x0403,
                    z: 0x0605
                }
            );
        }

        #[test]
        fn can_decode_negative_array() {
            let result = get_sensor3d_data(&[0x0B, 0x86, 0x0B, 0x86, 0x0B, 0x86]);
            assert_eq!(
                result,
                Sensor3DData {
                    x: -31221,
                    y: -31221,
                    z: -31221
                }
            );
        }
    }
}
