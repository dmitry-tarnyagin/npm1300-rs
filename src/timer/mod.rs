use crate::{common::Task, Bootmonitoractive, Slowdomainconfigured, Timermodesel, Timerprescaler};

impl<I2c: embedded_hal_async::i2c::I2c, Delay: embedded_hal_async::delay::DelayNs>
    crate::NPM1300<I2c, Delay>
{
    /// Start the timer
    ///
    /// This triggers the timer to start counting based on the configured mode.
    pub async fn start_timer(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .timer()
            .tasktimerset()
            .dispatch_async(|command| command.set_tasktimeren(Task::Trigger))
            .await
    }

    /// Stop the timer
    ///
    /// This stops the timer from counting.
    pub async fn stop_timer(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .timer()
            .tasktimerclr()
            .dispatch_async(|command| command.set_tasktimerdis(Task::Trigger))
            .await
    }

    /// Set the timer target value (24-bit)
    ///
    /// # Arguments
    ///
    /// * `target` - The 24-bit timer target value
    ///
    /// This writes the target value to the timer registers and strobes it to take effect.
    pub async fn set_timer_value(
        &mut self,
        target: u32,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        // Ensure the value fits in 24 bits
        let target = target & 0xFF_FFFF;

        // Write the three bytes (MSB first)
        let hi_byte = ((target >> 16) & 0xFF) as u8;
        let mid_byte = ((target >> 8) & 0xFF) as u8;
        let lo_byte = (target & 0xFF) as u8;

        self.device
            .timer()
            .timerhibyte()
            .write_async(|reg| reg.set_timerhibyte(hi_byte))
            .await?;

        self.device
            .timer()
            .timermidbyte()
            .write_async(|reg| reg.set_timermidbyte(mid_byte))
            .await?;

        self.device
            .timer()
            .timerlobyte()
            .write_async(|reg| reg.set_timerlobyte(lo_byte))
            .await?;

        // Strobe to load the target
        self.device
            .timer()
            .tasktimertargetstrobe()
            .dispatch_async(|command| command.set_tasktimertargetstrobe(Task::Trigger))
            .await?;

        Ok(())
    }

    /// Read the timer target value (24-bit)
    ///
    /// # Returns
    ///
    /// Returns the currently set 24-bit timer target value, or Err(NPM1300Error::I2c) if there
    /// was an error communicating with the device.
    pub async fn get_timer_value(&mut self) -> Result<u32, crate::NPM1300Error<I2c::Error>> {
        let hi_byte = self
            .device
            .timer()
            .timerhibyte()
            .read_async()
            .await?
            .timerhibyte();

        let mid_byte = self
            .device
            .timer()
            .timermidbyte()
            .read_async()
            .await?
            .timermidbyte();

        let lo_byte = self
            .device
            .timer()
            .timerlobyte()
            .read_async()
            .await?
            .timerlobyte();

        let value = ((hi_byte as u32) << 16) | ((mid_byte as u32) << 8) | (lo_byte as u32);

        Ok(value)
    }

    /// Kick the watchdog timer
    ///
    /// This resets the watchdog counter. Must be called periodically when the
    /// watchdog is enabled to prevent system reset.
    pub async fn kick_watchdog(&mut self) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .timer()
            .taskwatchdogkick()
            .dispatch_async(|command| command.set_taskwatchdogkick(Task::Trigger))
            .await
    }

    /// Configure the timer mode and prescaler
    ///
    /// # Arguments
    ///
    /// * `mode` - The timer mode (boot monitor, watchdog warning, watchdog reset, general purpose timer, or wakeup timer)
    /// * `prescaler` - The timer prescaler (SLOW: 16 ms, FAST: 2 ms)
    pub async fn configure_timer(
        &mut self,
        mode: Timermodesel,
        prescaler: Timerprescaler,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .timer()
            .timerconfig()
            .write_async(|reg| {
                reg.set_timermodesel(mode);
                reg.set_timerprescaler(prescaler);
            })
            .await
    }

    /// Set the timer mode
    ///
    /// # Arguments
    ///
    /// * `mode` - The timer mode to set
    pub async fn set_timer_mode(
        &mut self,
        mode: Timermodesel,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .timer()
            .timerconfig()
            .modify_async(|reg| reg.set_timermodesel(mode))
            .await
    }

    /// Get the timer mode
    ///
    /// # Returns
    ///
    /// Returns the current timer mode, or Err(NPM1300Error::I2c) if there was
    /// an error communicating with the device.
    pub async fn get_timer_mode(
        &mut self,
    ) -> Result<Timermodesel, crate::NPM1300Error<I2c::Error>> {
        Ok(self
            .device
            .timer()
            .timerconfig()
            .read_async()
            .await?
            .timermodesel()
            .unwrap())
    }

    /// Set the timer prescaler
    ///
    /// # Arguments
    ///
    /// * `prescaler` - The prescaler to set (SLOW: 16 ms, FAST: 2 ms)
    pub async fn set_timer_prescaler(
        &mut self,
        prescaler: Timerprescaler,
    ) -> Result<(), crate::NPM1300Error<I2c::Error>> {
        self.device
            .timer()
            .timerconfig()
            .modify_async(|reg| reg.set_timerprescaler(prescaler))
            .await
    }

    /// Get the timer prescaler
    ///
    /// # Returns
    ///
    /// Returns the current timer prescaler setting, or Err(NPM1300Error::I2c)
    /// if there was an error communicating with the device.
    pub async fn get_timer_prescaler(
        &mut self,
    ) -> Result<Timerprescaler, crate::NPM1300Error<I2c::Error>> {
        Ok(self
            .device
            .timer()
            .timerconfig()
            .read_async()
            .await?
            .timerprescaler())
    }

    /// Check if the boot monitor is active
    ///
    /// # Returns
    ///
    /// Returns `true` if the boot monitor is running, `false` otherwise, or
    /// Err(NPM1300Error::I2c) if there was an error communicating with the device.
    pub async fn is_boot_monitor_active(
        &mut self,
    ) -> Result<bool, crate::NPM1300Error<I2c::Error>> {
        Ok(self
            .device
            .timer()
            .timerstatus()
            .read_async()
            .await?
            .bootmonitoractive()
            == Bootmonitoractive::Active)
    }

    /// Check if the timer is configured (slow domain is syncronized)
    ///
    /// Returns `true` if the timer is ready after TIMERTARGETSTROBE, `false` otherwise.
    ///
    /// # Returns
    ///
    /// Returns `true` if the timer is configured, `false` otherwise, or
    /// Err(NPM1300Error::I2c) if there was an error communicating with the device.
    pub async fn is_timer_configured(&mut self) -> Result<bool, crate::NPM1300Error<I2c::Error>> {
        Ok(self
            .device
            .timer()
            .timerstatus()
            .read_async()
            .await?
            .slowdomainconfigured()
            == Slowdomainconfigured::Config)
    }
}
