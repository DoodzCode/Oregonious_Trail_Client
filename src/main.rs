extern crate chrome;

fn main() {

    #[derive(Debug)]
    struct Leg {
        // key: Something, // Unique ID
        name: String,
        // territory: Territory, // Missouri
        // biome: Biome;
        distance: i32,  // 80
        guide_note: String, // "west by north to Kansas River Crossing"
    }

    impl Leg {
        pub fn create() -> Leg {
            Leg{name: "Test".to_string(), distance: 10, guide_note: "Test Note".to_string() }
        }
    }

    struct Game {
        start_date: String,
        legs: Vec<Leg>,
        // locations: Locations,

    }

    impl Game {
        fn new() -> Game {
            let mut legs: Vec<Leg> = Vec::new();
            legs.push(Leg::create());
            legs.push(Leg::create());
            legs.push(Leg::create());

            Game { start_date: "04/15/1844".to_string(), legs: legs }
        }
    }



    // startup
    println!("STARTUP");
    let mut game: Game = Game::new();


    
    // let first_leg = Leg{name: "Test".to_string(), distance: 10, guide_note: "Test Note".to_string() };
    // println!("{:?}", legs[1]);
    
    let mut b: String = "test".to_string();
    println!("B: {}", b);

    b = stuff(&b);
    println!("B: {}", b);

    
    
    // main loop
    let mut i:i32 = 0;    
    loop {
        i = i+1;
        if i > 26 {
            break;
        }
        update(&i, &game.start_date);
    }

    // shutdown
    
}

fn stuff(thing: &String) -> String {
    println!("{}", thing);
    let new_thing: String = thing.clone();
    new_thing
}

fn update(i: &i32, start_date: &String) {

    // Update Game Date
    let current_date = start_date + (i * 7);
    // let current_date = start_date + "00".to_string();

}






