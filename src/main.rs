use domino_game::assets::DominoOnTable;
use ggez::input::keyboard;
use ggez::{ContextBuilder, Context, GameResult}; 
use ggez::conf::{Conf, WindowMode, WindowSetup};
use ggez::filesystem;
use ggez::event;
use ggez::mint::Point2;

use std::{env, path};

use domino_game::entities::main_state::MainState;


pub fn main() {
    let conf = Conf {  
        window_setup: WindowSetup { title: "Domino Game".to_owned(), ..Default::default() },
        window_mode: WindowMode {
            width: 1000.0,
            height: 1000.0,
            ..Default::default()
        },
        ..Default::default()
    };

    
    let (mut ctx, event_loop) = ContextBuilder::new("domino_game", "Vladimir").
        default_conf(conf.clone()).
        build().
        unwrap();
    
    
    
        // We add the CARGO_MANIFEST_DIR/resources do the filesystems paths so
    // we we look in the cargo project for files.
    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        filesystem::mount(&mut ctx, &path, true);
    }
    
  

    let mut state = MainState::new(&mut ctx, &conf).unwrap();

    // for i in 0..6 {
    //     println!("{:?}   === {:?} ",state.all_dominos[i].points, state.player_hand.hand[i].points);
    // }

    //state.game_board.dominos.push(DominoOnTable::new((0,1), 3, 0, 2.0).unwrap());
    state.game_board.dominos.push(DominoOnTable::new((4,4), 6, 7, 1.0).unwrap());
    //state.game_board.dominos.push(DominoOnTable::new((3,6), 7, 6, 3.0).unwrap());
   // state.game_board.dominos.push(DominoOnTable::new((2,3), 0, 1, 0.0).unwrap());


    for i in state.player_hand.hand.iter() {
        println!("{:?}", i);
    }
    event::run(ctx, event_loop, state);
}