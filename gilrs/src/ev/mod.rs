// Copyright 2016-2018 Mateusz Sieczko and other GilRs Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Gamepad state and other event related functionality.

pub mod filter;
pub mod state;

use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    time::SystemTime,
};

use crate::{constants::*, gamepad::GamepadId, utils};

#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Serialize};

/// Platform specific event code.
///
/// This type represents single gamepads's element like specific axis or button.
/// It can't be directly created, but you can get it from events or using
/// `Gamepad`'s methods [`button_code`](crate::Gamepad::button_code) and
/// [`axis_code`](crate::Gamepad::axis_code). If `serde-serialize` feature is
/// enabled, `Code` can be serialized and deserialized, but keep in mind that
/// layout **is** platform-specific. So it's not possible to serialize `Code` on
/// Linux and deserialize it on Windows. This also apply to `Display` implementation.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub struct Code(pub(crate) gilrs_core::EvCode);

impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.0.fmt(f)
    }
}

impl Code {
    pub fn into_u32(&self) -> u32 {
        self.0.into_u32()
    }
}

/// Holds information about gamepad event.
#[derive(Copy, Clone, PartialEq, Debug)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct Event {
    /// Id of gamepad.
    pub id: GamepadId,
    /// Event's data.
    pub event: EventType,
    /// Time when event was emitted.
    pub time: SystemTime,
}

impl Event {
    /// Creates new event with current time.
    pub fn new(id: GamepadId, event: EventType) -> Self {
        Event {
            id,
            event,
            time: utils::time_now(),
        }
    }

    /// Creates new event with current time.
    pub fn new_with_time(id: GamepadId, event: EventType, time: SystemTime) -> Self {
        Event {
            id,
            event,
            time,
        }
    }

    /// Returns `Event` with `EventType::Dropped`.
    pub fn drop(mut self) -> Event {
        self.event = EventType::Dropped;

        self
    }

    /// Returns true if event is `Dropped` and should be ignored.
    pub fn is_dropped(&self) -> bool {
        self.event == EventType::Dropped
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
#[non_exhaustive]
/// Gamepad event.
pub enum EventType {
    /// Some button on gamepad has been pressed.
    ButtonPressed(Button, Code),
    /// This event can be generated by [`ev::Repeat`](filter/struct.Repeat.html) event filter.
    ButtonRepeated(Button, Code),
    /// Previously pressed button has been released.
    ButtonReleased(Button, Code),
    /// Value of button has changed. Value can be in range [0.0, 1.0].
    ButtonChanged(Button, f32, Code),
    /// Value of axis has changed. Value can be in range [-1.0, 1.0].
    AxisChanged(Axis, f32, Code),
    /// Gamepad has been connected. If gamepad's UUID doesn't match one of disconnected gamepads,
    /// newly connected gamepad will get new ID.
    Connected,
    /// Gamepad has been disconnected. Disconnected gamepad will not generate any new events.
    Disconnected,
    /// There was an `Event`, but it was dropped by one of filters. You should ignore it.
    Dropped,
    /// A force feedback effect has ran for its duration and stopped.
    ForceFeedbackEffectCompleted,
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
/// Gamepad's elements which state can be represented by value from 0.0 to 1.0.
///
/// ![Controller layout](https://gilrs-project.gitlab.io/gilrs/img/controller.svg)
pub enum Button {
    // Action Pad
    South = BTN_SOUTH,
    East = BTN_EAST,
    North = BTN_NORTH,
    West = BTN_WEST,
    C = BTN_C,
    Z = BTN_Z,
    // Triggers
    LeftTrigger = BTN_LT,
    LeftTrigger2 = BTN_LT2,
    RightTrigger = BTN_RT,
    RightTrigger2 = BTN_RT2,
    // Menu Pad
    Select = BTN_SELECT,
    Start = BTN_START,
    Mode = BTN_MODE,
    // Sticks
    LeftThumb = BTN_LTHUMB,
    RightThumb = BTN_RTHUMB,
    // D-Pad
    DPadUp = BTN_DPAD_UP,
    DPadDown = BTN_DPAD_DOWN,
    DPadLeft = BTN_DPAD_LEFT,
    DPadRight = BTN_DPAD_RIGHT,

    #[default]
    Unknown = BTN_UNKNOWN,
}

impl Button {
    pub fn is_action(self) -> bool {
        use crate::Button::*;
        matches!(self, South | East | North | West | C | Z)
    }

    pub fn is_trigger(self) -> bool {
        use crate::Button::*;
        matches!(
            self,
            LeftTrigger | LeftTrigger2 | RightTrigger | RightTrigger2
        )
    }

    pub fn is_menu(self) -> bool {
        use crate::Button::*;
        matches!(self, Select | Start | Mode)
    }

    pub fn is_stick(self) -> bool {
        use crate::Button::*;
        matches!(self, LeftThumb | RightThumb)
    }

    pub fn is_dpad(self) -> bool {
        use crate::Button::*;
        matches!(self, DPadUp | DPadDown | DPadLeft | DPadRight)
    }

    pub fn to_nec(self) -> Option<Code> {
        use gilrs_core::native_ev_codes as necs;

        match self {
            Button::South => Some(necs::BTN_SOUTH),
            Button::East => Some(necs::BTN_EAST),
            Button::North => Some(necs::BTN_NORTH),
            Button::West => Some(necs::BTN_WEST),
            Button::C => Some(necs::BTN_C),
            Button::Z => Some(necs::BTN_Z),
            Button::LeftTrigger => Some(necs::BTN_LT),
            Button::LeftTrigger2 => Some(necs::BTN_LT2),
            Button::RightTrigger => Some(necs::BTN_RT),
            Button::RightTrigger2 => Some(necs::BTN_RT2),
            Button::Select => Some(necs::BTN_SELECT),
            Button::Start => Some(necs::BTN_START),
            Button::Mode => Some(necs::BTN_MODE),
            Button::LeftThumb => Some(necs::BTN_LTHUMB),
            Button::RightThumb => Some(necs::BTN_RTHUMB),
            Button::DPadUp => Some(necs::BTN_DPAD_UP),
            Button::DPadDown => Some(necs::BTN_DPAD_DOWN),
            Button::DPadLeft => Some(necs::BTN_DPAD_LEFT),
            Button::DPadRight => Some(necs::BTN_DPAD_RIGHT),
            _ => None,
        }
        .map(Code)
    }
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
/// Gamepad's elements which state can be represented by value from -1.0 to 1.0.
///
/// ![Controller layout](https://gilrs-project.gitlab.io/gilrs/img/controller.svg)
pub enum Axis {
    LeftStickX = AXIS_LSTICKX,
    LeftStickY = AXIS_LSTICKY,
    LeftZ = AXIS_LEFTZ,
    RightStickX = AXIS_RSTICKX,
    RightStickY = AXIS_RSTICKY,
    RightZ = AXIS_RIGHTZ,
    DPadX = AXIS_DPADX,
    DPadY = AXIS_DPADY,
    Unknown = AXIS_UNKNOWN,
}

impl Axis {
    /// Returns true if axis is `LeftStickX`, `LeftStickY`, `RightStickX` or `RightStickY`.
    pub fn is_stick(self) -> bool {
        use crate::Axis::*;
        matches!(self, LeftStickX | LeftStickY | RightStickX | RightStickY)
    }

    /// Returns the other axis from same element of gamepad, if any.
    ///
    /// | input       | output            |
    /// |-------------|-------------------|
    /// |`LeftStickX` |`Some(LeftStickY)` |
    /// |`LeftStickY` |`Some(LeftStickX)` |
    /// |`RightStickX`|`Some(RightStickY)`|
    /// |`RightStickY`|`Some(RightStickX)`|
    /// |`DpadX`      |`Some(DpadY)`      |
    /// |`DpadY`      |`Some(DpadX)`      |
    /// | …           |`None`             |
    pub fn second_axis(self) -> Option<Self> {
        use crate::Axis::*;
        match self {
            LeftStickX => Some(LeftStickY),
            LeftStickY => Some(LeftStickX),
            RightStickX => Some(RightStickY),
            RightStickY => Some(RightStickX),
            DPadX => Some(DPadY),
            DPadY => Some(DPadX),
            _ => None,
        }
    }
}

/// Represents `Axis` or `Button`.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde-serialize", derive(Serialize, Deserialize))]
pub enum AxisOrBtn {
    Axis(Axis),
    Btn(Button),
}

impl AxisOrBtn {
    pub(crate) fn is_button(&self) -> bool {
        matches!(self, AxisOrBtn::Btn(_))
    }
}
