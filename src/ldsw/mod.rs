use crate::common::Task;

mod types;

// Re-export everything in types.rs
pub use types::*;

impl<I2c: embedded_hal_async::i2c::I2c, Delay: embedded_hal_async::delay::DelayNs>
    crate::NPM1300<I2c, Delay>
{
    /// Enable or disable a LDSW regulator
    ///
    /// # Arguments
    ///
    /// * `ldsw_index` - Index of the LDSW regulator (0 for LDSW1, 1 for LDSW2)
    /// * `enable` - true to enable the regulator, false to disable it
    async fn control_ldsw_power(
        &mut self,
        ldsw_index: u8,
        enable: bool,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        if enable {
            self.device
                .ldsw()
                .ldswset(ldsw_index.into())
                .dispatch_async(|command| command.set_taskldswset(Task::Trigger))
                .await
        } else {
            self.device
                .ldsw()
                .ldswclr(ldsw_index.into())
                .dispatch_async(|command| command.set_taskldswclr(Task::Trigger))
                .await
        }
    }

    /// Enable LDSW1
    pub async fn enable_ldsw1(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.control_ldsw_power(0, true).await
    }

    /// Disable LDSW1
    pub async fn disable_ldsw1(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.control_ldsw_power(0, false).await
    }

    /// Enable LDSW2
    pub async fn enable_ldsw2(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.control_ldsw_power(1, true).await
    }

    /// Disable LDSW2
    pub async fn disable_ldsw2(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.control_ldsw_power(1, false).await
    }

    /// Get LDSW status register (raw field-set type)
    pub async fn get_ldsw_status(&mut self) -> Result<LdswStatus, crate::NPM1300Error<I2c::Error>> {
        let status = self.device.ldsw().ldswstatus().read_async().await?;
        Ok(LdswStatus {
            ldsw1_load_switch: status.ldsw_1_pwrupldsw().unwrap(),
            ldsw1_ldo: status.ldsw_1_pwrupldo().unwrap(),
            ldsw2_load_switch: status.ldsw_2_pwrupldsw().unwrap(),
            ldsw2_ldo: status.ldsw_2_pwrupldo().unwrap(),
            ldsw_enable: status.ldswenable().unwrap(),
        })
    }

    /// Configure which GPIO controls LDSW1 and whether to invert the sense.
    pub async fn configure_ldsw1_gpio_control(
        &mut self,
        gpio: crate::gpios::Gpio,
        polarity: crate::gpios::GpioPolarity,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .ldsw()
            .ldsw_1_gpisel()
            .write_async(|reg| {
                reg.set_ldsw_1_gpisel(gpio);
                reg.set_ldsw_1_gpiinv(polarity);
            })
            .await
    }

    /// Configure which GPIO controls LDSW2 and whether to invert the sense.
    pub async fn configure_ldsw2_gpio_control(
        &mut self,
        gpio: crate::gpios::Gpio,
        polarity: crate::gpios::GpioPolarity,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .ldsw()
            .ldsw_2_gpisel()
            .write_async(|reg| {
                reg.set_ldsw_2_gpisel(gpio);
                reg.set_ldsw_2_gpiinv(polarity);
            })
            .await
    }

    /// Configure LDSW1 soft-start: enable/disable and select current level.
    pub async fn configure_ldsw1_softstart(
        &mut self,
        disable_softstart: bool,
        level: SoftStartLevel,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .ldsw()
            .ldswconfig()
            .modify_async(|reg| {
                reg.set_ldsw_1_softstartdisable(if disable_softstart {
                    SoftStartDisable::Disabled
                } else {
                    SoftStartDisable::Enabled
                });
                reg.set_ldsw_1_softstartsel(level);
            })
            .await
    }

    /// Configure LDSW2 soft-start: enable/disable and select current level.
    pub async fn configure_ldsw2_softstart(
        &mut self,
        disable_softstart: bool,
        level: SoftStartLevel,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .ldsw()
            .ldswconfig()
            .modify_async(|reg| {
                reg.set_ldsw_2_softstartdisable(if disable_softstart {
                    SoftStartDisable::Disabled
                } else {
                    SoftStartDisable::Enabled
                });
                reg.set_ldsw_2_softstartsel(level);
            })
            .await
    }

    /// Enable/disable LDSW1 active discharge.
    pub async fn set_ldsw1_active_discharge(
        &mut self,
        enable: bool,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .ldsw()
            .ldswconfig()
            .modify_async(|reg| {
                reg.set_ldsw_1_activedischarge(if enable {
                    ActiveDischarge::Enabled
                } else {
                    ActiveDischarge::Disabled
                })
            })
            .await
    }

    /// Enable/disable LDSW2 active discharge.
    pub async fn set_ldsw2_active_discharge(
        &mut self,
        enable: bool,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .ldsw()
            .ldswconfig()
            .modify_async(|reg| {
                reg.set_ldsw_2_activedischarge(if enable {
                    ActiveDischarge::Enabled
                } else {
                    ActiveDischarge::Disabled
                })
            })
            .await
    }

    /// Select LDSW1 mode: Load switch or LDO.
    pub async fn set_ldsw1_mode(
        &mut self,
        mode: LdswMode,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .ldsw()
            .ldsw_1_ldosel()
            .write_async(|reg| reg.set_ldsw_1_ldosel(mode))
            .await
    }

    /// Select LDSW2 mode: Load switch or LDO.
    pub async fn set_ldsw2_mode(
        &mut self,
        mode: LdswMode,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .ldsw()
            .ldsw_2_ldosel()
            .write_async(|reg| reg.set_ldsw_2_ldosel(mode))
            .await
    }

    /// Set LDSW1 LDO output voltage (only meaningful when LDSW1 is in LDO mode).
    pub async fn set_ldsw1_ldo_voltage(
        &mut self,
        voltage: LdoVoltage,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .ldsw()
            .ldsw_1_voutsel()
            .write_async(|reg| reg.set_ldsw_1_voutsel(u8::from(voltage)))
            .await
    }

    /// Set LDSW2 LDO output voltage (only meaningful when LDSW2 is in LDO mode).
    pub async fn set_ldsw2_ldo_voltage(
        &mut self,
        voltage: LdoVoltage,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .ldsw()
            .ldsw_2_voutsel()
            .write_async(|reg| reg.set_ldsw_2_voutsel(u8::from(voltage)))
            .await
    }

    /// Read LDSW1 LDO output voltage selection.
    ///
    /// Returns `None` if the raw register field is outside the defined 1.0–3.3V (0–23) range.
    pub async fn get_ldsw1_ldo_voltage(
        &mut self,
    ) -> Result<Option<LdoVoltage>, crate::NPM1300Error<I2c::Error>> {
        let raw = self
            .device
            .ldsw()
            .ldsw_1_voutsel()
            .read_async()
            .await?
            .ldsw_1_voutsel();
        Ok(LdoVoltage::try_from(raw).ok())
    }

    /// Read LDSW2 LDO output voltage selection.
    ///
    /// Returns `None` if the raw register field is outside the defined 1.0–3.3V (0–23) range.
    pub async fn get_ldsw2_ldo_voltage(
        &mut self,
    ) -> Result<Option<LdoVoltage>, crate::NPM1300Error<I2c::Error>> {
        let raw = self
            .device
            .ldsw()
            .ldsw_2_voutsel()
            .read_async()
            .await?
            .ldsw_2_voutsel();
        Ok(LdoVoltage::try_from(raw).ok())
    }
}
