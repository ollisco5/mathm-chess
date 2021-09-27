#![allow(unused)]

use bevy::prelude::*;

const WHITE_PAWN_SPRITE: &str = "wP.png";

const BOARD_SPRITE: &str = "board.jpg";

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(1., 1., 1.)))
        .insert_resource(WindowDescriptor {
            title: "Tjack!".to_string(),
            width: 800.,
            height: 800.,
            resizable: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .run();
}