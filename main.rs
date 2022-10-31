use std::io;

fn funnyguy() {
    println!("Oh so you think you can weasel your way out of this, huh? Well, I am God here, and for your insolence, I cast you into Oblivion!")
}

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
        println!("unfortunately however you are not alone...");
        println!("You feel a cold drop run down your back, and then warm blood gush out of the stab wound.");
        println!("You are now dead");
        main()
    }
    else if input ==2 {
        println!("You go to the kitchen and check on the candy bowl.");
        println!("Inside the candy bowl, you find keys to a car.");
        println!("You see that the vehicle is a 2019 Dodge Challenger SRT Hellcat Redeye.");
        println!("What should you do next?");
        println!("1. Drive the car");
        println!("2. Leave the car");
        println!("3. Go back to the movie");
        
    }
    else if input ==3 {
        println!("You go to the bathroom and check on the candy bowl.");
        println!("You realize that there is no candy bowl in the bathroom.");
        println!("You decide to go back to the living room in front of the TV.");
        main()

    }
    else if input == 4 {
        println!("You go to the bedroom and check on the candy bowl.");
        println!("Before reaching for the door, you hear a noise coming from the porch.");
        println!("You hear muffled footsteps, as well as the rustling of clothes.");
        println!("What do you do?");

        println!("1. Barricade myself in the room");
        println!("2. Go for the shotgun over the fireplace, just in case.");
        println!("3. Go directly to the front door, taking care not to get noticed.");
        println!("4. Act as if nothing happened and continue watching the movie.");


        let mut second_input = String::new();

        io::stdin().read_line(&mut second_input)
            .expect("Failed to read line");

        let second: u32 = second_input.trim().parse()   
            .expect("Please type a number!");
        match second{
            1=> println!("You barricade yourself for the whole night. In the morning, upon going in the living room, you realize the candy bowl on the porch was completely looted. The still-on TV spews news of a massive car crash on the interstate. Your parents and brother are named in the casualties. You have now their death on your conscience for not staying cowardly at home"),
            2=> println!("You reach for the shotgun. At the door, a black figure is hunched over near the door. Without thinking, you blast the shotgun off. A red mist, and the figure falls to the ground. You recognize the neighbor's kid Jim underneath the black costume. You are charged in the coming weeks for Manslaughter."),
            3=> {println!("You sneak your way to the entrance. You observe a black form hunched over the candy bowl on the porch. You recognize Jim, the neighbor's kid, taking the candies left for grabs. Looking further back, you see an imposing man in blue overalls and a white opera mask.\nWhat do you do?\n1. Look from afar.\n2.Get on the porch");
            
                let mut third_input = String::new();
                io::stdin().read_line(&mut third_input)
                    .expect("Failed to read line");
    
                let third: u32 = third_input.trim().parse()   
                    .expect("Please type a number!");
                match third{
                    1=> println!("You stay hidden and witness as Jim walks off the porch. Unaware of the man, he gets snatched away by him and taken into a beat*up white van.\nThe next day, the neighbor frantically asks you if you have seen his son. You deny having witnessed whatsoever the night before"),
                    2=> println!("You greet Jim, who greets you back. He then goes on his merry way as you look for the strange man in the shadows. He is nowhere to be found. Distressed, you decide to shut yourself at home."),
                    _=> funnyguy(),
            }},
            4=> println!("You come back to the movie, and still feel uneasy."),
            _=> funnyguy(),
        }
            main()
    }
        

}
