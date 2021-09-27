#![allow(unused)]
use chess_engine::{
    self,
    piece::Kind::{self, Bishop, King, Knight, Pawn, Queen, Rook},
    util::{
        self,
        Color::{Black, White},
    },
};

use bevy::prelude::*;

const WHITE_PAWN_SPRITE: &str = "big_pieces/wP.png";
const WHITE_ROOK_SPRITE: &str = "big_pieces/wR.png";
const WHITE_KNIGHT_SPRITE: &str = "big_pieces/wN.png";
const WHITE_BISHOP_SPRITE: &str = "big_pieces/wB.png";
const WHITE_KING_SPRITE: &str = "big_pieces/wK.png";
const WHITE_QUEEN_SPRITE: &str = "big_pieces/wQ.png";

const BLACK_PAWN_SPRITE: &str = "big_pieces/bP.png";
const BLACK_ROOK_SPRITE: &str = "big_pieces/bR.png";
const BLACK_KNIGHT_SPRITE: &str = "big_pieces/bN.png";
const BLACK_BISHOP_SPRITE: &str = "big_pieces/bB.png";
const BLACK_KING_SPRITE: &str = "big_pieces/bK.png";
const BLACK_QUEEN_SPRITE: &str = "big_pieces/bQ.png";

const BOARD_SPRITE: &str = "board.png";

const SCALE_FACTOR: f32 = 5. / 8.;

pub struct Materials {
    board: Handle<ColorMaterial>,
    white_pawn: Handle<ColorMaterial>,
    white_rook: Handle<ColorMaterial>,
    white_knight: Handle<ColorMaterial>,
    white_bishop: Handle<ColorMaterial>,
    white_king: Handle<ColorMaterial>,
    white_queen: Handle<ColorMaterial>,
    black_pawn: Handle<ColorMaterial>,
    black_rook: Handle<ColorMaterial>,
    black_knight: Handle<ColorMaterial>,
    black_bishop: Handle<ColorMaterial>,
    black_king: Handle<ColorMaterial>,
    black_queen: Handle<ColorMaterial>,
}

struct Piece;
struct Game {
    game: chess_engine::Game,
}

struct WindowSize {
    width: f32,
    height: f32,
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Tjack!".to_string(),
            width: 1024.,
            height: 1024.,
            resizable: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage(
            "game_setup_board",
            SystemStage::single(board_spawn.system()),
        )
        .add_startup_stage(
            "game_setup_pieces",
            SystemStage::single(pieces_spawn.system()),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    let mut window = windows.get_primary_mut().unwrap();
    let mut game = chess_engine::Game::new(chess_engine::Board::default());

    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // create the main resources
    commands.insert_resource(Materials {
        board: materials.add(asset_server.load(BOARD_SPRITE).into()),

        white_pawn: materials.add(asset_server.load(WHITE_PAWN_SPRITE).into()),
        white_rook: materials.add(asset_server.load(WHITE_ROOK_SPRITE).into()),
        white_knight: materials.add(asset_server.load(WHITE_KNIGHT_SPRITE).into()),
        white_bishop: materials.add(asset_server.load(WHITE_BISHOP_SPRITE).into()),
        white_king: materials.add(asset_server.load(WHITE_KING_SPRITE).into()),
        white_queen: materials.add(asset_server.load(WHITE_QUEEN_SPRITE).into()),

        black_pawn: materials.add(asset_server.load(BLACK_PAWN_SPRITE).into()),
        black_rook: materials.add(asset_server.load(BLACK_ROOK_SPRITE).into()),
        black_knight: materials.add(asset_server.load(BLACK_KNIGHT_SPRITE).into()),
        black_bishop: materials.add(asset_server.load(BLACK_BISHOP_SPRITE).into()),
        black_king: materials.add(asset_server.load(BLACK_KING_SPRITE).into()),
        black_queen: materials.add(asset_server.load(BLACK_QUEEN_SPRITE).into()),
    });

    commands.insert_resource(Game { game });

    commands.insert_resource(WindowSize {
        width: window.width(),
        height: window.height(),
    });
}

fn board_spawn(mut commands: Commands, materials: Res<Materials>, window_size: Res<WindowSize>) {
    // spawn background sprite

    commands.spawn_bundle(SpriteBundle {
        material: materials.board.clone(),
        transform: Transform {
            scale: Vec3::new(SCALE_FACTOR, SCALE_FACTOR, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn pieces_spawn(
    mut commands: Commands,
    materials: Res<Materials>,
    game: Res<Game>,
    window_size: Res<WindowSize>,
) {
    let tiles = game.game.board().tiles(); // TODO: REFACTOR
    let tile_size = window_size.width * SCALE_FACTOR / 8.;

    let mut i: usize = 0; // TODO: change to enumerate
    let mut j: usize = 0;

    let mut y: f32 = 290.;
    for row in tiles {
        let mut x: f32 = -280.;
        for tile in row {
            // tile is an Option<Piece>
            let piece = match tile {
                Some(p) => p,
                None => {
                    x += tile_size;
                    j += 1;
                    continue;
                }
            };
            let sprite = match_piece_to_sprite(piece.kind, piece.color, &materials);
            piece_spawn(&mut commands, sprite, x, y);
            x += tile_size;
            j += 1;
        }
        y -= tile_size + 1.;
        i += 1;
    }
}

fn piece_spawn(commands: &mut Commands, sprite: Handle<ColorMaterial>, x: f32, y: f32) {
    commands.spawn_bundle(SpriteBundle {
        material: sprite,
        transform: Transform {
            scale: Vec3::new(0.2, 0.2, 1.),
            translation: Vec3::new(x, y, 10.),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn match_piece_to_sprite(
    kind: Kind,
    color: util::Color,
    materials: &Res<Materials>,
) -> Handle<ColorMaterial> {
    match (kind, color) {
        (Pawn, White) => materials.white_pawn.clone(),
        (Rook, White) => materials.white_rook.clone(),
        (Knight, White) => materials.white_knight.clone(),
        (Bishop, White) => materials.white_bishop.clone(),
        (King, White) => materials.white_king.clone(),
        (Queen, White) => materials.white_queen.clone(),
        (Pawn, Black) => materials.black_pawn.clone(),
        (Rook, Black) => materials.black_rook.clone(),
        (Knight, Black) => materials.black_knight.clone(),
        (Bishop, Black) => materials.black_bishop.clone(),
        (King, Black) => materials.black_king.clone(),
        (Queen, Black) => materials.black_queen.clone(),
    }
}
