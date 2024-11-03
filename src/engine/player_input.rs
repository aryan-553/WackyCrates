use bevy::prelude::*;
use std::time::Duration;

#[derive(Debug)]
pub enum Direction {
    Left(Vec2),
    Right(Vec2),
}

#[derive(Event, Debug)]
pub enum PlayerInputs {
    Walk(Direction),
    Attack,
}

pub fn keyboard_input(

    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_move_event: EventWriter<PlayerInputs>,
    
) {
    let mut movement = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        movement.x -= 1.0;
        player_move_event.send(PlayerInputs::Walk(Direction::Left(movement)));
    } else if keyboard_input.pressed(KeyCode::ArrowRight) {
        movement.x += 1.0;
        player_move_event.send(PlayerInputs::Walk(Direction::Right(movement)));
    } else {
        player_move_event.send(PlayerInputs::Walk(Direction::Right(Vec2::ZERO)));
    }
    
    // if movement != Vec2::ZERO {
    //     player_move_event.send(PlayerInputs::Walk(movement));
    // } else {
    //     player_move_event.send(PlayerInputs::Walk(Vec2::ZERO));
    // }

    if keyboard_input.pressed(KeyCode::Space) {
        player_move_event.send(PlayerInputs::Attack);
    }
}