use ascii_utils::*;
use bevy::input::keyboard::*;
use bevy::prelude::*;
use bevy_ascii_terminal::*;

fn setup(mut commands: Commands) {
    let terminal = Terminal::new([20, 3]).with_border(Border::single_line());

    commands.spawn((
        // Spawn the terminal bundle from our terminal
        TerminalBundle::from(terminal),
        // Automatically set up the camera to render the terminal
        AutoCamera,
    ));
}

#[derive(Resource)]
struct CurrentString(String);

fn render_current_string(current_string: Res<CurrentString>, mut terminal: Query<&mut Terminal>) {
    for mut t in terminal.iter_mut() {
        t.clear_string([1, 1], 20);
        t.put_string([1, 1], current_string.0.to_string().fg(Color::WHITE));
    }
}

fn text_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut current_string: ResMut<CurrentString>,
) {
    for ev in char_evr.iter() {
        if ev.char.is_printable() {
            println!("Got char: '{}'", ev.char);
            current_string.0.push(ev.char);
        } else if ev.char as u8 == 8 {
            println!("Got backspace");
            current_string.0.pop();
        }
    }

    if keys.just_pressed(KeyCode::Return) {
        println!("Text input: {}", current_string.0);
        current_string.0.clear();
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .add_startup_system(setup)
        .insert_resource(CurrentString("".to_string()))
        .add_system(render_current_string)
        .add_system(text_input)
        .run();
}
