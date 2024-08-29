use std::{
    fmt::Display,
    ops::{Add, Sub},
    time::Duration,
};

use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod interface;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum ControlRequest {
    FetchMissionPlan,
    PauseResume(bool),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum ControlResponse {
    SendMissionPlan(MissionPlan),
    PauseResume(bool),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MissionPlan {
    pub id: Uuid,
    pub nodes: Vec<MissionNode>,
    pub params: MissionParams,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MissionParams {
    pub target_velocity: Vector3<f64>,
    pub target_acceleration: Vector3<f64>,
    pub target_jerk: Vector3<f64>,
    pub disable_yaw: bool,
}

impl Display for MissionPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let id: String = self.id.to_string().chars().take(8).collect();
        let nds: Vec<String> = self.nodes.iter().map(|n| format!("{:?}", n.item)).collect();
        let nds = nds.join(", ");
        write!(f, "Mission {id} [{nds}]")
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct MissionNode {
    pub id: Uuid,
    pub item: MissionItem,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum MissionItem {
    Init,
    Takeoff { altitude: f64 },
    Waypoint(Waypoint),
    Delay(Duration),
    FindSafeSpot,
    Transition,
    Land,
    PrecLand,
    End,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Waypoint {
    LocalOffset(Vector3<f64>),
    GlobalFixedHeight {
        lat: f64,
        lon: f64,
        alt: f64,
    },
    GlobalRelativeHeight {
        lat: f64,
        lon: f64,
        height_diff: f64,
    },
}

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct LocalPosition {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl LocalPosition {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn expand(self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }

    pub fn to_nalgebra(self) -> Vector3<f32> {
        Vector3::new(self.x, self.y, self.z)
    }
}

impl Sub for LocalPosition {
    type Output = Vector3<f32>;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Add for LocalPosition {
    type Output = LocalPosition;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Display for LocalPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:.03}, {:.03}, {:.03}]", self.x, self.y, self.z)
    }
}

impl From<Vector3<f32>> for LocalPosition {
    fn from(value: Vector3<f32>) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct GlobalPosition {
    pub lat: f64,
    pub lon: f64,
    pub alt: f32,
}

impl Display for GlobalPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.06}, {:.06}, {:.06})", self.lat, self.lon, self.alt)
    }
}
