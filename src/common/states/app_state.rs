use crate::common::models::data::SharedAppData;
use crate::common::models::iot_devices::{IotDevice, IotDeviceType, SAlphaDeviceDetails};
use crate::common::models::iot_settings::{IotSettings, SAlphaIotPresets};
use crate::constants::default_values::DefaultValues;
use crate::helpers::date::get_time_now_for_default_tz;
use crate::helpers::system_time_x::SystemTimeX;
use crate::push_to_last_and_maintain_capacity_of_vector;
use crate::utils::math::max_of;
use actix_web::web;
use chrono::{DateTime, Duration};
use std::cell::Cell;
use std::collections::HashMap;
use std::num::Wrapping;
use std::ops::Add;
use std::sync::Mutex;

pub type IotDevicesActivityBucket = HashMap<String, IotDeviceActivityContainer>;
pub type SharedAppState = web::Data<Mutex<AppState>>;

#[derive(Debug)]
pub struct AppState {
    pub iot_devices_state: IotDevicesState,
    pub launch_time: DateTime<chrono_tz::Tz>,
    pub launch_time_tz: chrono_tz::Tz,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            iot_devices_state: IotDevicesState::new(),
            launch_time: get_time_now_for_default_tz(),
            launch_time_tz: DefaultValues::DEFAULT_TIMEZONE,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IotDevicesState {
    pub devices_activity_bucket: IotDevicesActivityBucket,
}

impl IotDevicesState {
    pub fn new() -> Self {
        Self {
            devices_activity_bucket: HashMap::new(),
        }
    }

    pub fn insert_new(
        &mut self,
        device_id: &str,
        device_type: IotDeviceType,
        iot_device: &IotDevice,
        shared_app_data: &SharedAppData,
    ) {
        let existing_activity_bucket = self.devices_activity_bucket.get(device_id);
        match existing_activity_bucket {
            None => {
                let next_iot_device_activity_container =
                    IotDeviceActivityContainer::new(device_id, device_type, iot_device);

                self.devices_activity_bucket
                    .entry(device_id.to_owned())
                    .or_insert_with(|| next_iot_device_activity_container);
            }
            Some(current_iot_device_activity_container) => {
                let next_iot_device_activity_container =
                    current_iot_device_activity_container.clone().update(
                        device_id,
                        device_type,
                        iot_device,
                        &shared_app_data.config.iot_settings,
                    );

                self.devices_activity_bucket
                    .insert(device_id.to_owned(), next_iot_device_activity_container);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct IotDeviceActivityContainer {
    pub data_storage: Vec<IotDeviceActivityDataUnit>,
    pub device_type: IotDeviceType,
    pub device_id: String,
    pub device_location: String,
    pub device_name: String,
    pub last_activity_time: DateTime<chrono_tz::Tz>,
    pub last_activity_tz: chrono_tz::Tz,
    pub total_running_time: Duration,
    pub is_first_ping_after_device_turned_on: bool,
    pub next_continuous_period_buzzer_notification_time: Cell<SystemTimeX>,
    pub device_states: IotDeviceAppState,
}

struct IotDeviceActivity {
    pub total_running_time: Duration,
    pub is_first_ping_after_device_turned_on: bool,
}

impl IotDeviceActivityContainer {
    pub fn new(device_id: &str, device_type: IotDeviceType, device_data: &IotDevice) -> Self {
        let next_device_activity_data_unit =
            IotDeviceActivityDataUnit::new(device_id, device_type, device_data);

        let last_activity_time = next_device_activity_data_unit.time;
        let last_activity_tz = next_device_activity_data_unit.tz;

        let SAlphaDeviceDetails {
            is_first_ping_after_device_turned_on,
            device_location,
            device_name,
            ..
        } = match device_data {
            IotDevice::RoofWaterHeater(d)
            | IotDevice::BoreWellMotor(d)
            | IotDevice::GroundWellMotor(d) => d,
        };

        Self {
            data_storage: vec![next_device_activity_data_unit],
            device_type,
            device_id: device_id.to_owned(),
            device_location: device_location.clone(),
            device_name: device_name.clone(),
            last_activity_time,
            last_activity_tz,
            total_running_time: Duration::zero(),
            device_states: IotDeviceAppState::default(device_type),
            is_first_ping_after_device_turned_on: *is_first_ping_after_device_turned_on,
            next_continuous_period_buzzer_notification_time: Cell::new(SystemTimeX::now()),
        }
    }

    fn update(
        self,
        device_id: &str,
        device_type: IotDeviceType,
        device_data: &IotDevice,
        iot_settings: &IotSettings,
    ) -> Self {
        let next_continuous_period_buzzer_notification_time = self
            .next_continuous_period_buzzer_notification_time
            .clone();

        let device_name = self.device_name.clone();
        let device_location = self.device_location.clone();

        let mut current_iot_device_activity_container_data_storage = self.data_storage.clone();
        let next_device_activity_data_unit =
            IotDeviceActivityDataUnit::new(device_id, device_type, device_data);

        let next_device_activity_data_unit_clone = next_device_activity_data_unit.clone();

        // we keep a hard cap of [MAX_ACTIVITIES_VECTOR_LENGTH_PER_DEVICE] values
        // to prevent the app from consuming too much memory
        if current_iot_device_activity_container_data_storage.len()
            > DefaultValues::MAX_ACTIVITIES_VECTOR_LENGTH_PER_DEVICE
        {
            // if the activities vector contains more than allowed number of items in the memory
            // then we clear a range of items from the start position of the vector
            // to prevent the `out of range` error we choose `clear memory length` to splice the vector or `maximum allowed length`, which ever is the lowest of two.

            // we splice out a chunk from the vector to prevent vector size management everytime we reach the maximum allowed threshold
            #[allow(clippy::integer_arithmetic)]
            {
                current_iot_device_activity_container_data_storage =
                    push_to_last_and_maintain_capacity_of_vector(
                        current_iot_device_activity_container_data_storage,
                        max_of(
                            Wrapping(
                                DefaultValues::MAX_ACTIVITIES_VECTOR_LENGTH_PER_DEVICE
                                    - DefaultValues::MAX_ACTIVITIES_PER_DEVICE_CLEAR_SPLICE_SIZE,
                            ),
                            Wrapping(0),
                        ),
                        next_device_activity_data_unit,
                    );
            }
        } else {
            current_iot_device_activity_container_data_storage.push(next_device_activity_data_unit);
        }

        let last_activity_tz = next_device_activity_data_unit_clone.tz;
        let last_activity_time = next_device_activity_data_unit_clone.time;

        let device_states = self.device_states.clone();
        let IotDeviceActivity {
            total_running_time,
            is_first_ping_after_device_turned_on,
        } = self.fetch_activity_time(
            &next_device_activity_data_unit_clone,
            iot_settings,
            device_data,
        );

        Self {
            total_running_time,
            last_activity_time,
            data_storage: current_iot_device_activity_container_data_storage,
            device_type,
            last_activity_tz,
            device_id: device_id.to_owned(),
            device_location,
            device_states,
            is_first_ping_after_device_turned_on,
            next_continuous_period_buzzer_notification_time,
            device_name,
        }
    }

    fn fetch_activity_time(
        self,
        next_device_activity_data_unit: &IotDeviceActivityDataUnit,
        iot_settings: &IotSettings,
        device_data: &IotDevice,
    ) -> IotDeviceActivity {
        match device_data {
            IotDevice::RoofWaterHeater(d) => self.fetch_total_activity_time_salpha(
                next_device_activity_data_unit,
                &iot_settings.settings.presets.roof_water_heater,
                d,
            ),
            IotDevice::BoreWellMotor(d) => self.fetch_total_activity_time_salpha(
                next_device_activity_data_unit,
                &iot_settings.settings.presets.bore_well_motor,
                d,
            ),
            IotDevice::GroundWellMotor(d) => self.fetch_total_activity_time_salpha(
                next_device_activity_data_unit,
                &iot_settings.settings.presets.ground_well_motor,
                d,
            ),
        }
    }

    fn fetch_total_activity_time_salpha(
        self,
        next_device_activity_data_unit: &IotDeviceActivityDataUnit,
        salpha_presets: &SAlphaIotPresets,
        d: &SAlphaDeviceDetails,
    ) -> IotDeviceActivity {
        let next_device_activity_time = next_device_activity_data_unit.time;

        let total_running_time: Duration = (|| {
            let time_diff: Duration = next_device_activity_time - self.last_activity_time;
            let time_diff_ms = time_diff.num_milliseconds();

            // if the time difference between the [next_device_activity_time] and the [last_activity_time] is eq.to or greater than the [SAlphaIotPresets.max_interval_to_persist_session_ms] then [total_running_time] should reset to 0
            if time_diff_ms >= salpha_presets.max_interval_to_persist_session_ms {
                return Duration::zero();
            }

            // if the time difference between the [next_device_activity_time] and the [last_activity_time] is eq.to or greater than the [SAlphaIotPresets.pause_total_running_time_on_inactive_for_ms] then pause the activity after [salpha_presets.pause_total_running_time_on_inactive_for_ms] duration after the [last_activity_time]

            // (  [last_activity_time] | <---[pause_total_running_time_on_inactive_for_ms]---> | <---[silent_activity_time_to_skip]---> | [next_device_activity_time] )
            // (  <--- [last_activity_permitted_before_pausing]   -------------> | )
            if time_diff_ms >= salpha_presets.pause_total_running_time_on_inactive_for_ms {
                let last_activity_permitted_before_pausing =
                    self.last_activity_time.add(Duration::milliseconds(
                        salpha_presets.pause_total_running_time_on_inactive_for_ms,
                    ));

                let silent_activity_time_to_skip =
                    next_device_activity_time - last_activity_permitted_before_pausing;

                return self.total_running_time + time_diff - silent_activity_time_to_skip;
            }

            // else increment the time diff to the current [total_running_time] and return
            self.total_running_time + time_diff
        })();

        IotDeviceActivity {
            total_running_time,
            is_first_ping_after_device_turned_on: d.is_first_ping_after_device_turned_on,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IotDeviceActivityDataUnit {
    pub time: DateTime<chrono_tz::Tz>,
    pub tz: chrono_tz::Tz,
    pub device_data: IotDevice,
    pub device_id: String,
    pub device_type: IotDeviceType,
}

impl IotDeviceActivityDataUnit {
    pub fn new(device_id: &str, device_type: IotDeviceType, device_data: &IotDevice) -> Self {
        Self {
            time: get_time_now_for_default_tz(),
            tz: DefaultValues::DEFAULT_TIMEZONE,
            device_data: device_data.clone(),
            device_id: device_id.to_owned(),
            device_type,
        }
    }
}

// Stores the states of the IOT devices
#[derive(Debug, Clone)]
pub enum IotDeviceAppState {
    RoofWaterHeater(SAlphaAppState),

    BoreWellMotor(SAlphaAppState),

    GroundWellMotor(SAlphaAppState),
}

impl IotDeviceAppState {
    pub const fn default(device_type: IotDeviceType) -> Self {
        match device_type {
            IotDeviceType::RoofWaterHeater => Self::RoofWaterHeater(SAlphaAppState::default()),
            IotDeviceType::BoreWellMotor => Self::BoreWellMotor(SAlphaAppState::default()),
            IotDeviceType::GroundWellMotor => Self::GroundWellMotor(SAlphaAppState::default()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SAlphaAppState {
    pub last_short_period_buzzer_activity_time: Cell<Option<DateTime<chrono_tz::Tz>>>,
    pub last_short_period_buzzer_activity_tz: chrono_tz::Tz,
    pub last_continuous_period_buzzer_activity_time: Cell<Option<DateTime<chrono_tz::Tz>>>,
    pub last_continuous_short_period_buzzer_activity_tz: chrono_tz::Tz,
}

impl SAlphaAppState {
    pub const fn default() -> Self {
        Self {
            last_short_period_buzzer_activity_time: Cell::new(None),
            last_short_period_buzzer_activity_tz: DefaultValues::DEFAULT_TIMEZONE,
            last_continuous_period_buzzer_activity_time: Cell::new(None),
            last_continuous_short_period_buzzer_activity_tz: DefaultValues::DEFAULT_TIMEZONE,
        }
    }

    pub fn update_short_period_buzzer_activity_time(&self) -> &Self {
        self.last_short_period_buzzer_activity_time
            .set(Some(get_time_now_for_default_tz()));

        self
    }

    pub fn update_continuous_period_buzzer_activity_time(&self) -> &Self {
        self.last_continuous_period_buzzer_activity_time
            .set(Some(get_time_now_for_default_tz()));

        self
    }

    pub fn reset_short_period_buzzer_activity_time(&self) -> &Self {
        self.last_short_period_buzzer_activity_time.set(None);

        self
    }

    pub fn reset_continuous_period_buzzer_activity_time(&self) -> &Self {
        self.last_continuous_period_buzzer_activity_time.set(None);

        self
    }
}
