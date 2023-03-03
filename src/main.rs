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
    fn attack_basic (&self, s: &Player) -> i16 {
        self.strength - s.defense
    } 

    fn attack_magic (&self, s: &Player) -> i16 {
        self.strength
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

fn game_core(mut p1: Player, mut ai1: Player) {
    while true {
        println!("Player HP: {}", p1.current_hp);
        println!("AI HP: {}", ai1.current_hp);
        if p1.current_hp <= 0 {
            println!("AI wins DEBUG:\n{:?}\n{:?}", p1, ai1);
            break
        } else if ai1.current_hp <= 0 {
            println!("Player wins DEBUG:\n{:?}\n{:?}", p1, ai1);
            break
        }

        ai1.current_hp -= usr_turn(&p1, &ai1);
        p1.current_hp -= ai_turn(&p1, &ai1);

        thread::sleep(time::Duration::from_secs(2));
   }
}

fn usr_turn(p1: &Player, ai1: &Player) -> i16 {
    let mut usrin: String = String::new();
    io::stdin().read_line(&mut usrin).expect("no");
    
    if usrin.trim() == String::from("1") {
        p1.attack_basic(&ai1)
    } else if usrin.trim() == String::from("2") {
        p1.attack_magic(&ai1)
    } else {
        println!("No");
        0
    }
}

fn ai_turn(ai1: &Player, p1: &Player) -> i16 {
    let selec: u8 = thread_rng().gen_range(0..=1);
    if selec == 0 {
        ai1.attack_basic(&p1)
    } else if selec == 1 {
        ai1.attack_magic(&p1)
    } else {
        println!("No");
        0
    }
}