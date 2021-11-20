use crate::common::models::data::IotDevice;
use crate::constants::default_values::DefaultValues;
use crate::{max_of, push_to_last_and_maintain_capacity_of_vector};
use chrono::Utc;
use std::collections::HashMap;
use std::num::Wrapping;
use std::ops::Deref;

type IotDevicesActivity = HashMap<IotDevice, Vec<IotDeviceActivityData>>;

#[derive(Debug)]
pub struct AppState {
    pub iot_devices_state: IotDevicesState,
}

#[derive(Debug)]
pub struct IotDevicesState {
    pub devices_activity: IotDevicesActivity,
}

#[derive(Debug, Clone)]
pub struct IotDeviceActivityData {
    pub time: chrono::DateTime<Utc>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            iot_devices_state: IotDevicesState::new(),
        }
    }
}

impl IotDevicesState {
    pub fn new() -> Self {
        Self {
            devices_activity: HashMap::new(),
        }
    }

    pub fn insert_new(&mut self, iot_device: IotDevice) {
        let activity_data = IotDeviceActivityData::new();

        let existing_activity_bucket = self.devices_activity.get(&iot_device);

        match existing_activity_bucket {
            None => {
                self.devices_activity
                    .insert(iot_device, Vec::<IotDeviceActivityData>::new());
            }
            Some(current_activities) => {
                let mut current_activities_cloned = current_activities.clone();

                // we keep a hard cap of [MAX_ACTIVITIES_VECTOR_LENGTH_PER_DEVICE] values
                // to prevent the app from consuming too much memory
                if current_activities_cloned.len()
                    > DefaultValues::MAX_ACTIVITIES_VECTOR_LENGTH_PER_DEVICE
                {
                    // if the activities vector contains more than allowed number of items in the memory
                    // then we clear a range of items from the start position of the vector
                    // to prevent the `out of range` error we choose `clear memory length` to splice the vector or `maximum allowed length`, which ever is the lowest of two.

                    // we splice out a chunk from the vector to prevent vector size management everytime we reach the maximum allowed threshold

                    current_activities_cloned = push_to_last_and_maintain_capacity_of_vector(
                        current_activities_cloned,
                        max_of(
                            Wrapping(DefaultValues::MAX_ACTIVITIES_VECTOR_LENGTH_PER_DEVICE)
                                - Wrapping(
                                    DefaultValues::MAX_ACTIVITIES_PER_DEVICE_CLEAR_SPLICE_SIZE,
                                ),
                            Wrapping(0),
                        ),
                        activity_data,
                    );
                } else {
                    current_activities_cloned.push(activity_data);
                }

                self.devices_activity
                    .insert(iot_device, current_activities_cloned);
            }
        };
    }
}

impl IotDeviceActivityData {
    pub fn new() -> Self {
        Self {
            time: chrono::Utc::now(),
        }
    }
}
