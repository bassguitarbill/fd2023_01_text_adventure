use ascii_utils::*;
use bevy::input::keyboard::*;
use bevy::prelude::*;
use bevy_ascii_terminal::*;

const TERMINAL_WIDTH: usize = 40;
const TERMINAL_HEIGHT: usize = 20;

const TERMINAL_PRINTABLE_HEIGHT: usize = TERMINAL_HEIGHT - 4;

const PROMPT: &str = "> ";
const PROMPT_WIDTH: usize = PROMPT.len();
const CURRENT_STRING_WIDTH: usize = TERMINAL_WIDTH - PROMPT_WIDTH;

fn setup(mut commands: Commands) {
    let terminal = Terminal::new([TERMINAL_WIDTH, TERMINAL_HEIGHT]);

    commands.spawn((
        // Spawn the terminal bundle from our terminal
        TerminalBundle::from(terminal),
        // Automatically set up the camera to render the terminal
        AutoCamera,
    ));
}

#[derive(Resource)]
struct CurrentString(String);

#[derive(Resource)]
struct Lines(Vec<String>);

fn render_lines(mut terminal: Query<&mut Terminal>, lines: Res<Lines>) {
    if lines.0.len() < TERMINAL_PRINTABLE_HEIGHT {
        for i in 0..lines.0.len() {
            for mut t in terminal.iter_mut() {
                t.clear_string([1, TERMINAL_PRINTABLE_HEIGHT - i + 2], TERMINAL_WIDTH);
                t.put_string(
                    [1, TERMINAL_PRINTABLE_HEIGHT - i + 2],
                    lines.0.get(i).unwrap(),
                );
            }
        }
    } else {
        for i in 0..TERMINAL_PRINTABLE_HEIGHT {
            let line_to_print = lines.0.len() - TERMINAL_PRINTABLE_HEIGHT + i;
            for mut t in terminal.iter_mut() {
                t.clear_string([1, TERMINAL_PRINTABLE_HEIGHT - i + 2], TERMINAL_WIDTH);
                t.put_string(
                    [1, TERMINAL_PRINTABLE_HEIGHT - i + 2],
                    lines.0.get(line_to_print).unwrap(),
                );
            }
        }
    }
}

fn render_current_string(current_string: Res<CurrentString>, mut terminal: Query<&mut Terminal>) {
    for mut t in terminal.iter_mut() {
        t.clear_string([1, 1], TERMINAL_WIDTH);
        t.put_string([1, 1], PROMPT.fg(Color::YELLOW));
        t.put_string(
            [PROMPT_WIDTH + 1, 1],
            current_string.0.to_string().fg(Color::WHITE),
        );
    }
}

fn text_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut current_string: ResMut<CurrentString>,
    mut lines: ResMut<Lines>,
) {
    for ev in char_evr.iter() {
        if current_string.0.len() > CURRENT_STRING_WIDTH - 4 {
        } else if ev.char.is_printable() {
            println!("Got char: '{}'", ev.char);
            current_string.0.push(ev.char);
        } else if ev.char as u8 == 8 {
            println!("Got backspace");
            current_string.0.pop();
        }
    }

    if keys.just_pressed(KeyCode::Return) {
        println!("Text input: {}", current_string.0);
        lines.0.push((&current_string.0).to_string());
        current_string.0.clear();
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerminalPlugin)
        .add_startup_system(setup)
        .insert_resource(CurrentString("".to_string()))
        .insert_resource(Lines(vec![]))
        .add_system(render_current_string)
        .add_system(render_lines)
        .add_system(text_input)
        .run();
}
