use crate::{ControlRequest, ControlResponse, GlobalPosition, LocalPosition, MissionPlan};
use macros::interface;
use serde::{Deserialize, Serialize};

interface!(ILocalPosition, "local_position", LocalPosition);
interface!(IGlobalPosition, "global_position", GlobalPosition);
interface!(IYaw, "yaw", f32);
interface!(IControlRequest, "control/in", ControlRequest);
interface!(IControlResponse, "control/out", ControlResponse);
interface!(IMissionStep, "mission/step", i32);
interface!(IMissionUpdate, "mission/update", MissionPlan);

pub trait Interface {
    type Message: Serialize + for<'a> Deserialize<'a>;
    fn topic() -> &'static str;
}

mod macros {
    macro_rules! interface {
        ($name:ident, $topic:expr, $typ:ty) => {
            pub struct $name;

            impl Interface for $name {
                type Message = $typ;

                fn topic() -> &'static str {
                    $topic
                }
            }
        };
    }
    pub(crate) use interface;
}
