use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.")
	.author("kyclark@gmail.com")
	.about("Rust Echo")
	.arg(
	    Arg::with_name("text")
	        .value_name("TEXT")
		.help("Input text")
		.required(true)
		.min_values(1),
	)
	.arg(
            Arg::with_name("omit_newline")
	        .short("n")
		.help("Do not print newline")
		.takes_value(false),
	)
	.get_matches();

    //println!("{:#?}", matches);
    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");

    let ending = if omit_newline { "" } else { "\n" };
    print!("{}{}", text.join(" "), ending);
}
