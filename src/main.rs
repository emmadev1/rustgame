use rand::{Rng, thread_rng};
use std::{io, time, thread};

#[derive(Debug)]
enum Ctype {
    A,
    B,
    C,
}

#[derive(Debug)]
enum Atype {
    A,
    B,
    C,
}

#[derive(Debug)]
enum Stype {
    A,
    B,
    C,
}

#[derive(Debug)]
struct Stage {
    size: u8,
    stype: Stype,
}

#[derive(Debug)]
struct Player {
    name: String,
    ctype: Ctype,
    max_hp: i16,
    current_hp: i16,
    strength: i16,
    magic: i16,
    defense: i16,
}

#[derive(Debug)]
struct Ai {
    name: String,
    atype: Atype,
    max_hp: i16,
    current_hp: i16,
    strength: i16,
    magic: i16,
    defense: i16,
}

impl Player {
    fn attack_basic (&self, s: &Ai) -> i16 {
        self.strength - s.defense
    } 

    fn attack_magic (&self, s: &Ai) -> i16 {
        self.magic
    }
}

impl Ai {
    fn attack_basic (&self, s: &Player) -> i16 {
        self.strength - s.defense
    }

    fn attack_magic (&self, s: &Player) -> i16 {
        self.magic
    }
}

fn main() {
    let mut usrmode: String = String::new();

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
    let player1: Player = player_init();
    println!("{:?}", player1);

    let aiplayer1: Ai = ai_init("Ai".to_string());
    println!("{:?}", aiplayer1);

    game_core(player1, aiplayer1);
}

fn player_init() -> Player {
    let mut player_name: String = String::new();
    println!("Choose a name: ");
    io::stdin().read_line(&mut player_name).expect("No");
    player_name = String::from(player_name.trim());
    
    let mut player: Player = Player {
        name: player_name,
        ctype: get_player_class(),
        max_hp: 0,
        current_hp: 0,
        strength: 0,
        magic: 0,
        defense: 0,
    };
    player.max_hp = get_player_attribute(&player.ctype, "max_hp".to_string());
    player.strength = get_player_attribute(&player.ctype, "strength".to_string());
    player.magic = get_player_attribute(&player.ctype, "magic".to_string());
    player.defense = get_player_attribute(&player.ctype, "defense".to_string());

    player.current_hp = player.max_hp;
    return player
}

fn get_player_class() -> Ctype {
    let mut player_class: String = String::new();
    loop {
        println!("Choose a class (h for details):\n1)  A\n2)  B\n3)  C");
        io::stdin().read_line(&mut player_class).expect("No");
        
        if player_class.trim() == "h".to_string() {
            println!("There are no details");
            player_class.clear();
        } else if player_class.trim() == "a".to_string() {
            return Ctype::A
        } else if player_class.trim() == "b".to_string() {
            return Ctype::B
        } else if player_class.trim() == "c".to_string() {
            return Ctype::C
        } else {
            println!("Choose a proper class!");
            player_class.clear();
        }
    };
}

fn get_player_attribute(template: &Ctype, attrib: String) -> i16 {
    match template {
        Ctype::A => {
            if attrib == "max_hp".to_string() {
                return thread_rng().gen_range(100..=120)
            } else if attrib == "strength" {
                return thread_rng().gen_range(50..=90)
            } else if attrib == "magic" {
                return 0
            } else if attrib == "defense" {
                return thread_rng().gen_range(0..=5)
            } else {
                println!("Hola");
                return 0
            }
        }
        Ctype::B => {
            if attrib == "max_hp".to_string() {
                return thread_rng().gen_range(200..=250)
            } else if attrib == "strength" {
                return thread_rng().gen_range(10..=35)
            } else if attrib == "magic" {
                return 0
            } else if attrib == "defense" {
                return thread_rng().gen_range(15..=25)
            } else {
                println!("Hola");
                return 0
            }
        }
        Ctype::C => {
            if attrib == "max_hp".to_string() {
                return thread_rng().gen_range(70..=140)
            } else if attrib == "strength" {
                return thread_rng().gen_range(10..=30)
            } else if attrib == "magic" {
                return thread_rng().gen_range(70..=90)
            } else if attrib == "defense" {
                return thread_rng().gen_range(0..=2)
            } else {
                println!("Hola");
                return 0
            }
        }
    }
}

fn ai_init(pname: String) -> Ai {
    let mut ai: Ai = Ai {
        name: pname,
        atype: get_ai_class(),
        max_hp: 0,
        current_hp: 0,
        strength: 0,
        magic: 0,
        defense: 0,
    };
    ai.max_hp = get_ai_attribute(&ai.atype, "max_hp".to_string());
    ai.strength = get_ai_attribute(&ai.atype, "strength".to_string());
    ai.magic = get_ai_attribute(&ai.atype, "magic".to_string());
    ai.defense = get_ai_attribute(&ai.atype, "defense".to_string());

    ai.current_hp = ai.max_hp;
    return ai
}

fn get_ai_class() -> Atype {
    let randclass: u8 = thread_rng().gen_range(1..=3);

    if randclass == 1 {
        return Atype::A
    } else if randclass == 2 {
        return Atype::B
    } else if randclass == 3 {
        return Atype::C
    } else {
        println!("Alejandro como?");
        return Atype::A
    }
}

fn get_ai_attribute(template: &Atype, attrib: String) -> i16 {
    match template {
        Atype::A => {
            if attrib == "max_hp".to_string() {
                return thread_rng().gen_range(100..=120)
            } else if attrib == "strength" {
                return thread_rng().gen_range(50..=90)
            } else if attrib == "magic" {
                return 0
            } else if attrib == "defense" {
                return thread_rng().gen_range(0..=5)
            } else {
                println!("Hola");
                return 0
            }
        }
        Atype::B => {
            if attrib == "max_hp".to_string() {
                return thread_rng().gen_range(200..=250)
            } else if attrib == "strength" {
                return thread_rng().gen_range(10..=35)
            } else if attrib == "magic" {
                return 0
            } else if attrib == "defense" {
                return thread_rng().gen_range(15..=25)
            } else {
                println!("Hola");
                return 0
            }
        }
        Atype::C => {
            if attrib == "max_hp".to_string() {
                return thread_rng().gen_range(70..=140)
            } else if attrib == "strength" {
                return thread_rng().gen_range(10..=30)
            } else if attrib == "magic" {
                return thread_rng().gen_range(70..=90)
            } else if attrib == "defense" {
                return thread_rng().gen_range(0..=2)
            } else {
                println!("Hola");
                return 0
            }
        }
    }
}

fn game_core(mut p1: Player, mut ai1: Ai) {
    loop {
        println!("Player \x1b[01;31mHP\x1b[0m: {}", p1.current_hp);
        println!("AI \x1b[01;31mHP\x1b[0m: {}", ai1.current_hp);
        if p1.current_hp <= 0 {
            println!("\x1b[1;33mAI\x1b[0m wins DEBUG:\n{:?}\n{:?}", p1, ai1);
            break
        } else if ai1.current_hp <= 0 {
            println!("\x1b[01;32mPlayer\x1b[0m wins DEBUG:\n{:?}\n{:?}", p1, ai1);
            break
        }

        ai1.current_hp -= usr_turn(&p1, &ai1);
        p1.current_hp -= ai_turn(&ai1, &p1);

        //thread::sleep(time::Duration::from_secs(2));
   }
}

fn usr_turn(p1: &Player, ai1: &Ai) -> i16 {
    let mut usrin: String = String::new();
    println!("Choose a move (impl 1 and 2): ");
    io::stdin().read_line(&mut usrin).expect("no");
    
    if usrin.trim() == String::from("1") {
        println!("{} uses basic!", p1.name);
        p1.attack_basic(&ai1)
    } else if usrin.trim() == String::from("2") {
        println!("{} uses magic!", p1.name);
        p1.attack_magic(&ai1)
    } else {
        println!("No");
        0
    }
}

fn ai_turn(ai1: &Ai, p1: &Player) -> i16 {
    let selec: u8 = thread_rng().gen_range(0..=1);

    if p1.defense > 11 {
        println!("Ai uses magic!");
        ai1.attack_magic(&p1)
    } else if selec == 0 {
        println!("Ai uses basic!");
        ai1.attack_basic(&p1)
    } else if selec == 1 {
        println!("Ai uses magic!");
        ai1.attack_magic(&p1)
    } else {
        println!("No");
        0
    }
}