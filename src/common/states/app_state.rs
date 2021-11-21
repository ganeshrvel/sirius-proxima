use crate::common::models::data::IotDevice;
use crate::constants::default_values::DefaultValues;
use crate::helpers::date::get_time_now_default_tz;
use crate::push_to_last_and_maintain_capacity_of_vector;
use crate::utils::math::max_of;
use chrono::DateTime;
use std::collections::HashMap;
use std::num::Wrapping;

type IotDevicesActivity = HashMap<IotDevice, Vec<IotDeviceActivityData>>;

#[derive(Debug)]
pub struct AppState {
    pub iot_devices_state: IotDevicesState,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            iot_devices_state: IotDevicesState::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IotDevicesState {
    pub devices_activity_bucket: IotDevicesActivity,
    pub last_activity_time: DateTime<chrono_tz::Tz>,
    pub last_activity_tz: chrono_tz::Tz,
    pub total_running_time: chrono::Duration,
}

impl IotDevicesState {
    pub fn new() -> Self {
        Self {
            devices_activity_bucket: HashMap::new(),
            last_activity_time: get_time_now_default_tz(),
            last_activity_tz: DefaultValues::DEFAULT_TIMEZONE,
            total_running_time: chrono::Duration::zero(),
        }
    }

    fn update_iot_device_activity(
        &mut self,
        activity_data: &IotDeviceActivityData,
        iot_device_activity_bucket_length: usize,
    ) {
        // we are cloning multiple items here to assist the intellij statical analysus since `chrono_tz` crate builds source file which is more than 8MB and the code insights will be turn off for the generated file.

        // if the length of `iot device activities` is greater than 1 then set the `total_running_time` else it will be defaulted to zero
        if iot_device_activity_bucket_length > 1 {
            self.total_running_time = self.total_running_time
                + (activity_data.clone().time - self.clone().last_activity_time);
        }

        self.last_activity_tz = activity_data.tz;
        self.last_activity_time = activity_data.clone().time;
    }

    pub fn insert_new(&mut self, iot_device: IotDevice) {
        let activity_data = IotDeviceActivityData::new();
        let activity_data_cloned = activity_data.clone();

        // false positive linting
        #[allow(unused_assignments)]
        let mut iot_device_activity_bucket_length: usize = 0;

        let existing_activity_bucket = self.devices_activity_bucket.get(&iot_device);

        match existing_activity_bucket {
            None => {
                self.devices_activity_bucket
                    .entry(iot_device)
                    .or_insert_with(|| vec![activity_data]);

                iot_device_activity_bucket_length = 1;
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
                    #[allow(clippy::integer_arithmetic)]
                    {
                        current_activities_cloned = push_to_last_and_maintain_capacity_of_vector(
                            current_activities_cloned,
                            max_of(
                                Wrapping(
                                    DefaultValues::MAX_ACTIVITIES_VECTOR_LENGTH_PER_DEVICE
                                        - DefaultValues::MAX_ACTIVITIES_PER_DEVICE_CLEAR_SPLICE_SIZE,
                                ),
                                Wrapping(0),
                            ),
                            activity_data,
                        );
                    }
                } else {
                    current_activities_cloned.push(activity_data);
                }

                iot_device_activity_bucket_length = current_activities_cloned.len();

                self.devices_activity_bucket
                    .insert(iot_device, current_activities_cloned);
            }
        };

        self.update_iot_device_activity(&activity_data_cloned, iot_device_activity_bucket_length);
    }
}

#[derive(Debug, Clone)]
pub struct IotDeviceActivityData {
    pub time: DateTime<chrono_tz::Tz>,
    pub tz: chrono_tz::Tz,
}

impl IotDeviceActivityData {
    pub fn new() -> Self {
        Self {
            time: get_time_now_default_tz(),
            tz: DefaultValues::DEFAULT_TIMEZONE,
        }
    }
}
