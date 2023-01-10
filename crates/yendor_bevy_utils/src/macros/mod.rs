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
