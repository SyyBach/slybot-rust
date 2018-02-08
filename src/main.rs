extern crate discord;

use discord::Discord;
use discord::model::Event;
//use std::env;
use std::process::Command;
use std::process::Output;

use std::fs::File;



fn main() {
	// Log in to Discord using a bot token from the environment
	let discord = Discord::from_bot_token(
		//&env::var("DISCORD_TOKEN").expect("Expected token"),
        //dev token
        //"MjQ1MzEyMTk3NjYyNzM2Mzg0.DVaFMg.6utSoHmukhDT5q6vhSoSGlBT6QI"
        //release_token
        "MjQ1NjkyNDk5MzEyNjQwMDAw.DVaIPA.78UMRufnI3nC9FWZbk5eYrDonro"
	).expect("login failed");

	// Establish and use a websocket connection
	let (mut connection, _) = discord.connect().expect("connect failed");
	println!("Ready.");
	loop {
		match connection.recv_event() {
			Ok(Event::MessageCreate(message)) => {
				//println!("{} says: {}", message.author.name, message.content);
				if message.content == "!test" {
					let _ = discord.send_message(message.channel_id, "This is a reply to the test.", "", false);
				} else if message.content == "!quit" {
					println!("Quitting.");
					break
				} else if message.content.len()>=5 && &message.content[..5] == "./tex" {
                    println!("Calling compile_latex_formula.");
                    let output = compile_latex_formula(&message.content[6..]);
                    println!("status : {}", output.status);

                    println!("Calling pdf_to_png.");
                    let output = pdf_to_png("latex/formula.pdf");
                    println!("status : {}", output.status);
                    //println!("stdout : {}", String::from_utf8_lossy(&output.stdout));
                    //println!("stderr : {}", String::from_utf8_lossy(&output.stderr));

                    let mut file = File::open("latex/tmplatex.png")
                                    .expect("Couldn't open file latex/tmplatex.png");
					let _ = discord.send_file(message.channel_id,
                                              &format!("{}", message.author.mention()),
                                              file,
                                              "tmplatex.png");
                }
			}
			Ok(_) => {}
			Err(discord::Error::Closed(code, body)) => {
				println!("Gateway closed on us with code {:?}: {}", code, body);
				break
			}
			Err(err) => println!("Receive error: {:?}", err)
		}
	}
}



fn compile_latex_formula(formula : &str) -> Output {
    let flag_list = ["-interaction=nonstopmode",
                    "-halt-on-error",
                    "-jobname latex_formula",
                    "--output-directory=latex/"];

    let arg = String::from("\\def\\formula{")
                    +formula
                    +"}\\input{helpers/formula.tex}";

    Command::new("pdflatex")
        .args(&flag_list)
        .arg(arg)
        .output()
        .expect("failed to execute process pdflatex")
}


fn pdf_to_png(pdf_file : &str) -> Output {
    let flag_list = ["-density", "300", pdf_file, "-quality", "90", "latex/tmplatex.png"];

    Command::new("convert")
        .args(&flag_list)
        .output()
        .expect("failed to execute process convert")
}
