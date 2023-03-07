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
    ctype: Atype,
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
    let mut player1 = player_init();
    println!("{:?}", player1);

    let mut aiplayer1 = ai_init("Ai".to_string(), Atype::B);
    println!("{:?}", aiplayer1);

    game_core(player1, aiplayer1);
}

fn player_init() -> Player {
    let mut player_name: String = String::new();
    println!("Choose a name: ");
    io::stdin().read_line(&mut player_name).expect("No");
    player_name = String::from(player_name.trim());
    
    let mut player = Player {
        name: player_name,
        ctype: get_player_class(),
        max_hp: thread_rng().gen_range(150..=200),
        current_hp: 0,
        strength: thread_rng().gen_range(20..=60),
        magic: thread_rng().gen_range(30..=50),
        defense: thread_rng().gen_range(0..=15),
    };
    player.current_hp = player.max_hp;
    return player
}

fn get_player_class() -> Ctype {
    let mut player_class: String = String::new();
    loop {
        println!("Choose a class (h for details):\n1)  A\n2)  B\n3)  C");
        io::stdin().read_line(&mut player_class).expect("No");
        
        if player_class.trim() == "h".to_string() {
            println!("Placeholder");
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

fn ai_init(pname: String, class: Atype) -> Ai {
    let mut ai = Ai {
        name: pname,
        ctype: class,
        max_hp: thread_rng().gen_range(150..=200),
        current_hp: 0,
        strength: thread_rng().gen_range(20..=60),
        magic: thread_rng().gen_range(30..=50),
        defense: thread_rng().gen_range(0..=15),
    };
    ai.current_hp = ai.max_hp;
    return ai
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