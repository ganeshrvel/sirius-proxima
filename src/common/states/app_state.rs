use crate::common::models::data::IotDevice;
use crate::constants::default_values::DefaultValues;
use chrono::Utc;
use std::collections::HashMap;

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

                // we keep a hard cap of [MAX_ACTIVITIES_VECTOR_LENGTH_PER_DEVICE] values to prevent the app from consuming too much memory
                if current_activities_cloned.len()
                    >= DefaultValues::MAX_ACTIVITIES_VECTOR_LENGTH_PER_DEVICE
                {
                    current_activities_cloned.remove(0);
                }

                current_activities_cloned.push(activity_data);

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
