/// LDSW/LDO regulator voltages available on the nPM1300 (1.0V to 3.3V, 100mV steps)
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum LdoVoltage {
    V1_0 = 0,
    V1_1 = 1,
    V1_2 = 2,
    V1_3 = 3,
    V1_4 = 4,
    V1_5 = 5,
    V1_6 = 6,
    V1_7 = 7,
    V1_8 = 8,
    V1_9 = 9,
    V2_0 = 10,
    V2_1 = 11,
    V2_2 = 12,
    V2_3 = 13,
    V2_4 = 14,
    V2_5 = 15,
    V2_6 = 16,
    V2_7 = 17,
    V2_8 = 18,
    V2_9 = 19,
    V3_0 = 20,
    V3_1 = 21,
    V3_2 = 22,
    V3_3 = 23,
}

impl TryFrom<u8> for LdoVoltage {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use LdoVoltage::*;
        Ok(match value {
            0 => V1_0,
            1 => V1_1,
            2 => V1_2,
            3 => V1_3,
            4 => V1_4,
            5 => V1_5,
            6 => V1_6,
            7 => V1_7,
            8 => V1_8,
            9 => V1_9,
            10 => V2_0,
            11 => V2_1,
            12 => V2_2,
            13 => V2_3,
            14 => V2_4,
            15 => V2_5,
            16 => V2_6,
            17 => V2_7,
            18 => V2_8,
            19 => V2_9,
            20 => V3_0,
            21 => V3_1,
            22 => V3_2,
            23 => V3_3,
            _ => return Err(()),
        })
    }
}

impl From<LdoVoltage> for u8 {
    fn from(v: LdoVoltage) -> Self {
        v as u8
    }
}

/// Simple on/off state used by status bits.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum LdswPowerState {
    Off = 0,
    On = 1,
}

impl TryFrom<u8> for LdswPowerState {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Off),
            1 => Ok(Self::On),
            _ => Err(()),
        }
    }
}

impl From<LdswPowerState> for u8 {
    fn from(v: LdswPowerState) -> Self {
        v as u8
    }
}

/// Soft-start disable bit value.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum SoftStartDisable {
    Enabled = 0,  // soft-start enabled (default behavior)
    Disabled = 1, // soft-start disabled
}

impl TryFrom<u8> for SoftStartDisable {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Enabled),
            1 => Ok(Self::Disabled),
            _ => Err(()),
        }
    }
}

impl From<SoftStartDisable> for u8 {
    fn from(v: SoftStartDisable) -> Self {
        v as u8
    }
}

/// Soft-start current level selection.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum SoftStartLevel {
    Ma10 = 0,
    Ma20 = 1,
    Ma35 = 2,
    Ma50 = 3,
}

impl TryFrom<u8> for SoftStartLevel {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Ma10),
            1 => Ok(Self::Ma20),
            2 => Ok(Self::Ma35),
            3 => Ok(Self::Ma50),
            _ => Err(()),
        }
    }
}

impl From<SoftStartLevel> for u8 {
    fn from(v: SoftStartLevel) -> Self {
        v as u8
    }
}

/// Active discharge enable/disable.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum ActiveDischarge {
    Disabled = 0,
    Enabled = 1,
}

impl TryFrom<u8> for ActiveDischarge {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Disabled),
            1 => Ok(Self::Enabled),
            _ => Err(()),
        }
    }
}

impl From<ActiveDischarge> for u8 {
    fn from(v: ActiveDischarge) -> Self {
        v as u8
    }
}

/// Select whether the channel behaves as a load switch or an LDO.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum LdswMode {
    LoadSwitch = 0,
    Ldo = 1,
}

impl TryFrom<u8> for LdswMode {
    type Error = ();

    fn try_from(v: u8) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::LoadSwitch),
            1 => Ok(Self::Ldo),
            _ => Err(()),
        }
    }
}

impl From<LdswMode> for u8 {
    fn from(v: LdswMode) -> Self {
        v as u8
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub struct LdswStatus {
    /// LDSW1 load switch power state.
    pub ldsw1_load_switch: LdswPowerState,
    /// LDSW1 LDO power state.
    pub ldsw1_ldo: LdswPowerState,
    /// LDSW2 load switch power state.
    pub ldsw2_load_switch: LdswPowerState,
    /// LDSW2 LDO power state.
    pub ldsw2_ldo: LdswPowerState,
    /// Indicates whether LDSW1 or LDSW2 is in use.
    pub ldsw_enable: LdswPowerState,
}
