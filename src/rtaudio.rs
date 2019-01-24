#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

use std::fmt;


/// Struct with specific audio device information. 
#[derive(Clone, Default)]
pub struct CS_AudioDevice {
    pub device_name: String,
    pub device_id:   String,
    pub rt_module:   String,
    pub max_nchnls:  u32,
    pub isOutput:    u32,
}

/// Struct with specific MIDI device information. 
#[derive(Clone, Default)]
pub struct CS_MidiDevice {
    pub device_name:    String,
    pub interface_name: String,
    pub device_id:      String,
    pub midi_module:    String,
    pub isOutput:       u32,
}

impl fmt::Debug for CS_MidiDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        f.debug_struct("CS_MidiDevice")
            .field("device_name", &self.device_name)
            .field("interface_name", &self.interface_name)
            .field("device_id", &self.device_id)
            .field("midi_module", &self.midi_module)
            .field("isOutput", &self.isOutput)
            .finish()
        }
}

impl fmt::Debug for CS_AudioDevice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        f.debug_struct("CS_AudioDevice")
            .field("device_name", &self.device_name)
            .field("device_id", &self.device_id)
            .field("rt_module", &self.rt_module)
            .field("max_nchnls", &self.max_nchnls)
            .field("isOutput", &self.isOutput)
            .finish()
    }
}

/// Real time audio params for a specific
/// audio Device.
#[derive(Debug, Clone, Default)]
pub struct RT_AudioParams {
    /// Device Name.
    pub devName:        String,
    /// Device number.
    pub devNum:         u32,
    /// Device software buffer size.
    pub bufSamp_SW:     u32,
    /// Device hardware buffer size.
    pub bufSamp_HW:     u32,
    /// Device max number of channels supported.
    pub nChannels:      u32,
    /// Device audio sample format.
    pub sampleFormat:   u32,
    /// Device max sample rate.
    pub sampleRate:     f32,
}
