use crate::utils;
use std::io::Cursor;
use serde::{Serialize, Deserialize};
use serde_json;

/// dash datagram
#[derive(Serialize, Deserialize, Debug)]
pub struct Datagram {
    /// = 1 when race is on. = 0 when in menus/race stopped<br />
    /// 在菜单中是0，比赛中是1
    pub is_race_on: i32,

    // Can overflow to 0 eventually
    pub timestamp_ms: u32,
    /// 发动机断油转速
    pub engine_max_rpm: f32,
    /// 发动机怠速转速
    pub engine_idle_rpm: f32,
    /// 当前发动机转速
    pub current_engine_rpm: f32,

    // In the car's local space; X = right, Y = up, Z = forward
    pub acceleration_x: f32,
    pub acceleration_y: f32,
    pub acceleration_z: f32,

    // In the car's local space; X = right, Y = up, Z = forward
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub velocity_z: f32,

    // In the car's local space; X = right, Y = up, Z = forward
    pub angular_velocity_x: f32,
    pub angular_velocity_y: f32,
    pub angular_velocity_z: f32,

    pub yaw: f32,
    pub pitch: f32,
    pub roll: f32,

    // Suspension travel normalized: 0.0f = max stretch; 1.0 = max compression
    pub normalized_suspension_travel_front_left: f32,
    pub normalized_suspension_travel_front_right: f32,
    pub normalized_suspension_travel_rear_left: f32,
    pub normalized_suspension_travel_rear_right: f32,

    // Tire normalized slip ratio, = 0 means 100% grip and |ratio| > 1.0 means loss of grip.
    pub tire_slip_ratio_front_left: f32,
    pub tire_slip_ratio_front_right: f32,
    pub tire_slip_ratio_rear_left: f32,
    pub tire_slip_ratio_rear_right: f32,

    // Wheels rotation speed radians/sec<br />
    // 车轮转速
    pub wheel_rotation_speed_front_left: f32,
    pub wheel_rotation_speed_front_right: f32,
    pub wheel_rotation_speed_rear_left: f32,
    pub wheel_rotation_speed_rear_right: f32,

    // = 1 when wheel is on rumble strip, = 0 when off.<br />
    // 车轮是否接触到赛道边缘的凹凸不平区域，可能用于横量赛道中悬挂调教
    pub wheel_on_rumble_strip_front_left: i32,
    pub wheel_on_rumble_strip_front_right: i32,
    pub wheel_on_rumble_strip_rear_left: i32,
    pub wheel_on_rumble_strip_rear_right: i32,

    // = from 0 to 1, where 1 is the deepest puddle<br />
    // 车轮在水坑中的深度，应该是判断赛道积水量的相关指标
    pub wheel_in_puddle_depth_front_left: f32,
    pub wheel_in_puddle_depth_front_right: f32,
    pub wheel_in_puddle_depth_rear_left: f32,
    pub wheel_in_puddle_depth_rear_right: f32,

    // Non-dimensional surface rumble values passed to controller force feedback<br />
    // 车轮所在路面颠簸情况，是一个无单位的值，似乎是用于类似手柄之类设备的反馈
    pub surface_rumble_front_left: f32,
    pub surface_rumble_front_right: f32,
    pub surface_rumble_rear_left: f32,
    pub surface_rumble_rear_right: f32,

    // Tire normalized slip angle, = 0 means 100% grip and |angle| > 1.0 means loss of grip.
    pub tire_slip_angle_front_left: f32,
    pub tire_slip_angle_front_right: f32,
    pub tire_slip_angle_rear_left: f32,
    pub tire_slip_angle_rear_right: f32,

    // Tire normalized combined slip, = 0 means 100% grip and |slip| > 1.0 means loss of grip.
    pub tire_combined_slip_front_left: f32,
    pub tire_combined_slip_front_right: f32,
    pub tire_combined_slip_rear_left: f32,
    pub tire_combined_slip_rear_right: f32,

    // Actual suspension travel in meters
    pub suspension_travel_meters_front_left: f32,
    pub suspension_travel_meters_front_right: f32,
    pub suspension_travel_meters_rear_left: f32,
    pub suspension_travel_meters_rear_right: f32,

    // Unique ID of the car make/model
    pub car_ordinal: i32,

    // Between 0 (D -- worst cars) and 7 (X class -- best cars) inclusive
    pub car_class: i32,

    // Between 100 (worst car) and 999 (best car) inclusive
    pub car_performance_index: i32,

    // 0 = FWD, 1 = RWD, 2 = AWD
    pub drivetrain_type: i32,

    // Number of cylinders in the engine
    pub num_cylinders: i32,
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    /// meter pre sec<br />
    /// 米/每秒，需要根据需要转为为KM/H或者MPH
    pub speed: f32,
    /// watts
    /// 瓦特
    pub power: f32,
    /// 扭矩，单位：牛顿/米
    pub torque: f32,
    /// 左前轮华氏度
    pub tire_temp_front_left: f32,
    /// 右前轮华氏度
    pub tire_temp_front_right: f32,
    /// 左后轮华氏度
    pub tire_temp_rear_left: f32,
    /// 右后轮华氏度
    pub tire_temp_rear_right: f32,
    pub boost: f32,
    pub fuel: f32,
    pub distance_traveled: f32,
    /// 个人最快圈速，单位：秒
    pub best_lap: f32,
    /// 上一个单圈圈速，单位：秒
    pub last_lap: f32,
    /// 当前单圈圈速，单位：秒
    pub current_lap: f32,
    /// 当前比赛耗时，单位：秒
    pub current_race_time: f32,
    /// 目前跑圈，0代表第一圈
    pub lap_number: u16,
    pub race_position: u8,
    pub accel: u8,
    pub brake: u8,
    pub clutch: u8,
    pub hand_brake: u8,
    /// 当前挡位
    pub gear: u8,
    pub steer: i8,
    pub normalized_driving_line: i8,
    pub normalized_aibrake_difference: i8,

    pub tire_wear_front_left: f32,
    pub tire_wear_front_right: f32,
    pub tire_wear_rear_left: f32,
    pub tire_wear_rear_right: f32,

    /// ID for track，赛道布局对应的ID
    pub track_ordinal: i32,
}

impl Datagram {
    /// `get_speed_by_kmh` m/S => KM/H
    /// 将速度转换为KM/H
    pub fn get_speed_by_kmh(&self) -> f32 {
        self.speed * 3.6
    }

    /// `get_speed_by_mph` m/S => Mph
    /// 将速度转换为MPH
    pub fn get_speed_by_mph(&self) -> f32 {
        self.speed * 2.2369
    }

    pub fn to_json(&self) -> String { serde_json::to_string(&self).unwrap() }
    
}

/// `parse`
/// parse dash format upd packet datagram<br />
/// 解析Dash格式的UPD报文数据
/// # Example
/// ```rust
/// use forza_dataout_parse::dash_parser;
///
/// let datagram = dash_parser::parse(&udp_buf);
/// ```
pub fn parse(buf: &[u8]) -> Datagram {
    // is race on
    let mut cursor = Cursor::new(buf);

    Datagram {
        is_race_on: utils::parse_i32(&mut cursor),

        timestamp_ms: utils::parse_u32(&mut cursor),
        engine_max_rpm: utils::parse_f32(&mut cursor),
        engine_idle_rpm: utils::parse_f32(&mut cursor),
        current_engine_rpm: utils::parse_f32(&mut cursor),

        acceleration_x: utils::parse_f32(&mut cursor),
        acceleration_y: utils::parse_f32(&mut cursor),
        acceleration_z: utils::parse_f32(&mut cursor),

        velocity_x: utils::parse_f32(&mut cursor),
        velocity_y: utils::parse_f32(&mut cursor),
        velocity_z: utils::parse_f32(&mut cursor),

        angular_velocity_x: utils::parse_f32(&mut cursor),
        angular_velocity_y: utils::parse_f32(&mut cursor),
        angular_velocity_z: utils::parse_f32(&mut cursor),

        yaw: utils::parse_f32(&mut cursor),
        pitch: utils::parse_f32(&mut cursor),
        roll: utils::parse_f32(&mut cursor),

        normalized_suspension_travel_front_left: utils::parse_f32(&mut cursor),
        normalized_suspension_travel_front_right: utils::parse_f32(&mut cursor),
        normalized_suspension_travel_rear_left: utils::parse_f32(&mut cursor),
        normalized_suspension_travel_rear_right: utils::parse_f32(&mut cursor),

        tire_slip_ratio_front_left: utils::parse_f32(&mut cursor),
        tire_slip_ratio_front_right: utils::parse_f32(&mut cursor),
        tire_slip_ratio_rear_left: utils::parse_f32(&mut cursor),
        tire_slip_ratio_rear_right: utils::parse_f32(&mut cursor),

        wheel_rotation_speed_front_left: utils::parse_f32(&mut cursor),
        wheel_rotation_speed_front_right: utils::parse_f32(&mut cursor),
        wheel_rotation_speed_rear_left: utils::parse_f32(&mut cursor),
        wheel_rotation_speed_rear_right: utils::parse_f32(&mut cursor),

        wheel_on_rumble_strip_front_left: utils::parse_i32(&mut cursor),
        wheel_on_rumble_strip_front_right: utils::parse_i32(&mut cursor),
        wheel_on_rumble_strip_rear_left: utils::parse_i32(&mut cursor),
        wheel_on_rumble_strip_rear_right: utils::parse_i32(&mut cursor),

        wheel_in_puddle_depth_front_left: utils::parse_f32(&mut cursor),
        wheel_in_puddle_depth_front_right: utils::parse_f32(&mut cursor),
        wheel_in_puddle_depth_rear_left: utils::parse_f32(&mut cursor),
        wheel_in_puddle_depth_rear_right: utils::parse_f32(&mut cursor),

        surface_rumble_front_left: utils::parse_f32(&mut cursor),
        surface_rumble_front_right: utils::parse_f32(&mut cursor),
        surface_rumble_rear_left: utils::parse_f32(&mut cursor),
        surface_rumble_rear_right: utils::parse_f32(&mut cursor),

        tire_slip_angle_front_left: utils::parse_f32(&mut cursor),
        tire_slip_angle_front_right: utils::parse_f32(&mut cursor),
        tire_slip_angle_rear_left: utils::parse_f32(&mut cursor),
        tire_slip_angle_rear_right: utils::parse_f32(&mut cursor),

        tire_combined_slip_front_left: utils::parse_f32(&mut cursor),
        tire_combined_slip_front_right: utils::parse_f32(&mut cursor),
        tire_combined_slip_rear_left: utils::parse_f32(&mut cursor),
        tire_combined_slip_rear_right: utils::parse_f32(&mut cursor),

        suspension_travel_meters_front_left: utils::parse_f32(&mut cursor),
        suspension_travel_meters_front_right: utils::parse_f32(&mut cursor),
        suspension_travel_meters_rear_left: utils::parse_f32(&mut cursor),
        suspension_travel_meters_rear_right: utils::parse_f32(&mut cursor),

        car_ordinal: utils::parse_i32(&mut cursor),

        car_class: utils::parse_i32(&mut cursor),

        car_performance_index: utils::parse_i32(&mut cursor),

        drivetrain_type: utils::parse_i32(&mut cursor),

        num_cylinders: utils::parse_i32(&mut cursor),
        position_x: utils::parse_f32(&mut cursor),
        position_y: utils::parse_f32(&mut cursor),
        position_z: utils::parse_f32(&mut cursor),
        speed: utils::parse_f32(&mut cursor),
        power: utils::parse_f32(&mut cursor),
        torque: utils::parse_f32(&mut cursor),
        tire_temp_front_left: utils::parse_f32(&mut cursor),
        tire_temp_front_right: utils::parse_f32(&mut cursor),
        tire_temp_rear_left: utils::parse_f32(&mut cursor),
        tire_temp_rear_right: utils::parse_f32(&mut cursor),
        boost: utils::parse_f32(&mut cursor),
        fuel: utils::parse_f32(&mut cursor),
        distance_traveled: utils::parse_f32(&mut cursor),
        best_lap: utils::parse_f32(&mut cursor),
        last_lap: utils::parse_f32(&mut cursor),
        current_lap: utils::parse_f32(&mut cursor),
        current_race_time: utils::parse_f32(&mut cursor),
        lap_number: utils::parse_u16(&mut cursor),
        race_position: utils::parse_u8(&mut cursor),
        accel: utils::parse_u8(&mut cursor),
        brake: utils::parse_u8(&mut cursor),
        clutch: utils::parse_u8(&mut cursor),
        hand_brake: utils::parse_u8(&mut cursor),
        gear: utils::parse_u8(&mut cursor),
        steer: utils::parse_i8(&mut cursor),
        normalized_driving_line: utils::parse_i8(&mut cursor),
        normalized_aibrake_difference: utils::parse_i8(&mut cursor),

        tire_wear_front_left: utils::parse_f32(&mut cursor),
        tire_wear_front_right: utils::parse_f32(&mut cursor),
        tire_wear_rear_left: utils::parse_f32(&mut cursor),
        tire_wear_rear_right: utils::parse_f32(&mut cursor),

        track_ordinal:  utils::parse_i32(&mut cursor),
    }
}
