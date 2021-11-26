use crate::common::models::data::IotDevice;
use crate::constants::default_values::DefaultValues;
use crate::helpers::date::get_time_now_default_tz;
use crate::push_to_last_and_maintain_capacity_of_vector;
use crate::utils::math::max_of;
use chrono::DateTime;
use std::collections::HashMap;
use std::num::Wrapping;

type IotDevicesActivityBucket = HashMap<String, IotDevicesActivityContainer>;

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
            launch_time: get_time_now_default_tz(),
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

    pub fn insert_new(&mut self, device_id: &str, iot_device: &IotDevice) {
        let existing_activity_bucket = self.devices_activity_bucket.get(device_id);
        match existing_activity_bucket {
            None => {
                let next_iot_device_activity_container =
                    IotDevicesActivityContainer::new(device_id, iot_device);

                self.devices_activity_bucket
                    .entry(device_id.to_string())
                    .or_insert_with(|| next_iot_device_activity_container);
            }
            Some(current_iot_device_activity_container) => {
                let next_iot_device_activity_container = current_iot_device_activity_container
                    .clone()
                    .update(device_id, iot_device);

                self.devices_activity_bucket
                    .insert(device_id.to_string(), next_iot_device_activity_container);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct IotDevicesActivityContainer {
    pub data_storage: Vec<IotDeviceActivityDataUnit>,
    pub last_activity_time: DateTime<chrono_tz::Tz>,
    pub last_activity_tz: chrono_tz::Tz,
    pub total_running_time: chrono::Duration,
}

impl IotDevicesActivityContainer {
    pub fn new(device_id: &str, device_data: &IotDevice) -> Self {
        let next_device_activity_data_unit = IotDeviceActivityDataUnit::new(device_id, device_data);

        let last_activity_time = next_device_activity_data_unit.time;
        let last_activity_tz = next_device_activity_data_unit.tz;

        Self {
            data_storage: vec![next_device_activity_data_unit],
            last_activity_time,
            last_activity_tz,
            total_running_time: chrono::Duration::zero(),
        }
    }

    pub fn update(self, device_id: &str, device_data: &IotDevice) -> Self {
        let mut current_iot_device_activity_container_data_storage = self.data_storage.clone();
        let next_device_activity_data_unit = IotDeviceActivityDataUnit::new(device_id, device_data);

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

        let total_running_time = self.total_running_time
            + (next_device_activity_data_unit_clone.time - self.last_activity_time);

        let last_activity_tz = next_device_activity_data_unit_clone.tz;

        // we are cloning multiple items here to assist the intellij statical analysus since `chrono_tz` crate builds source file which is more than 8MB and the code insights will be turn off for the generated file.

        #[allow(clippy::redundant_clone)]
        let last_activity_time = next_device_activity_data_unit_clone.clone().time;

        Self {
            total_running_time,
            last_activity_time,
            data_storage: current_iot_device_activity_container_data_storage,
            last_activity_tz,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IotDeviceActivityDataUnit {
    pub time: DateTime<chrono_tz::Tz>,
    pub tz: chrono_tz::Tz,
    pub device_data: IotDevice,
    pub device_id: String,
}

impl IotDeviceActivityDataUnit {
    pub fn new(device_id: &str, device_data: &IotDevice) -> Self {
        Self {
            time: get_time_now_default_tz(),
            tz: DefaultValues::DEFAULT_TIMEZONE,
            device_data: device_data.clone(),
            device_id: device_id.to_string(),
        }
    }
}
