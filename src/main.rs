use rand::{Rng, thread_rng};
use std::{io, time, thread};

#[derive(Debug)]
enum Ctype {
   A,
   B,
   C,
}

#[derive(Debug)]
struct Player {
    name: String,
    ctype: Ctype,
    max_hp: i16,
    current_hp: i16,
    strength: i16,
    defense: i16,
}

impl Player {
    fn dmg_calc (&self, s: &Player) -> i16 {
        self.strength - s.defense
    } 
}

fn main() {
    let mut usrmode = String::new();

    println!("Choose something something: ");
    io::stdin().read_line(&mut usrmode).expect("no");

    if usrmode.trim() == String::from("q") {
        return
    } else if usrmode.trim() == String::from("a") {
        game_init();
    } else {
        println!("Hola");
    }
}

fn game_init() {
    let mut player1 = Player {
        name: String::from("Aa"),
        ctype: Ctype::A,
        max_hp: thread_rng().gen_range(20..=200),
        current_hp: 0,
        strength: thread_rng().gen_range(20..=50),
        defense: thread_rng().gen_range(0..=15),
    };    
    player1.current_hp = player1.max_hp;
    println!("{:?}", player1);

    let mut player2 = Player {
        name: String::from("Bb"),
        ctype: Ctype::B,
        max_hp: thread_rng().gen_range(20..=200),
        current_hp: 0,
        strength: thread_rng().gen_range(20..=50),
        defense: thread_rng().gen_range(0..=15),
    };
    player2.current_hp = player2.max_hp;
    println!("{:?}", player2);

    game_core(player1, player2);
}

fn game_core(mut p1: Player, mut p2: Player) {
    while true {
        println!("P1 HP: {}", p1.current_hp);
        println!("P2 HP: {}", p2.current_hp);
        if p1.current_hp <= 0 {
            println!("p1 wins DEBUG:{:?}", p1);
            break
        } else if p2.current_hp <= 0 {
            println!("p2 wins DEBUG:{:?}", p2);
            break
        }
        p2.current_hp -= p1.dmg_calc(&p2);
        p1.current_hp -= p2.dmg_calc(&p1);

        thread::sleep(time::Duration::from_secs(2));
   }
}
