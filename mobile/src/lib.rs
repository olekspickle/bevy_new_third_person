use bevy::{prelude::*, window::WindowMode, winit::WinitSettings};
use bevy_new_third_person::game; // ToDo: Replace with your new crate name.

#[bevy_main]
fn main() {
    #[cfg(target_os = "ios")]
    unsafe {
        // Sets our audio session to Ambient mode to prevent background music from stopping.
        // The default for iOS apps is SoloAmbient, which stops background music.
        // See apple docs: https://developer.apple.com/documentation/avfaudio/avaudiosession/category-swift.struct/ambient
        if let Err(e) = objc2_avf_audio::AVAudioSession::sharedInstance()
            .setCategory_error(objc2_avf_audio::AVAudioSessionCategoryAmbient.unwrap())
        {
            println!("Error setting audio session category: {:?}", e);
        }
    }

    App::new()
        .insert_resource(WinitSettings::mobile())
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                    // on iOS, gestures must be enabled.
                    #[cfg(target_os = "ios")]
                    recognize_rotation_gesture: true,
                    #[cfg(target_os = "ios")]
                    recognize_pinch_gesture: true,
                    ..default()
                }),
                ..default()
            }),
            game,
        ))
        .run();
}
