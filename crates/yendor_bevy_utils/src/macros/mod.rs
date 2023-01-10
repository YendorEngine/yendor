/// Switches the app state to the given state using iyes_loopless. This is a macro because it's
/// a bit more convenient to use.
///
/// # Usage
///
/// ```
/// pub struct AppState {
///     State1,
///     State2,
/// }
///
/// app.add_loopless_state(AppState::State1);
/// ...
/// switch_app_state!(commands, AppState::State2);
/// ```
#[macro_export]
macro_rules! switch_app_state {
    ($commands:ident, $s:expr) => {
        $commands.insert_resource(iyes_loopless::prelude::NextState($s))
    };

    ($e:expr) => {
        |mut commands: bevy::prelude::Commands| {
            debug!("Switching to state: {:?}", $e);
            commands.insert_resource(iyes_loopless::prelude::NextState($e));
        }
    };
}
