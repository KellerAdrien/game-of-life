/*
* Filename: main.rs
* Path: c:\Users\Adrien\Documents\code\game-of-life\src
* Created Date: Tuesday, May 23rd 2023, 4:19:32 pm
* Author: Adrien Keller
* 
*/

use druid::widget::{Flex, Label};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WindowDesc, WidgetExt};

mod world;
use world::World;

mod cell;


const GRID_SIZE : u16 = 20;
const WINDOW_TITLE: LocalizedString<HelloState> = LocalizedString::new("Conway's Game of Life");

#[derive(Clone, Data, Lens)]
struct HelloState {
    name: String,
}

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_grid())
    .title(WINDOW_TITLE)
    .window_size((420.0, 420.0));
    
    // create the initial app state
    let initial_state = HelloState {
        name: "World".into(),
    };
    
    let mut world = World::new(GRID_SIZE);
    world.create_grid();
    println!("{:?}", world.map());

    // start the application
    AppLauncher::with_window(main_window)
    .launch(initial_state)
    .expect("Failed to launch application");
}

fn build_grid() -> impl Widget<HelloState> {
    let mut grid = Flex::column();
    
    for _ in 0..GRID_SIZE as u16 {
        let mut row = Flex::row();
        for _ in 0..GRID_SIZE as u16 {
            row.add_child(
                Label::new("o").padding(1.0)
                .border(druid::Color::BLACK, 1.0)
                .fix_size(GRID_SIZE as f64, GRID_SIZE as f64)
            );
        }
        grid.add_child(row);
    }
    
    grid
}

fn update_grid() {
    
}

// fn build_root_widget() -> impl Widget<HelloState> {
//     // a label that will determine its text based on the current app data.
//     let label = Label::new(|data: &HelloState, _env: &Env| format!("Hello {}!", data.name));
//     // a textbox that modifies `name`.
//     let textbox = TextBox::new()
//     .with_placeholder("Who are we greeting?")
//     .fix_width(TEXT_BOX_WIDTH)
//     .lens(HelloState::name);
    
//     // arrange the two widgets vertically, with some padding
//     let layout = Flex::column()
//     .with_child(label)
//     .with_spacer(VERTICAL_WIDGET_SPACING)
//     .with_child(textbox);
    
//     // center the two widgets in the available space
//     Align::centered(layout)
// }