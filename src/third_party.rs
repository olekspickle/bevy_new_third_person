use crate::*;
use bevy_ahoy::prelude::*;
use bevy_fix_cursor_unlock_web::prelude::*;
#[cfg(feature = "third_person")]
pub use bevy_third_person_camera::*;
#[cfg(feature = "top_down")]
pub use bevy_top_down_camera::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        FixPointerUnlockPlugin,
        #[cfg(feature = "native")]
        SeedlingPlugin::default(),
        #[cfg(feature = "web")]
        SeedlingPlugin::new_web_audio(),
        EnhancedInputPlugin,
        SkeinPlugin::default(),
        PhysicsPlugins::default(),
        AhoyPlugins::default(),
    ));

    #[cfg(feature = "third_person")]
    app.add_plugins(ThirdPersonCameraPlugin).configure_sets(
        PostUpdate,
        bevy_third_person_camera::CameraSyncSet.before(TransformSystems::Propagate),
    );
    #[cfg(feature = "top_down")]
    app.add_plugins(TopDownCameraPlugin).configure_sets(
        PostUpdate,
        bevy_top_down_camera::CameraSyncSet.before(TransformSystems::Propagate),
    );
}
