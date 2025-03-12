//! Standard EC types
use core::mem::offset_of;

pub mod message;
pub mod structure;

/// Error type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// The requested base + offset is invalid
    InvalidLocation,
}

/// Convert from memory map offset and length to battery message
/// Modifies offset and length
pub fn mem_map_to_battery_msg(
    memory_map: &structure::ECMemory,
    offset: &mut usize,
    length: &mut usize,
) -> Result<message::BatteryMessage, Error> {
    let local_offset = *offset - offset_of!(structure::ECMemory, batt);
    let mut message: Option<message::BatteryMessage> = None;

    if local_offset == offset_of!(structure::Battery, events) {
        let value = memory_map.batt.events;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::BatteryMessage::Events(value));
    } else if local_offset == offset_of!(structure::Battery, last_full_charge) {
        let value = memory_map.batt.last_full_charge;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::BatteryMessage::LastFullCharge(value));
    } else if local_offset == offset_of!(structure::Battery, cycle_count) {
        let value = memory_map.batt.cycle_count;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::BatteryMessage::CycleCount(value));
    } else if local_offset == offset_of!(structure::Battery, state) {
        let value = memory_map.batt.state;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::BatteryMessage::State(value));
    } else if local_offset == offset_of!(structure::Battery, present_rate) {
        let value = memory_map.batt.present_rate;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::BatteryMessage::PresentRate(value));
    } else if local_offset == offset_of!(structure::Battery, remain_cap) {
        let value = memory_map.batt.remain_cap;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::BatteryMessage::RemainCap(value));
    } else if local_offset == offset_of!(structure::Battery, present_volt) {
        let value = memory_map.batt.present_volt;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::BatteryMessage::PresentVolt(value));
    } else if local_offset == offset_of!(structure::Battery, psr_state) {
        let value = memory_map.batt.psr_state;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::BatteryMessage::PsrState(value));
    } else if local_offset == offset_of!(structure::Battery, psr_max_out) {
        let value = memory_map.batt.psr_max_out;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::BatteryMessage::PsrMaxOut(value));
    } else if local_offset == offset_of!(structure::Battery, psr_max_in) {
        let value = memory_map.batt.psr_max_in;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::BatteryMessage::PsrMaxIn(value));
    } else if local_offset == offset_of!(structure::Battery, peak_level) {
        let value = memory_map.batt.peak_level;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::BatteryMessage::PeakLevel(value));
    } else if local_offset == offset_of!(structure::Battery, peak_power) {
        let value = memory_map.batt.peak_power;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::BatteryMessage::PeakPower(value));
    } else if local_offset == offset_of!(structure::Battery, sus_level) {
        let value = memory_map.batt.sus_level;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::BatteryMessage::SusLevel(value));
    } else if local_offset == offset_of!(structure::Battery, sus_power) {
        let value = memory_map.batt.sus_power;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::BatteryMessage::SusPower(value));
    } else if local_offset == offset_of!(structure::Battery, peak_thres) {
        let value = memory_map.batt.peak_thres;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::BatteryMessage::PeakThres(value));
    } else if local_offset == offset_of!(structure::Battery, sus_thres) {
        let value = memory_map.batt.sus_thres;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::BatteryMessage::SusThres(value));
    } else if local_offset == offset_of!(structure::Battery, trip_thres) {
        let value = memory_map.batt.trip_thres;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::BatteryMessage::TripThres(value));
    } else if local_offset == offset_of!(structure::Battery, bmc_data) {
        let value = memory_map.batt.bmc_data;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::BatteryMessage::BmcData(value));
    } else if local_offset == offset_of!(structure::Battery, bmd_data) {
        let value = memory_map.batt.bmd_data;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::BatteryMessage::BmdData(value));
    } else if local_offset == offset_of!(structure::Battery, bmd_flags) {
        let value = memory_map.batt.bmd_flags;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::BatteryMessage::BmdFlags(value));
    } else if local_offset == offset_of!(structure::Battery, bmd_count) {
        let value = memory_map.batt.bmd_count;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::BatteryMessage::BmdCount(value));
    } else if local_offset == offset_of!(structure::Battery, charge_time) {
        let value = memory_map.batt.charge_time;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::BatteryMessage::ChargeTime(value));
    } else if local_offset == offset_of!(structure::Battery, run_time) {
        let value = memory_map.batt.run_time;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::BatteryMessage::RunTime(value));
    } else if local_offset == offset_of!(structure::Battery, sample_time) {
        let value = memory_map.batt.sample_time;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::BatteryMessage::SampleTime(value));
    }

    if let Some(msg) = message {
        Ok(msg)
    } else {
        Err(Error::InvalidLocation)
    }
}

/// Convert from memory map offset and length to thermal message from offset and length
/// Modifies offset and length
pub fn mem_map_to_thermal_msg(
    memory_map: &structure::ECMemory,
    offset: &mut usize,
    length: &mut usize,
) -> Result<message::ThermalMessage, Error> {
    let local_offset = *offset - offset_of!(structure::ECMemory, therm);
    let mut message: Option<message::ThermalMessage> = None;

    if local_offset == offset_of!(structure::Thermal, events) {
        let value = memory_map.therm.events;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::ThermalMessage::Events(value));
    } else if local_offset == offset_of!(structure::Thermal, cool_mode) {
        let value = memory_map.therm.cool_mode;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::ThermalMessage::CoolMode(value));
    } else if local_offset == offset_of!(structure::Thermal, dba_limit) {
        let value = memory_map.therm.dba_limit;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::ThermalMessage::DbaLimit(value));
    } else if local_offset == offset_of!(structure::Thermal, sonne_limit) {
        let value = memory_map.therm.sonne_limit;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::ThermalMessage::SonneLimit(value));
    } else if local_offset == offset_of!(structure::Thermal, ma_limit) {
        let value = memory_map.therm.ma_limit;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::ThermalMessage::MaLimit(value));
    } else if local_offset == offset_of!(structure::Thermal, fan1_on_temp) {
        let value = memory_map.therm.fan1_on_temp;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::ThermalMessage::Fan1OnTemp(value));
    } else if local_offset == offset_of!(structure::Thermal, fan1_ramp_temp) {
        let value = memory_map.therm.fan1_ramp_temp;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::ThermalMessage::Fan1RampTemp(value));
    } else if local_offset == offset_of!(structure::Thermal, fan1_max_temp) {
        let value = memory_map.therm.fan1_max_temp;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::ThermalMessage::Fan1MaxTemp(value));
    } else if local_offset == offset_of!(structure::Thermal, fan1_crt_temp) {
        let value = memory_map.therm.fan1_crt_temp;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::ThermalMessage::Fan1CrtTemp(value));
    } else if local_offset == offset_of!(structure::Thermal, fan1_hot_temp) {
        let value = memory_map.therm.fan1_hot_temp;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::ThermalMessage::Fan1HotTemp(value));
    } else if local_offset == offset_of!(structure::Thermal, fan1_max_rpm) {
        let value = memory_map.therm.fan1_max_rpm;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::ThermalMessage::Fan1MaxRpm(value));
    } else if local_offset == offset_of!(structure::Thermal, fan1_cur_rpm) {
        let value = memory_map.therm.fan1_cur_rpm;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::ThermalMessage::Fan1CurRpm(value));
    } else if local_offset == offset_of!(structure::Thermal, tmp1_val) {
        let value = memory_map.therm.tmp1_val;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::ThermalMessage::Tmp1Val(value));
    } else if local_offset == offset_of!(structure::Thermal, tmp1_timeout) {
        let value = memory_map.therm.tmp1_timeout;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::ThermalMessage::Tmp1Timeout(value));
    } else if local_offset == offset_of!(structure::Thermal, tmp1_low) {
        let value = memory_map.therm.tmp1_low;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::ThermalMessage::Tmp1Low(value));
    } else if local_offset == offset_of!(structure::Thermal, tmp1_high) {
        let value = memory_map.therm.tmp1_high;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::ThermalMessage::Tmp1High(value));
    }

    if let Some(msg) = message {
        Ok(msg)
    } else {
        Err(Error::InvalidLocation)
    }
}

/// Convert from memory map offset and length to time alarm message
/// Modifies offset and length
pub fn mem_map_to_time_alarm_msg(
    memory_map: &structure::ECMemory,
    offset: &mut usize,
    length: &mut usize,
) -> Result<message::TimeAlarmMessage, Error> {
    let local_offset = *offset - offset_of!(structure::ECMemory, alarm);
    let mut message: Option<message::TimeAlarmMessage> = None;

    if local_offset == offset_of!(structure::TimeAlarm, events) {
        let value = memory_map.alarm.events;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::TimeAlarmMessage::Events(value));
    } else if local_offset == offset_of!(structure::TimeAlarm, capability) {
        let value = memory_map.alarm.capability;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::TimeAlarmMessage::Capability(value));
    } else if local_offset == offset_of!(structure::TimeAlarm, year) {
        let value = memory_map.alarm.year;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::TimeAlarmMessage::Year(value));
    } else if local_offset == offset_of!(structure::TimeAlarm, month) {
        let value = memory_map.alarm.month;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::TimeAlarmMessage::Month(value));
    } else if local_offset == offset_of!(structure::TimeAlarm, day) {
        let value = memory_map.alarm.day;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::TimeAlarmMessage::Day(value));
    } else if local_offset == offset_of!(structure::TimeAlarm, hour) {
        let value = memory_map.alarm.hour;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::TimeAlarmMessage::Hour(value));
    } else if local_offset == offset_of!(structure::TimeAlarm, minute) {
        let value = memory_map.alarm.minute;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::TimeAlarmMessage::Minute(value));
    } else if local_offset == offset_of!(structure::TimeAlarm, second) {
        let value = memory_map.alarm.second;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::TimeAlarmMessage::Second(value));
    } else if local_offset == offset_of!(structure::TimeAlarm, valid) {
        let value = memory_map.alarm.valid;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::TimeAlarmMessage::Valid(value));
    } else if local_offset == offset_of!(structure::TimeAlarm, daylight) {
        let value = memory_map.alarm.daylight;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::TimeAlarmMessage::Daylight(value));
    } else if local_offset == offset_of!(structure::TimeAlarm, res1) {
        let value = memory_map.alarm.res1;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::TimeAlarmMessage::Res1(value));
    } else if local_offset == offset_of!(structure::TimeAlarm, milli) {
        let value = memory_map.alarm.milli;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::TimeAlarmMessage::Milli(value));
    } else if local_offset == offset_of!(structure::TimeAlarm, time_zone) {
        let value = memory_map.alarm.time_zone;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::TimeAlarmMessage::TimeZone(value));
    } else if local_offset == offset_of!(structure::TimeAlarm, res2) {
        let value = memory_map.alarm.res2;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::TimeAlarmMessage::Res2(value));
    } else if local_offset == offset_of!(structure::TimeAlarm, alarm_status) {
        let value = memory_map.alarm.alarm_status;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::TimeAlarmMessage::AlarmStatus(value));
    } else if local_offset == offset_of!(structure::TimeAlarm, ac_time_val) {
        let value = memory_map.alarm.ac_time_val;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::TimeAlarmMessage::AcTimeVal(value));
    } else if local_offset == offset_of!(structure::TimeAlarm, dc_time_val) {
        let value = memory_map.alarm.dc_time_val;
        *offset += size_of_val(&value);
        *length -= size_of_val(&value);
        message = Some(message::TimeAlarmMessage::DcTimeVal(value));
    }

    if let Some(msg) = message {
        Ok(msg)
    } else {
        Err(Error::InvalidLocation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem_map_to_battery_msg() {
        use crate::ec_type::message::BatteryMessage;
        use crate::ec_type::structure::{Battery, ECMemory};

        let memory_map = ECMemory {
            batt: Battery {
                events: 1,
                last_full_charge: 2,
                cycle_count: 3,
                state: 4,
                present_rate: 5,
                remain_cap: 6,
                present_volt: 7,
                psr_state: 8,
                psr_max_out: 9,
                psr_max_in: 10,
                peak_level: 11,
                peak_power: 12,
                sus_level: 13,
                sus_power: 14,
                peak_thres: 15,
                sus_thres: 16,
                trip_thres: 17,
                bmc_data: 18,
                bmd_data: 19,
                bmd_flags: 20,
                bmd_count: 21,
                charge_time: 22,
                run_time: 23,
                sample_time: 24,
            },
            ..Default::default()
        };

        let mut offset = offset_of!(ECMemory, batt);
        let mut length = size_of::<Battery>();

        let events = memory_map.batt.events;
        let next_offset = offset + size_of_val(&events);
        let next_length = length - size_of_val(&events);
        let msg = mem_map_to_battery_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, BatteryMessage::Events(1));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let last_full_charge = memory_map.batt.last_full_charge;
        let next_offset = offset + size_of_val(&last_full_charge);
        let next_length = length - size_of_val(&last_full_charge);
        let msg = mem_map_to_battery_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, BatteryMessage::LastFullCharge(2));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let cycle_count = memory_map.batt.cycle_count;
        let next_offset = offset + size_of_val(&cycle_count);
        let next_length = length - size_of_val(&cycle_count);
        let msg = mem_map_to_battery_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, BatteryMessage::CycleCount(3));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let state = memory_map.batt.state;
        let next_offset = offset + size_of_val(&state);
        let next_length = length - size_of_val(&state);
        let msg = mem_map_to_battery_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, BatteryMessage::State(4));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let present_rate = memory_map.batt.present_rate;
        let next_offset = offset + size_of_val(&present_rate);
        let next_length = length - size_of_val(&present_rate);
        let msg = mem_map_to_battery_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, BatteryMessage::PresentRate(5));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let remain_cap = memory_map.batt.remain_cap;
        let next_offset = offset + size_of_val(&remain_cap);
        let next_length = length - size_of_val(&remain_cap);
        let msg = mem_map_to_battery_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, BatteryMessage::RemainCap(6));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let present_volt = memory_map.batt.present_volt;
        let next_offset = offset + size_of_val(&present_volt);
        let next_length = length - size_of_val(&present_volt);
        let msg = mem_map_to_battery_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, BatteryMessage::PresentVolt(7));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let psr_state = memory_map.batt.psr_state;
        let next_offset = offset + size_of_val(&psr_state);
        let next_length = length - size_of_val(&psr_state);
        let msg = mem_map_to_battery_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, BatteryMessage::PsrState(8));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let psr_max_out = memory_map.batt.psr_max_out;
        let next_offset = offset + size_of_val(&psr_max_out);
        let next_length = length - size_of_val(&psr_max_out);
        let msg = mem_map_to_battery_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, BatteryMessage::PsrMaxOut(9));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let psr_max_in = memory_map.batt.psr_max_in;
        let next_offset = offset + size_of_val(&psr_max_in);
        let next_length = length - size_of_val(&psr_max_in);
        let msg = mem_map_to_battery_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, BatteryMessage::PsrMaxIn(10));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let peak_level = memory_map.batt.peak_level;
        let next_offset = offset + size_of_val(&peak_level);
        let next_length = length - size_of_val(&peak_level);
        let msg = mem_map_to_battery_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, BatteryMessage::PeakLevel(11));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let peak_power = memory_map.batt.peak_power;
        let next_offset = offset + size_of_val(&peak_power);
        let next_length = length - size_of_val(&peak_power);
        let msg = mem_map_to_battery_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, BatteryMessage::PeakPower(12));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let sus_level = memory_map.batt.sus_level;
        let next_offset = offset + size_of_val(&sus_level);
        let next_length = length - size_of_val(&sus_level);
        let msg = mem_map_to_battery_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, BatteryMessage::SusLevel(13));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let sus_power = memory_map.batt.sus_power;
        let next_offset = offset + size_of_val(&sus_power);
        let next_length = length - size_of_val(&sus_power);
        let msg = mem_map_to_battery_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, BatteryMessage::SusPower(14));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let peak_thres = memory_map.batt.peak_thres;
        let next_offset = offset + size_of_val(&peak_thres);
        let next_length = length - size_of_val(&peak_thres);
        let msg = mem_map_to_battery_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, BatteryMessage::PeakThres(15));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let sus_thres = memory_map.batt.sus_thres;
        let next_offset = offset + size_of_val(&sus_thres);
        let next_length = length - size_of_val(&sus_thres);
        let msg = mem_map_to_battery_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, BatteryMessage::SusThres(16));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let trip_thres = memory_map.batt.trip_thres;
        let next_offset = offset + size_of_val(&trip_thres);
        let next_length = length - size_of_val(&trip_thres);
        let msg = mem_map_to_battery_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, BatteryMessage::TripThres(17));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let bmc_data = memory_map.batt.bmc_data;
        let next_offset = offset + size_of_val(&bmc_data);
        let next_length = length - size_of_val(&bmc_data);
        let msg = mem_map_to_battery_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, BatteryMessage::BmcData(18));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let bmd_data = memory_map.batt.bmd_data;
        let next_offset = offset + size_of_val(&bmd_data);
        let next_length = length - size_of_val(&bmd_data);
        let msg = mem_map_to_battery_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, BatteryMessage::BmdData(19));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let bmd_flags = memory_map.batt.bmd_flags;
        let next_offset = offset + size_of_val(&bmd_flags);
        let next_length = length - size_of_val(&bmd_flags);
        let msg = mem_map_to_battery_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, BatteryMessage::BmdFlags(20));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let bmd_count = memory_map.batt.bmd_count;
        let next_offset = offset + size_of_val(&bmd_count);
        let next_length = length - size_of_val(&bmd_count);
        let msg = mem_map_to_battery_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, BatteryMessage::BmdCount(21));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let charge_time = memory_map.batt.charge_time;
        let next_offset = offset + size_of_val(&charge_time);
        let next_length = length - size_of_val(&charge_time);
        let msg = mem_map_to_battery_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, BatteryMessage::ChargeTime(22));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let run_time = memory_map.batt.run_time;
        let next_offset = offset + size_of_val(&run_time);
        let next_length = length - size_of_val(&run_time);
        let msg = mem_map_to_battery_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, BatteryMessage::RunTime(23));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let sample_time = memory_map.batt.sample_time;
        let next_offset = offset + size_of_val(&sample_time);
        let next_length = length - size_of_val(&sample_time);
        let msg = mem_map_to_battery_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, BatteryMessage::SampleTime(24));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        assert_eq!(length, 0);
    }

    #[test]
    fn test_mem_map_to_battery_msg_error() {
        use crate::ec_type::structure::{Battery, ECMemory};

        let memory_map = ECMemory {
            batt: Battery {
                events: 1,
                last_full_charge: 2,
                cycle_count: 3,
                state: 4,
                present_rate: 5,
                remain_cap: 6,
                present_volt: 7,
                psr_state: 8,
                psr_max_out: 9,
                psr_max_in: 10,
                peak_level: 11,
                peak_power: 12,
                sus_level: 13,
                sus_power: 14,
                peak_thres: 15,
                sus_thres: 16,
                trip_thres: 17,
                bmc_data: 18,
                bmd_data: 19,
                bmd_flags: 20,
                bmd_count: 21,
                charge_time: 22,
                run_time: 23,
                sample_time: 24,
            },
            ..Default::default()
        };

        let mut offset = offset_of!(ECMemory, batt) + 1;
        let mut length = size_of::<Battery>();

        let res = mem_map_to_battery_msg(&memory_map, &mut offset, &mut length);
        assert!(res.is_err() && res.unwrap_err() == Error::InvalidLocation);
    }

    #[test]
    fn test_mem_map_to_thermal_msg() {
        use crate::ec_type::message::ThermalMessage;
        use crate::ec_type::structure::{ECMemory, Thermal};

        let memory_map = ECMemory {
            therm: Thermal {
                events: 1,
                cool_mode: 2,
                dba_limit: 3,
                sonne_limit: 4,
                ma_limit: 5,
                fan1_on_temp: 6,
                fan1_ramp_temp: 7,
                fan1_max_temp: 8,
                fan1_crt_temp: 9,
                fan1_hot_temp: 10,
                fan1_max_rpm: 11,
                fan1_cur_rpm: 12,
                tmp1_val: 13,
                tmp1_timeout: 14,
                tmp1_low: 15,
                tmp1_high: 16,
            },
            ..Default::default()
        };

        let mut offset = offset_of!(ECMemory, therm);
        let mut length = size_of::<Thermal>();

        let events = memory_map.therm.events;
        let next_offset = offset + size_of_val(&events);
        let next_length = length - size_of_val(&events);
        let msg = mem_map_to_thermal_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, ThermalMessage::Events(1));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let cool_mode = memory_map.therm.cool_mode;
        let next_offset = offset + size_of_val(&cool_mode);
        let next_length = length - size_of_val(&cool_mode);
        let msg = mem_map_to_thermal_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, ThermalMessage::CoolMode(2));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let dba_limit = memory_map.therm.dba_limit;
        let next_offset = offset + size_of_val(&dba_limit);
        let next_length = length - size_of_val(&dba_limit);
        let msg = mem_map_to_thermal_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, ThermalMessage::DbaLimit(3));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let sonne_limit = memory_map.therm.sonne_limit;
        let next_offset = offset + size_of_val(&sonne_limit);
        let next_length = length - size_of_val(&sonne_limit);
        let msg = mem_map_to_thermal_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, ThermalMessage::SonneLimit(4));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let ma_limit = memory_map.therm.ma_limit;
        let next_offset = offset + size_of_val(&ma_limit);
        let next_length = length - size_of_val(&ma_limit);
        let msg = mem_map_to_thermal_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, ThermalMessage::MaLimit(5));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let fan1_on_temp = memory_map.therm.fan1_on_temp;
        let next_offset = offset + size_of_val(&fan1_on_temp);
        let next_length = length - size_of_val(&fan1_on_temp);
        let msg = mem_map_to_thermal_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, ThermalMessage::Fan1OnTemp(6));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let fan1_ramp_temp = memory_map.therm.fan1_ramp_temp;
        let next_offset = offset + size_of_val(&fan1_ramp_temp);
        let next_length = length - size_of_val(&fan1_ramp_temp);
        let msg = mem_map_to_thermal_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, ThermalMessage::Fan1RampTemp(7));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let fan1_max_temp = memory_map.therm.fan1_max_temp;
        let next_offset = offset + size_of_val(&fan1_max_temp);
        let next_length = length - size_of_val(&fan1_max_temp);
        let msg = mem_map_to_thermal_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, ThermalMessage::Fan1MaxTemp(8));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let fan1_crt_temp = memory_map.therm.fan1_crt_temp;
        let next_offset = offset + size_of_val(&fan1_crt_temp);
        let next_length = length - size_of_val(&fan1_crt_temp);
        let msg = mem_map_to_thermal_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, ThermalMessage::Fan1CrtTemp(9));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let fan1_hot_temp = memory_map.therm.fan1_hot_temp;
        let next_offset = offset + size_of_val(&fan1_hot_temp);
        let next_length = length - size_of_val(&fan1_hot_temp);
        let msg = mem_map_to_thermal_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, ThermalMessage::Fan1HotTemp(10));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let fan1_max_rpm = memory_map.therm.fan1_max_rpm;
        let next_offset = offset + size_of_val(&fan1_max_rpm);
        let next_length = length - size_of_val(&fan1_max_rpm);
        let msg = mem_map_to_thermal_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, ThermalMessage::Fan1MaxRpm(11));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let fan1_cur_rpm = memory_map.therm.fan1_cur_rpm;
        let next_offset = offset + size_of_val(&fan1_cur_rpm);
        let next_length = length - size_of_val(&fan1_cur_rpm);
        let msg = mem_map_to_thermal_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, ThermalMessage::Fan1CurRpm(12));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let tmp1_val = memory_map.therm.tmp1_val;
        let next_offset = offset + size_of_val(&tmp1_val);
        let next_length = length - size_of_val(&tmp1_val);
        let msg = mem_map_to_thermal_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, ThermalMessage::Tmp1Val(13));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let tmp1_timeout = memory_map.therm.tmp1_timeout;
        let next_offset = offset + size_of_val(&tmp1_timeout);
        let next_length = length - size_of_val(&tmp1_timeout);
        let msg = mem_map_to_thermal_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, ThermalMessage::Tmp1Timeout(14));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let tmp1_low = memory_map.therm.tmp1_low;
        let next_offset = offset + size_of_val(&tmp1_low);
        let next_length = length - size_of_val(&tmp1_low);
        let msg = mem_map_to_thermal_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, ThermalMessage::Tmp1Low(15));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let tmp1_high = memory_map.therm.tmp1_high;
        let next_offset = offset + size_of_val(&tmp1_high);
        let next_length = length - size_of_val(&tmp1_high);
        let msg = mem_map_to_thermal_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, ThermalMessage::Tmp1High(16));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        assert_eq!(length, 0);
    }

    #[test]
    fn test_mem_map_to_thermal_msg_error() {
        use crate::ec_type::structure::{ECMemory, Thermal};

        let memory_map = ECMemory {
            therm: Thermal {
                events: 1,
                cool_mode: 2,
                dba_limit: 3,
                sonne_limit: 4,
                ma_limit: 5,
                fan1_on_temp: 6,
                fan1_ramp_temp: 7,
                fan1_max_temp: 8,
                fan1_crt_temp: 9,
                fan1_hot_temp: 10,
                fan1_max_rpm: 11,
                fan1_cur_rpm: 12,
                tmp1_val: 13,
                tmp1_timeout: 14,
                tmp1_low: 15,
                tmp1_high: 16,
            },
            ..Default::default()
        };

        let mut offset = offset_of!(ECMemory, therm) + 1;
        let mut length = size_of::<Thermal>();

        let res = mem_map_to_thermal_msg(&memory_map, &mut offset, &mut length);
        assert!(res.is_err() && res.unwrap_err() == Error::InvalidLocation);
    }

    #[test]
    fn test_mem_map_to_time_alarm_msg() {
        use crate::ec_type::message::TimeAlarmMessage;
        use crate::ec_type::structure::{ECMemory, TimeAlarm};

        let memory_map = ECMemory {
            alarm: TimeAlarm {
                events: 1,
                capability: 2,
                year: 2025,
                month: 3,
                day: 12,
                hour: 10,
                minute: 30,
                second: 45,
                valid: 1,
                daylight: 0,
                res1: 0,
                milli: 500,
                time_zone: 1,
                res2: 0,
                alarm_status: 1,
                ac_time_val: 100,
                dc_time_val: 200,
            },
            ..Default::default()
        };

        let mut offset = offset_of!(ECMemory, alarm);
        let mut length = size_of::<TimeAlarm>();

        let events = memory_map.alarm.events;
        let next_offset = offset + size_of_val(&events);
        let next_length = length - size_of_val(&events);
        let msg = mem_map_to_time_alarm_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, TimeAlarmMessage::Events(1));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let capability = memory_map.alarm.capability;
        let next_offset = offset + size_of_val(&capability);
        let next_length = length - size_of_val(&capability);
        let msg = mem_map_to_time_alarm_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, TimeAlarmMessage::Capability(2));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let year = memory_map.alarm.year;
        let next_offset = offset + size_of_val(&year);
        let next_length = length - size_of_val(&year);
        let msg = mem_map_to_time_alarm_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, TimeAlarmMessage::Year(2025));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let month = memory_map.alarm.month;
        let next_offset = offset + size_of_val(&month);
        let next_length = length - size_of_val(&month);
        let msg = mem_map_to_time_alarm_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, TimeAlarmMessage::Month(3));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let day = memory_map.alarm.day;
        let next_offset = offset + size_of_val(&day);
        let next_length = length - size_of_val(&day);
        let msg = mem_map_to_time_alarm_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, TimeAlarmMessage::Day(12));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let hour = memory_map.alarm.hour;
        let next_offset = offset + size_of_val(&hour);
        let next_length = length - size_of_val(&hour);
        let msg = mem_map_to_time_alarm_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, TimeAlarmMessage::Hour(10));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let minute = memory_map.alarm.minute;
        let next_offset = offset + size_of_val(&minute);
        let next_length = length - size_of_val(&minute);
        let msg = mem_map_to_time_alarm_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, TimeAlarmMessage::Minute(30));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let second = memory_map.alarm.second;
        let next_offset = offset + size_of_val(&second);
        let next_length = length - size_of_val(&second);
        let msg = mem_map_to_time_alarm_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, TimeAlarmMessage::Second(45));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let valid = memory_map.alarm.valid;
        let next_offset = offset + size_of_val(&valid);
        let next_length = length - size_of_val(&valid);
        let msg = mem_map_to_time_alarm_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, TimeAlarmMessage::Valid(1));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let daylight = memory_map.alarm.daylight;
        let next_offset = offset + size_of_val(&daylight);
        let next_length = length - size_of_val(&daylight);
        let msg = mem_map_to_time_alarm_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, TimeAlarmMessage::Daylight(0));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let res1 = memory_map.alarm.res1;
        let next_offset = offset + size_of_val(&res1);
        let next_length = length - size_of_val(&res1);
        let msg = mem_map_to_time_alarm_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, TimeAlarmMessage::Res1(0));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let milli = memory_map.alarm.milli;
        let next_offset = offset + size_of_val(&milli);
        let next_length = length - size_of_val(&milli);
        let msg = mem_map_to_time_alarm_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, TimeAlarmMessage::Milli(500));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let time_zone = memory_map.alarm.time_zone;
        let next_offset = offset + size_of_val(&time_zone);
        let next_length = length - size_of_val(&time_zone);
        let msg = mem_map_to_time_alarm_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, TimeAlarmMessage::TimeZone(1));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let res2 = memory_map.alarm.res2;
        let next_offset = offset + size_of_val(&res2);
        let next_length = length - size_of_val(&res2);
        let msg = mem_map_to_time_alarm_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, TimeAlarmMessage::Res2(0));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let alarm_status = memory_map.alarm.alarm_status;
        let next_offset = offset + size_of_val(&alarm_status);
        let next_length = length - size_of_val(&alarm_status);
        let msg = mem_map_to_time_alarm_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, TimeAlarmMessage::AlarmStatus(1));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let ac_time_val = memory_map.alarm.ac_time_val;
        let next_offset = offset + size_of_val(&ac_time_val);
        let next_length = length - size_of_val(&ac_time_val);
        let msg = mem_map_to_time_alarm_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, TimeAlarmMessage::AcTimeVal(100));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        let dc_time_val = memory_map.alarm.dc_time_val;
        let next_offset = offset + size_of_val(&dc_time_val);
        let next_length = length - size_of_val(&dc_time_val);
        let msg = mem_map_to_time_alarm_msg(&memory_map, &mut offset, &mut length).unwrap();
        assert_eq!(msg, TimeAlarmMessage::DcTimeVal(200));
        assert_eq!(offset, next_offset);
        assert_eq!(length, next_length);

        assert_eq!(length, 0);
    }

    #[test]
    fn test_mem_map_to_time_alarm_msg_error() {
        use crate::ec_type::structure::{ECMemory, TimeAlarm};

        let memory_map = ECMemory {
            alarm: TimeAlarm {
                events: 1,
                capability: 2,
                year: 2025,
                month: 3,
                day: 12,
                hour: 10,
                minute: 30,
                second: 45,
                valid: 1,
                daylight: 0,
                res1: 0,
                milli: 500,
                time_zone: 1,
                res2: 0,
                alarm_status: 1,
                ac_time_val: 100,
                dc_time_val: 200,
            },
            ..Default::default()
        };

        let mut offset = offset_of!(ECMemory, alarm) + 1;
        let mut length = size_of::<TimeAlarm>();

        let res = mem_map_to_time_alarm_msg(&memory_map, &mut offset, &mut length);
        assert!(res.is_err() && res.unwrap_err() == Error::InvalidLocation);
    }
}
