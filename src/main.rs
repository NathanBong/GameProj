use std::{clone, collections::HashMap, env, fmt::Display, io, string};


#[derive(Clone)]
struct Player
{
    max_health: u32,
    current_health: u32,
    controller: PlayerController,
    actions: HashMap<String, ActionOption>,
}

impl Player
{
    fn new(max_health: u32, controller: PlayerController) -> Self
    {
        Player
        {
            max_health: max_health,
            current_health: max_health,
            controller: controller,
            actions: HashMap::new(),
        }
    }

    fn add_move(&mut self, action: ActionOption) -> Option<ActionOption>
    {
        self.actions.insert(action.name.clone(), action)
    }
}

#[derive(Clone)]
struct ActionOption
{
    name: String,
    damage: u32,
}

impl Display for ActionOption
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "{}: {} damage", self.name, self.damage)
    }
}

impl ActionOption
{
    fn new(name: String, damage: u32) -> Self
    {
        ActionOption
        {
            name: name,
            damage: damage,
        }
    }
}

impl Player
{
    // returns if the turn ends
    fn run_turn(&self) -> bool
    {
        match self.controller
        {
            PlayerController::PLAYER => 
            {
                self.display_actions();

                let mut guess = String::new();
                io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

                let guess = guess.trim();
                println!("You input: {guess}");

                match self.actions.get(guess)
                {
                    Some(action) => 
                    {
                        println!("player casts {action}");
                        false
                    },
                    None => 
                    {
                        println!("no action used");
                        false
                    },
                }
            },
            PlayerController::AI => 
            {
                println!("AI CASTS FIRECOCK");
                false
            },
        }
    }

    fn display_actions(&self)
    {
        println!("Actions: ");
        for entry in &self.actions
        {
            println!("{}", entry.1);
        }
        println!();
    }
}


#[derive(Clone, Copy)]
enum PlayerController
{
    PLAYER,
    AI
}

// turn based system
// takes an input, then moves to another turn
fn main() 
{
    game_loop();
}

fn game_loop()
{
    let mut keep_playing: bool = true;

    // MAGIC NUMBERS
    let mut players: Vec<Player> = Vec::new(); 

    let mut temp = Player::new(10, PlayerController::PLAYER);
    temp.add_move(ActionOption::new("railgun".to_owned(), 100));
    temp.add_move(ActionOption::new("autogun_right".to_owned(), 75));
    temp.add_move(ActionOption::new("autogun_left".to_owned(), 75));
    
    players.push(temp);
    players.push(Player::new(10, PlayerController::AI));
    players.push(Player::new(10, PlayerController::AI));

    // turn order:
    // list of all "players"
    // each of them make a turn
    // turn marker?
    while keep_playing
    {
        // execute turn order
        for i in &players
        {
            if i.run_turn()
            {
                keep_playing = false;
                break;
            }
        }
    }
}