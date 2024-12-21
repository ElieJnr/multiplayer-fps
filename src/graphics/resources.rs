use bevy::prelude::*;

#[derive(Debug, Resource, Component, PartialEq, Eq, Clone, Copy, Default)]
pub enum Map {
    #[default]
    Map00,
    // Map01,
    // Map02,
}
