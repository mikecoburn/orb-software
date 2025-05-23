use orb_update_agent_core::{Component, Slot};

pub mod can;
pub mod capsule;
pub mod gpt;
pub mod raw;

pub trait Update {
    fn update<R>(&self, slot: Slot, src: R) -> eyre::Result<()>
    where
        R: std::io::Read + std::io::Seek;
}

impl Update for Component {
    fn update<R>(&self, slot: Slot, src: R) -> eyre::Result<()>
    where
        R: std::io::Read + std::io::Seek,
    {
        match self {
            Component::Can(c) => c.update(slot, src),
            Component::Gpt(c) => c.update(slot, src),
            Component::Raw(c) => c.update(slot, src),
            Component::Capsule(c) => c.update(slot, src),
        }
    }
}

#[cfg(test)]
mod tests;

pub use can::{try_mcu_set_static_fan_speed, RECOVERY_STATIC_FAN_SPEED_PERCENTAGE};
