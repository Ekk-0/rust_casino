// Dev: Eben 31/12/2022, Blackjack game written in RUST
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, self},
    io::Result,
    io::{ErrorKind, self},
};

#[derive(Serialize, Deserialize)]

struct User {
    username: String,
    password: String,
    money: i32,
}

impl User {
    pub const fn new() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            money: 0,
        }
    }
}

fn main() {
    let mut current_user: User = User::new();

    println!("Welcome to the slots!");
    println!("Please Log In or Register");
    println!("################");
    println!("1. Login");
    println!("2. Register");
    println!("################");

    let mut response = String::new();
    println!("Option: ");
    io::stdin().read_line(&mut response).unwrap();

    let x = response.trim().parse::<i32>().unwrap();

    let auth = match x {
        1 => login(&mut current_user),
        2 => {
            let _r = register();
            if _r.is_ok() {
                login(&mut current_user)
            } else {
                println!("{:?}", _r);
                false
            }
        }
        _ => {
            println!("Wrong Option");
            false
        }
    };

    if auth {
        loop {
            println!(
                "Signed in as {}, current balance is {}",
                &current_user.username, &current_user.money
            );
            if game(&mut current_user) {
                break;
            }
        }
        println!("Thanks for playing!");
    }
}

// Game {

fn game(user: &mut User) -> bool {
    println!("### Game ###");
    println!("1. Start");
    println!("2. Deposit Cash");
    println!("3. Exit");

    let mut response = String::new();
    println!("Option: ");
    io::stdin().read_line(&mut response).unwrap();

    let x = response.trim().parse::<i32>().unwrap();

    match x {
        1 => start(user),
        2 => deposit(user),
        3 => return true,
        _ => {
            println!("Wrong Option")
        }
    };

    false
}

fn deposit(x: &mut User) {
    println!("Depositing...");
    println!("Insert Amount...");

    let mut response = String::new();
    io::stdin().read_line(&mut response).unwrap();

    let amount = response.trim().parse::<i32>().unwrap();

    let _x = update_account(x, amount);
    if _x.is_ok() {
        x.money += amount;
        println!("Amount {}, Successfully Inserted", amount);
        return;
    } else {
        println!("Error has occurred!");
    }
}

fn start(user: &mut User) {
    println!("##### Game Started #####");
    println!("Place your bet!");

    if user.money < 0 {
        println!("Please Deposit Money");
        return;
    }

    let mut response = String::new();
    io::stdin().read_line(&mut response).unwrap();

    let mut bet = response.trim().parse::<i32>().unwrap();

    if bet > user.money {
        println!("Insufficient funds for bet!");
        return;
    }

    let mut rng = rand::thread_rng();

    let mut dealer_number: i32 = rng.gen_range(1..=11) + rng.gen_range(1..=11);
    let mut your_number: i32 = rng.gen_range(1..=11) + rng.gen_range(1..=11);

    let mut fold = false;
    let mut lose = false;
    let mut won = false;
    loop {
        loop {
            println!("Your bet is {}", bet);
            println!(
                "The dealer got {}, your number is {}",
                dealer_number, your_number
            );

            if your_number == 21 {
                println!("Blackjack!");
                won = true;
                break;
            }

            if dealer_number == 21 {
                println!("Blackjack for dealer!");
                lose = true;
                break;
            }

            println!("Options:\n\r1.hit\n\r2. stand\n\r3. double\n\r4. fold");
            let mut response = String::new();
            io::stdin().read_line(&mut response).unwrap();

            let x = response.trim().parse::<i32>().unwrap();

            match x {
                1 => {
                    your_number += rng.gen_range(0..=11);
                }
                2 => {
                    break;
                }
                3 => {
                    your_number += rng.gen_range(0..=11);
                    bet *= 2;
                }
                4 => {
                    fold = true;
                    break;
                }
                _ => {
                    println!("Wrong Option");
                }
            };

            if your_number > 21 {
                lose = true;
                break;
            }
        }
        if fold || lose {
            break;
        }

        match dealer_number {
            17 => {
                if dealer_number > your_number {
                } else {
                    won = true;
                    break;
                }
            }
            _ => {
                if dealer_number == your_number && dealer_number > 17 {
                    println!("Draw!");
                    break;
                }

                if dealer_number > your_number {
                    lose = true;
                    break;
                }

                dealer_number += rng.gen_range(0..=11);

                if dealer_number > 21 {
                    won = true;
                    break;
                }
            }
        }
    }
    if lose {
        println!("You lost!");
        println!(
            "The dealer got {}, your number is {}",
            dealer_number, your_number
        );
        let _x = update_account(user, -bet);
        if _x.is_ok() {
            user.money -= bet;
            return;
        } else {
            println!("Error has occurred!");
        }
        return;
    }

    if won {
        println!("You won!");
        println!(
            "The dealer got {}, your number is {}",
            dealer_number, your_number
        );
        let _x = update_account(user, bet);
        if _x.is_ok() {
            user.money += bet;
            return;
        } else {
            println!("Error has occurred!");
        }
        return;
    }
}

// }

fn update_account(_x: &mut User, amount: i32) -> Result<()> {
    let path = "db.json";
    let f = File::open(path);

    let _ = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _ => panic!("Error"),
        },
    };

    let data = fs::read_to_string(path).expect("Unable to read file");

    let mut users: Vec<User> = Vec::new();
    if fs::metadata(path).unwrap().len() != 0 {
        users = serde_json::from_str(&data)?;
    }

    for user in users.iter_mut() {
        if user.username == _x.username && user.password == _x.password {
            user.money += amount;
        }
    }

    let json: String = serde_json::to_string(&users)?;

    fs::write(path, &json).expect("Unable to write file");

    Ok(())
}

// Auth {

fn register() -> std::io::Result<()> {
    println!("Register: ");
    let mut username = String::new();
    let mut password = String::new();
    let mut confirm_password = String::new();

    println!("Username: ");
    io::stdin().read_line(&mut username).unwrap();
    println!("Password: ");
    io::stdin().read_line(&mut password).unwrap();
    println!("Confirm Password: ");
    io::stdin().read_line(&mut confirm_password).unwrap();

    if confirm_password == password {
        let path = "db.json";
        let f = File::open(path);

        let _ = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create(path) {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                _ => panic!("Error"),
            },
        };

        let data = fs::read_to_string(path).expect("Unable to read file");

        let mut users: Vec<User> = Vec::new();
        if fs::metadata(path).unwrap().len() != 0 {
            users = serde_json::from_str(&data)?;
        }

        let user = User {
            username: username.trim().to_owned(),
            password: password.trim().to_owned(),
            money: 0,
        };

        users.push(user);

        let json: String = serde_json::to_string(&users)?;

        fs::write(path, &json).expect("Unable to write file");

        Ok(())
    } else {
        Err(std::io::Error::new(
            ErrorKind::NotFound,
            "Passwords do not match.",
        ))
    }
}

fn login(current_user: &mut User) -> bool {
    let mut username = String::new();
    let mut password = String::new();

    println!("Username: ");
    io::stdin().read_line(&mut username).unwrap();
    println!("Password: ");
    io::stdin().read_line(&mut password).unwrap();

    let _r = validate_user(current_user, username, password);
    if _r.is_ok() {
        return true;
    } else {
        println!("{:?}", _r);
        return false;
    }
}

fn validate_user(current_user: &mut User, username: String, password: String) -> Result<()> {
    let path = "db.json";
    let f = File::open(path);

    let _ = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _ => panic!("Error"),
        },
    };

    let data = fs::read_to_string(path).expect("Unable to read file");

    let mut users: Vec<User> = Vec::new();
    if fs::metadata(path).unwrap().len() != 0 {
        users = serde_json::from_str(&data)?;
    }

    for user in users.iter() {
        if user.username == username.trim() && user.password == password.trim() {
            current_user.username = user.username.to_string();
            current_user.password = user.password.to_string();
            current_user.money = user.money;

            return Ok(());
        }
    }
    Err(std::io::Error::new(ErrorKind::NotFound, "User not found"))
}

// }
