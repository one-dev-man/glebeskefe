mod turing;

use turing::machine::{Machine, self};

use crate::turing::band::{Band, BandDirection};

fn main() {
    let mut turing_machine = Machine::new(0);

    // 

    turing_machine.setAction(0, "1", |state: i32, symbol: &str| { return ("0", 1, BandDirection::RIGHT); } );
    
    turing_machine.setAction(1, "0", |state: i32, symbol: &str| { return ("0", 2, BandDirection::RIGHT); } );
    turing_machine.setAction(1, "1", |state: i32, symbol: &str| { return ("1", 1, BandDirection::RIGHT); } );
    
    turing_machine.setAction(2, "0", |state: i32, symbol: &str| { return ("0", 3, BandDirection::LEFT); } );
    turing_machine.setAction(2, "1", |state: i32, symbol: &str| { return ("1", 2, BandDirection::RIGHT); } );
    
    turing_machine.setAction(3, "0", |state: i32, symbol: &str| { return ("0", 4, BandDirection::LEFT); } );
    turing_machine.setAction(3, "1", |state: i32, symbol: &str| { return ("1", 3, BandDirection::LEFT); } );
    
    turing_machine.setAction(4, "0", |state: i32, symbol: &str| { return ("1", 0, BandDirection::RIGHT); } );
    turing_machine.setAction(4, "1", |state: i32, symbol: &str| { return ("1", 4, BandDirection::LEFT); } );
    

    // 

    turing_machine.band = Band::from(vec!["1", "1"]);

    turing_machine.run(|instance, step: i32| {
        let _band_state = instance.band.state();
    
        match _band_state {
            Some(band_state) => {
                println!("[{}] Result : {}", step + 1, band_state.toString());
                println!("{}", instance.band.readCursor());
                println!("{}", instance.band.readAt(-50));
                println!()
            },
            None => ()
        }
    });
}
