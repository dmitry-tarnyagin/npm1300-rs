use crate::common::Task;

/// Reset cause information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct ResetCause {
    pub ship_mode_exit: bool,
    pub boot_monitor_timeout: bool,
    pub watchdog_timeout: bool,
    pub long_press_timeout: bool,
    pub thermal_shutdown: bool,
    pub vsys_low: bool,
    pub sw_reset: bool,
}

/// Charger error reason information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct ChargerErrorReason {
    pub ntc_sensor_error: bool,
    pub vbat_sensor_error: bool,
    pub vbat_low: bool,
    pub vtrickle: bool,
    pub measurement_timeout: bool,
    pub charge_timeout: bool,
    pub trickle_timeout: bool,
}

/// Charger error sensor state information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct ChargerErrorSensor {
    pub sensor_ntc_cold: bool,
    pub sensor_ntc_cool: bool,
    pub sensor_ntc_warm: bool,
    pub sensor_ntc_hot: bool,
    pub sensor_vterm: bool,
    pub sensor_recharge: bool,
    pub sensor_vtrickle: bool,
    pub sensor_vbat_low: bool,
}

impl<I2c: embedded_hal_async::i2c::I2c, Delay: embedded_hal_async::delay::DelayNs>
    crate::NPM1300<I2c, Delay>
{
    /// Clear the error log registers
    ///
    /// This clears RSTCAUSE, CHARGERERRREASON, and CHARGERERRSENSOR registers.
    pub async fn clear_error_log(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .reset()
            .taskclrerrlog()
            .dispatch_async(|command| command.set_taskclrerrlog(Task::Trigger))
            .await
    }

    /// Enable the boot monitor timer
    ///
    /// The boot monitor timer is only cleared by Power-On Reset (POR).
    pub async fn enable_boot_monitor(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .reset()
            .scratch_0()
            .modify_async(|reg| reg.set_boottimeren(true))
            .await
    }

    /// Disable the boot monitor timer
    ///
    /// The boot monitor timer is only cleared by Power-On Reset (POR).
    pub async fn disable_boot_monitor(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .reset()
            .scratch_0()
            .modify_async(|reg| reg.set_boottimeren(false))
            .await
    }

    /// Check if the boot monitor timer is enabled
    pub async fn is_boot_timer_enabled(&mut self) -> Result<bool, crate::NPM1300Error<I2c::Error>> {
        Ok(self
            .device
            .reset()
            .scratch_0()
            .read_async()
            .await?
            .boottimeren())
    }

    /// Write to scratch register 0 (7 bits)
    ///
    /// This register is only cleared by Power-On Reset (POR).
    /// The value is masked to 7 bits (0x00-0x7F).
    pub async fn write_scratch0(
        &mut self,
        value: u8,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        let value = value & 0x7F; // Ensure only 7 bits are used
        self.device
            .reset()
            .scratch_0()
            .modify_async(|reg| reg.set_scratch_0(value))
            .await
    }

    /// Read from scratch register 0 (7 bits)
    ///
    /// This register is only cleared by Power-On Reset (POR).
    pub async fn read_scratch0(&mut self) -> Result<u8, crate::NPM1300Error<I2c::Error>> {
        Ok(self
            .device
            .reset()
            .scratch_0()
            .read_async()
            .await?
            .scratch_0())
    }

    /// Write to scratch register 1 (8 bits)
    ///
    /// This register is only cleared by Power-On Reset (POR).
    pub async fn write_scratch1(
        &mut self,
        value: u8,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .reset()
            .scratch_1()
            .write_async(|reg| reg.set_scratch_1(value))
            .await
    }

    /// Read from scratch register 1 (8 bits)
    ///
    /// This register is only cleared by Power-On Reset (POR).
    pub async fn read_scratch1(&mut self) -> Result<u8, crate::NPM1300Error<I2c::Error>> {
        Ok(self
            .device
            .reset()
            .scratch_1()
            .read_async()
            .await?
            .scratch_1())
    }

    /// Get the reset cause
    ///
    /// Returns information about what caused the last reset.
    /// This register is cleared with `clear_error_log()`.
    pub async fn get_reset_cause(&mut self) -> Result<ResetCause, crate::NPM1300Error<I2c::Error>> {
        let reg = self.device.reset().rstcause().read_async().await?;

        Ok(ResetCause {
            ship_mode_exit: reg.shipmodeexit(),
            boot_monitor_timeout: reg.bootmonitortimeout(),
            watchdog_timeout: reg.watchdogtimeout(),
            long_press_timeout: reg.longpresstimeout(),
            thermal_shutdown: reg.thermalshutdown(),
            vsys_low: reg.vsyslow(),
            sw_reset: reg.swreset(),
        })
    }

    /// Get the charger error reason
    ///
    /// Returns information about charger errors.
    /// This register is cleared with `clear_error_log()`.
    pub async fn get_charger_error_reason(
        &mut self,
    ) -> Result<ChargerErrorReason, crate::NPM1300Error<I2c::Error>> {
        let reg = self.device.reset().chargererrreason().read_async().await?;

        Ok(ChargerErrorReason {
            ntc_sensor_error: reg.ntcsensorerr(),
            vbat_sensor_error: reg.vbatsensorerr(),
            vbat_low: reg.vbatlow(),
            vtrickle: reg.vtrickle(),
            measurement_timeout: reg.meastimeout(),
            charge_timeout: reg.chargetimeout(),
            trickle_timeout: reg.trickletimeout(),
        })
    }

    /// Get the charger error sensor state
    ///
    /// Returns the sensor states when the charger error occurred.
    /// This register is cleared with `clear_error_log()`.
    pub async fn get_charger_error_sensor(
        &mut self,
    ) -> Result<ChargerErrorSensor, crate::NPM1300Error<I2c::Error>> {
        let reg = self.device.reset().chargererrsensor().read_async().await?;

        Ok(ChargerErrorSensor {
            sensor_ntc_cold: reg.sensorntccold(),
            sensor_ntc_cool: reg.sensorntccool(),
            sensor_ntc_warm: reg.sensorntcwarm(),
            sensor_ntc_hot: reg.sensorntchot(),
            sensor_vterm: reg.sensorvterm(),
            sensor_recharge: reg.sensorrecharge(),
            sensor_vtrickle: reg.sensorvtrickle(),
            sensor_vbat_low: reg.sensorvbatlow(),
        })
    }
}
