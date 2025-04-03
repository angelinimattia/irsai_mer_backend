use serde::{Deserialize, Serialize};
use std::collections::HashMap; // Needed for the dynamic payload key

// --- Root Structure ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DroneStatus {
    pub payloads: Vec<Payload>,
    // 0:Standby, 1:Takeoff prep, 2:Takeoff prep complete, 3:Manual, 4:Auto takeoff, ...
    pub mode_code: i32,
    pub distance_limit_status: DistanceLimitStatus,
    pub wpmz_version: String,
    // Return home altitude (meters)
    pub rth_altitude: i32,
    // 0:Hovering, 1:Landing, 2:Returning to home
    pub rc_lost_action: i32,
    // [Deprecated] 0:Continue wayline, 1:Exit wayline & use rc_lost_action
    #[serde(rename = "exit_wayline_when_rc_lost")]
    pub exit_wayline_when_rc_lost_deprecated: Option<i32>, // Use Option for deprecated
    pub cameras: Vec<Camera>,
    pub country: String,
    // true:Normal, false:Abnormal
    pub rid_state: bool,
    // 0:Continue to-point, 1:Exit to-point & use rc_lost_action
    pub commander_mode_lost_action: i32,
    // 0:Optimal height, 1:Preset height
    pub current_commander_flight_mode: String, // Could be an Enum with custom serde
    // Height relative to takeoff point (meters)
    pub commander_flight_height: f64,
    // Reason code for current mode_code (see original description for mapping)
    pub mode_code_reason: i32,
    // 0:A, 1:P, 2:NAV, ...
    pub gear: i32,
    pub firmware_version: String,
    // 0:No consistency upgrade required, 1:Consistency upgrade required
    pub compatible_status: i32,
    // 0:Not upgraded, 1:Upgrading
    pub firmware_upgrade_status: i32,
    // Meters per second
    pub horizontal_speed: f64,
    // Meters per second
    pub vertical_speed: f64,
    pub longitude: f64,
    pub latitude: f64,
    // Absolute height relative to Earth ellipsoid
    pub height: f64,
    // Relative takeoff point altitude
    pub elevation: f64,
    // Degrees
    pub attitude_pitch: f64,
    // Degrees
    pub attitude_roll: f64,
    // Yaw relative to true north (degrees)
    pub attitude_head: i32,
    pub home_longitude: f64,
    pub home_latitude: f64,
    // Meters
    pub home_distance: f64,
    // 0.1 Meters per second
    pub wind_speed: f64,
    // 1:N, 2:NE, 3:E, 4:SE, 5:S, 6:SW, 7:W, 8:NW
    pub wind_direction: i32,
    // Device UUID or A/B for RC
    pub control_source: String,
    // Percent
    pub low_battery_warning_threshold: i32,
    // Percent
    pub serious_low_battery_warning_threshold: i32,
    // Seconds
    pub total_flight_time: f64,
    // Meters
    pub total_flight_distance: f64,
    pub battery: BatteryInfo,
    pub storage: StorageInfo,
    pub position_state: PositionState,
    // UUID String
    pub track_id: String,

    // --- Handling the dynamic key ---
    // This map will hold keys like "70-1-0" mapped to their specific info.
    // Using flatten merges the map keys directly into the DroneStatus struct during deserialization.
    #[serde(flatten)]
    pub payload_specific_data: HashMap<String, PayloadGimbalInfo>,

    pub total_flight_sorties: u32, // Use unsigned for counts
    pub maintain_status: MaintainStatus,
    // Unix timestamp in seconds
    pub activation_time: i64, // Use i64 for timestamps
    // 0:Disable, 1:On
    pub night_lights_state: i32,
    // Meters
    pub height_limit: i32,
    // 0:Not reaching, 1:Approaching
    pub is_near_height_limit: i32,
    // 0:Not reaching, 1:Approaching
    pub is_near_area_limit: i32,
    pub obstacle_avoidance: ObstacleAvoidance,
    // 0:Intelligent altitude, 1:Preset altitude
    pub current_rth_mode: i32,
    pub psdk_ui_resource: Vec<PsdkUiResource>,
    pub psdk_widget_values: Vec<PsdkWidgetValue>,
}

// --- Nested Structures ---

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct Payload {
    pub control_source: String,
    // Format: {type-subtype-gimbalindex}
    pub payload_index: String,
    pub firmware_version: String,
    pub sn: String,
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct DistanceLimitStatus {
    // 0:Not set, 1:Already set
    pub state: i32,
    // Limited distance in meters (15-8000)
    pub distance_limit: i32,
    // 0:Not reaching, 1:Approaching
    pub is_near_distance_limit: i32,
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct Camera {
    pub remain_photo_num: i32,
    // Seconds
    pub remain_record_duration: i32,
    // Seconds
    pub record_time: i32,
    // Format: {type-subtype-gimbalindex}
    pub payload_index: String,
    // 0:Capturing, 1:Recording, 2:Smart Low-Light, 3:Panorama, -1:Unsupported
    pub camera_mode: i32,
    // 0:Idle, 1:Capturing photo
    pub photo_state: i32,
    pub screen_split_enable: bool,
    // 0:Idle, 1:Recording
    pub recording_state: i32,
    // Zoom factor (e.g., 2.0 to 200.0)
    pub zoom_factor: f64,
    // Infrared zoom factor (e.g., 2.0 to 20.0)
    pub ir_zoom_factor: f64,
    pub liveview_world_region: LiveviewWorldRegion,
    // Array of strings like "current", "wide", "zoom", "ir"
    pub photo_storage_settings: Vec<String>,
    // Array of strings like "current", "wide", "zoom", "ir"
    pub video_storage_settings: Vec<String>,
    // 1:Auto, 2:Shutter priority, 3:Aperture priority, 4:Manual exposure
    pub wide_exposure_mode: i32,
    // 0:Auto, 1:Auto(High Sense), 2:50, ... 255:FIXED
    pub wide_iso: i32,
    // 0:"1/8000 s", ... 65534:Auto
    pub wide_shutter_speed: i32,
    // 1:"-5.0EV", ... 16:"0EV", ... 31:"5.0EV", 255:FIXED
    pub wide_exposure_value: i32,
    pub zoom_exposure_mode: i32,
    pub zoom_iso: i32,
    pub zoom_shutter_speed: i32,
    pub zoom_exposure_value: i32,
    // 0:MF, 1:AFS, 2:AFC
    pub zoom_focus_mode: i32,
    pub zoom_focus_value: i32,
    pub zoom_max_focus_value: i32,
    pub zoom_min_focus_value: i32,
    pub zoom_calibrate_farthest_focus_value: i32,
    pub zoom_calibrate_nearest_focus_value: i32,
    // 0:Idle, 1:Focusing, 2:Focus successful, 3:Focus failed
    pub zoom_focus_state: i32,
    // 0:Close, 1:Point, 2:Area
    pub ir_metering_mode: i32,
    pub ir_metering_point: IrMeteringPoint,
    pub ir_metering_area: IrMeteringArea,
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct LiveviewWorldRegion {
    // Coordinate origin is upper-left corner (0.0 to 1.0)
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct IrMeteringPoint {
    // Coordinate (0.0 to 1.0)
    pub x: f64,
    pub y: f64,
    // Celsius
    pub temperature: f64,
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct IrMeteringArea {
    // Top-left corner X (0.0 to 1.0)
    pub x: f64,
    // Top-left corner Y (0.0 to 1.0)
    pub y: f64,
    // Width (0.0 to 1.0)
    pub width: f64,
    // Height (0.0 to 1.0)
    pub height: f64,
    // Average temperature (Celsius)
    pub aver_temperature: f64,
    pub min_temperature_point: IrMeteringPoint,
    pub max_temperature_point: IrMeteringPoint,
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct BatteryInfo {
    // Percent (0-100)
    pub capacity_percent: i32,
    // Seconds
    pub remain_flight_time: i32,
    // Percent required for return home (0-100)
    pub return_home_power: i32,
    // Percent for forced landing (0-100)
    pub landing_power: i32,
    pub batteries: Vec<BatteryDetail>,
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct BatteryDetail {
    // Percent (0-100)
    pub capacity_percent: i32,
    pub index: i32,
    pub sn: String,
    // Use integer, specific enum mapping depends on device
    #[serde(rename = "type")]
    pub type_info: i32,
    pub sub_type: i32,
    pub firmware_version: String,
    pub loop_times: i32,
    // Millivolts
    pub voltage: i32,
    // Celsius (one decimal place suggests f64 is better)
    pub temperature: f64,
    // Days
    pub high_voltage_storage_days: i32,
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct StorageInfo {
    // Kilobytes
    pub total: i64, // Use i64 for potentially large sizes
    // Kilobytes
    pub used: i64,
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct PositionState {
    // 0:Not started, 1:Fixing, 2:Fixing successful, 3:Fixing failed
    pub is_fixed: i32,
    // 1-5: GPS quality, 10: RTK fixed
    pub quality: i32,
    pub gps_number: i32,
    pub rtk_number: i32,
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct PayloadGimbalInfo {
    // Degrees (-180 to 180)
    pub gimbal_pitch: f64,
    // Degrees (-180 to 180)
    pub gimbal_roll: f64,
    // Degrees (-180 to 180)
    pub gimbal_yaw: f64,
    // Degrees (-180 to 180)
    pub measure_target_longitude: f64,
    // Degrees (-90 to 90)
    pub measure_target_latitude: f64,
    // Meters
    pub measure_target_altitude: f64,
    // Meters
    pub measure_target_distance: f64,
    // 0:NORMAL, 1:TOO_CLOSE, 2:TOO_FAR, 3:NO_SIGNAL
    pub measure_target_error_state: i32,
    // Format: {type-subtype-gimbalindex} - can be redundant if used as key
    pub payload_index: String,
    // Specific zoom for this payload type if applicable
    pub zoom_factor: f64,
    // 0:WHITE_HOT, 1:BLACK_HOT, ...
    pub thermal_current_palette_style: i32,
    // Array of supported style codes
    pub thermal_supported_palette_styles: Vec<i32>,
    // 0:Auto, 1:Low Gain(0-500C), 2:High Gain(-20-150C)
    pub thermal_gain_mode: i32,
    // 0:Disable, 1:Enable
    pub thermal_isotherm_state: i32,
    // Celsius
    pub thermal_isotherm_upper_limit: i32,
    // Celsius
    pub thermal_isotherm_lower_limit: i32,
    // Celsius
    pub thermal_global_temperature_min: f64,
    // Celsius
    pub thermal_global_temperature_max: f64,
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct MaintainStatus {
    pub maintain_status_array: Vec<MaintainStatusDetail>,
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct MaintainStatusDetail {
    // 0:No maintenance, 1:With maintenance
    pub state: i32,
    // 1:Basic, 2:Regular, 3:Deep
    pub last_maintain_type: i32,
    // Unix timestamp in seconds
    pub last_maintain_time: i64, // Use i64 for timestamps
    // Hours
    pub last_maintain_flight_time: i32,
    pub last_maintain_flight_sorties: u32, // Use unsigned for counts
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct ObstacleAvoidance {
    // 0:Disable, 1:Enable
    pub horizon: i32,
    // 0:Disable, 1:Enable
    pub upside: i32,
    // 0:Disable, 1:Enable
    pub downside: i32,
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct PsdkUiResource {
    pub psdk_index: i32,
    // 0:Not ready, 1:Ready
    pub psdk_ready: i32,
    // OSS object key/path
    pub object_key: String,
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct PsdkWidgetValue {
    pub psdk_index: i32,
    pub psdk_name: String,
    pub psdk_sn: String,
    pub psdk_version: String,
    pub psdk_lib_version: String,
    pub speaker: Speaker,
    pub values: Vec<PsdkWidgetValueItem>,
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct Speaker {
    // 0:TTS payload mode, 1:Recording and speaking
    pub work_mode: i32,
    // 0:Single play, 1:Loop play (single track)
    pub play_mode: i32,
    // 0-100
    pub play_volume: i32,
    // 0:Idle, 1:Transmitting, 2:Playing, 3:Abnormal, 4:TTS converting, 99:Downloading
    pub system_state: i32,
    pub play_file_name: String,
    pub play_file_md5: String,
}

#[derive(Debug, Clone,  Serialize, Deserialize)]
pub struct PsdkWidgetValueItem {
    pub index: i32,
    // Value type depends on the widget, often int but could vary.
    // Using serde_json::Value allows flexibility.
    // If always integer, use i64 or similar.
    pub value: serde_json::Value,
}
