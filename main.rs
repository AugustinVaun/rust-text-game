use std::io;
fn main() {

    println!("Welcome to my text adventure game!");

    println!("It was Halloween night, my parents had just 
                left with my little brother to go to a party at the middle school.
                 I didn’t go because I was in the 11th grade and just spent the day drowning from essays and tests,
                  so I wasn’t in the mood to spend my night watching kids scream in terror from people in rubber masks
                   and styrofoam skeletons. Once they left I hopped on the couch and scrolled through the horror channels.
                    I eventually found a movie that seemed interesting “the not so good dead” directed by Sam Aimi.
                     I put the bright orange candy bowl on the porch and started to watch the movie, but I couldn’t focus,
                      I was too worried about my family.");

    println!("What do you do?");
    println!("1. Go to the front door and check on the candy bowl");
    println!("2. Go to the kitchen and check on the candy bowl");
    println!("3. Go to the bathroom and check on the candy bowl");
    println!("4. Go to the bedroom and check on the candy bowl");

    println!("Enter the number of your choice:");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = input.trim().parse()   
        .expect("Please type a number!");
        
    if input == 1 { 
        println!("You go to the front door and check on the candy bowl.");
        println!("unfortuenyul however you are not alone...");
        println!("You are now dead");
    }
    else if input ==2 {
        println!("You go to the kitchen and check on the candy bowl.");
        println!("Inside the candy bowl, you find keys to a car.");
        println!("You see that the vehicle is a 2019 Dodge Challenger SRT Hellcat Redeye.");
        println!("What should you do next?");
        println!("1. Drive the car");
        println!("2. Leave the car");
        println!("3. Go back to the movie");

        let mut input = String::new();
        
    }
    else if input ==3 {
        println!("You go to the bathroom and check on the candy bowl.");
    }
    else if input == 4 {
        println!("You go to the bedroom and check on the candy bowl.");
    }
        

}
