fn drop_header(input: &str) -> &str {
    let location = input.find(": ").unwrap_or(0)+3 as usize;
    &input[location-1..]
}

fn parse_game(input: &str) -> Vec<&str> {
    let input = drop_header(input);
    let sets = input.split("; ");
    let colours_revealed = sets.map(|s| s.split(", ")).flatten();
    colours_revealed.collect()
}


fn collect_colour(colour_list: Vec<&str>, colour: &str) -> Vec<u32> {
    colour_list
        .into_iter()
        .filter(|s| s.contains(colour))
        .map(|s| s[..1].parse().unwrap_or(0))
        .collect()
}

struct Colours {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let mut state = Colours {
        red: 0,
        green: 0,
        blue: 0,
    };
    let colour_strings = parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
    println!("{:?}", collect_colour(colour_strings,"red"));
}
