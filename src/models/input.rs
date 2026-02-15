use super::*;

#[derive(InputAction)]
#[action_output(bool)]
pub struct ZoomView;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Sprint;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Dash;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Pause;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Mute;

#[derive(InputAction)]
#[action_output(bool)]
pub struct Escape;

#[derive(InputAction)]
#[action_output(Vec2)]
pub struct NavigateModal;

/// Controller element select. F.e. for inventory cell
#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct Select;

/// Controller tab switch right
#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct CycleTab;

/// Controller tab switch left
#[derive(Debug, InputAction)]
#[action_output(bool)]
pub struct CycleTabBack;
