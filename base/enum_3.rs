enum ThingsInTheSky {
    Sun(String),
    Stars(String),
}

fn create_skystate(time: i32) -> ThingsInTheSky {
    match time {
        6..=18 => ThingsInTheSky::Sun("I can see the sun!".to_string()),
	_ => ThingsInTheSky::Stars("I can see the stars!".to_string()),
    }
}

fn check_skystate(state: &ThingsInTheSky) {
    match state {
        ThingsInTheSky::Sun(description) => println!("{description}"),
	ThingsInTheSky::Stars(n) => println!("{}", n),
    }
}

fn main() {
    let time = 8;
    let skystate = create_skystate(time);
    check_skystate(&skystate);
}
