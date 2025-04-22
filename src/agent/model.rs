use std::fmt::Display;

// Position Things
const EPSILON: f64 = 1e-6;

/// Generics Position
#[derive(Debug)]
pub struct Position<T> {
    x: T,
    y: T,
    angle: f64
}

impl PartialEq for Position<i32> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialEq for Position<f64> {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < EPSILON && (self.y - other.y).abs() < EPSILON
    }
}

impl<T> Display for Position<T> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Position: {{x: {}, y: {}, angle: {}}}", self.x, self.y, self.angle)
    }
}

impl<T> Position<T> {
    pub fn new(x: T, y: T, angle: f64) -> Position<T> {
        Position { x, y, angle }
    }
}

// Game Statistics Things...
#[derive(Debug, EnumString, PartialEq)]
pub enum Stage {
    Rest,
    Battle,
    End,
}

#[derive(Debug, PartialEq)]
pub struct TokenScore {
    token: String,
    score: u32
}

#[derive(Debug)]
pub struct ScoreBoard {
    scores: Vec<TokenScore>
}

#[derive(Debug)]
pub struct GameStatistics {
    current_stage: Stage,
    count_down: u32,
    ticks: u32,
    scores: ScoreBoard
}

impl TokenScore {
    pub fn new(token: String, score: u32) -> TokenScore {
        TokenScore { token, score }
    }
}

impl ScoreBoard {
    pub fn new() -> ScoreBoard {
        ScoreBoard { scores: Vec::new() }
    }
}

impl GameStatistics {
    pub fn new() -> GameStatistics {
        GameStatistics { 
            current_stage: Stage::Rest, 
            count_down: 0, 
            ticks: 0, 
            scores: ScoreBoard::new() 
        }
    }
}

impl Display for Stage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Stage: {}", self.to_string())
    }
}

impl Display for TokenScore {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " Token({}): Score({}) ", self.token, self.score)
    }
}

impl Display for ScoreBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[")?;
        for score in &self.scores {
            writeln!(f, "{}, ", score)?; // Multiline Scoreboard
        }
        writeln!(f, "]")
    }
}

impl Display for GameStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "GameStatistics: {{ Stage: {}, CountDown: {}, Ticks: {}, Scores: {} }}",
            self.current_stage, self.count_down, self.ticks, &self.scores
        )
    }
}

// Environment Info things...
#[derive(Debug)]
pub struct Wall {
    position: Position<i32>
}

#[derive(Debug)]
pub struct Fence {
    position: Position<i32>,
    health: u32
}

#[derive(Debug)]
pub struct Bullet {
    id: u32,
    is_missile: bool,
    is_anti_armor: bool,
    position: Position<f64>,
    speed: f64,
    damage: f64,
    traveled_distance: f64
}

#[derive(Debug)]
pub struct EnvironmentInfo {
    map_size: u32,
    walls: Vec<Wall>,
    fences: Vec<Fence>,
    bullets: Vec<Bullet>,
}

impl Display for Wall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Wall: {{ position: {} }}", self.position)
    }
}

impl Display for Fence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fence: {{ position: {}, health: {} }}", self.position, self.health)
    }
}

impl Display for Bullet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "Bullet: {{ \
            No: {}, \
            IsMissile: {}, \
            IsAntiArmor: {}, \
            Position: {}, \
            Speed: {}, \
            Damage: {}, \
            TraveledDistance: {} \
            }}", 
            self.id, 
            self.is_missile, 
            self.is_anti_armor, 
            self.position, 
            self.speed, 
            self.damage, 
            self.traveled_distance
        )
    }
}

impl Display for EnvironmentInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "EnvironmentInfo: {{ MapSize: {}, Walls: [", self.map_size)?;
        for wall in &self.walls {
            write!(f, "{}, ", wall)?;
        }
        write!(f, "], Fences: [")?;
        for fence in &self.fences {
            write!(f, "{}, ", fence)?;
        }
        write!(f, "], Bullets: [")?;
        for bullet in &self.bullets {
            write!(f, "{}, ", bullet)?;
        }
        write!(f, "] }}")
    }
}

// Available Buff things...
#[derive(Debug, EnumString, PartialEq, Clone)]
pub enum BuffKind {
    BlackOut,
    SpeedUp,
    Flash,
    Destroy,
    Construct,
    Trap,
    Missile,
    Kamui,
    BulletCount,
    BulletSpeed,
    AttackSpeed,
    Laser,
    Damage,
    AntiArmor,
    Armor,
    Reflect,
    Dodge,
    Knife,
    Gravity
}

// struct SkillKind(u8);

pub type AvailableBuffs = Vec<BuffKind>;

impl Display for BuffKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BuffKind: {}", self.to_string())
    }
}

impl PartialEq<SkillKind> for BuffKind {
    fn eq(&self, other: &SkillKind) -> bool {
        other.clone() as u8 == self.clone() as u8
    }
}

// Player things

#[derive(Debug, PartialEq, EnumString, Clone)]
pub enum ArmorKnifeState {
    NotOwner,
    Available,
    Active,
    Broken
}

#[derive(Debug, PartialEq, EnumString, Clone)]
pub enum SkillKind {
    BlackOut,
    SpeedUp,
    Flash,
    Destroy,
    Construct,
    Trap,
    Missile,
    Kamui
}

#[derive(Debug, Clone, PartialEq)]
pub struct Weapon {
    attack_speed: f64,
    bullet_speed: f64,
    is_laser: bool,
    anti_armor: bool,
    damage: u32,
    max_bullets: u32,
    current_bullets: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Armor {
    can_reflect: bool,
    gravity_field: bool,
    armor_value: u32,
    health: i32,
    dodge_rate: f64,
    knife: ArmorKnifeState
}

#[derive(Debug, Clone, PartialEq)]
pub struct Skill {
    name: SkillKind,
    max_cool_down: u32,
    current_cool_down: u32,
    is_active: bool
}

pub struct Player {
    token: String,
    position: Position<f64>,
    weapon: Weapon,
    armor: Armor,
    skills: Vec<Skill>
}

pub type Players = Vec<Player>;







