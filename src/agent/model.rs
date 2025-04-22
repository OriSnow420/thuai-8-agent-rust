use std::fmt::Display;

use getset::Getters;

// Position Things
const EPSILON: f64 = 1e-6;

/// Generics Position to represent a (x, y) pair with an angle.
///
/// [`PartialEq`] is implemented for [`Position<f64>`] and [`Position<i32>`].
/// You **Shouldn't** use [`Position<T>`] where `T` is not [`i32`] or [`f64`].
/// [`PartialEq`] doesn't compare angles.
///
///
/// [`Display`] is implemented for `T` implements [`Display`], thus [`Position<i32>`]
/// and [`Position<f64>`] both can be formatted printed.
///
/// Should be constructed with [`Position<T>::new`].
///
/// Fields should be get through getter method `field()`.
///
/// # Example
///
/// ```
/// use thuai_8_agent_rust::agent::model::Position;
///
/// let pos1 = Position::new(3, 2, 1.0);
/// let pos2 = Position::new(3, 2, 2.0);
///
/// assert_eq!(pos1, pos2);
/// ```
#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct Position<T> {
    x: T,
    y: T,
    angle: f64,
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

impl<T> Display for Position<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Position: {{x: {}, y: {}, angle: {}}}",
            self.x, self.y, self.angle
        )
    }
}

impl<T> Position<T> {
    /// Constructs a new [`Position<T>`] with `{ x, y, angle }`
    pub fn new(x: T, y: T, angle: f64) -> Position<T> {
        Position { x, y, angle }
    }
}

// Game Statistics Things...

/// Represent the game stage.
#[derive(Debug, EnumString, PartialEq)]
pub enum Stage {
    Rest,
    Battle,
    End,
}

/// One entry on the scoreboard, recording the player's token and score.
///
/// Should be created with [`TokenScore::new`].
///
/// Fields should be get through getter method `field()`.
#[derive(Debug, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct TokenScore {
    token: String,
    score: u32,
}

/// Records all player's token and his score.
///
/// Should be created with [`ScoreBoard::new`].
///
/// Fields should be get through getter method `field()`.
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct ScoreBoard {
    scores: Vec<TokenScore>,
}

/// Represent the Statistics Information of the game, including:
/// - current game stage
/// - count_down
/// - current ticks
/// - scoreboard
///
/// Should be created with [`GameStatistics::new`].
///
/// Fields should be get through getter method `field()`.
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct GameStatistics {
    current_stage: Stage,
    count_down: u32,
    ticks: u32,
    scores: ScoreBoard,
}

impl TokenScore {
    pub fn new(token: String, score: u32) -> TokenScore {
        TokenScore { token, score }
    }
}

impl ScoreBoard {
    pub fn new(scores: Vec<TokenScore>) -> ScoreBoard {
        ScoreBoard { scores }
    }
}

impl GameStatistics {
    pub fn new(
        current_stage: Stage,
        count_down: u32,
        ticks: u32,
        scores: ScoreBoard,
    ) -> GameStatistics {
        GameStatistics {
            current_stage,
            count_down,
            ticks,
            scores,
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
        write!(
            f,
            "GameStatistics: {{ Stage: {}, CountDown: {}, Ticks: {}, Scores: {} }}",
            self.current_stage, self.count_down, self.ticks, &self.scores
        )
    }
}

// Environment Info things...
/// Represent a unbreakable wall in the map.
///
/// Note that walls have directions, and it is recorded in `position.angle`,
/// though the angle of a wall will only be 0 (parallel to x axis) or 90 (
/// parallel to y axis).
///
/// Fields should be get through getter method `field()`.
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct Wall {
    position: Position<i32>,
}

/// Represent a breakable wall (aka fence in thuai-8) in the map.
///
/// Note that fences have directions, and it is recorded in `position.angle`,
/// though the angle of a fence will only be 0 (parallel to x axis) or 90
/// (parallel to y axis).
///
/// When health goes to 0, the fence will be broken and will disappear.
///
/// Fields should be get through getter method `field()`.
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct Fence {
    position: Position<i32>,
    health: u32,
}

/// Represent a bullet flying in the battlefield.
///
/// bullets have:
/// - id (in the API, it is called "no")
/// - its position
/// - flying speed
/// - the damage it can cause
/// - the distance it has traveled (used to control its disappearance)
///
/// and two additional bool values used to say if its a missile or it is
/// anti-armor.
///
/// Fields should be get through getter method `field()`.
#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct Bullet {
    id: u32,
    is_missile: bool,
    is_anti_armor: bool,
    position: Position<f64>,
    speed: f64,
    damage: f64,
    traveled_distance: f64,
}

/// Represents the environment info.
///
/// Contains:
/// - Map size
/// - List of [`Wall`]s and [`Fence`]s
/// - List of [`Bullet`]s
///
/// Fields should be get through getter method `field()`.
#[derive(Debug, Getters)]
#[getset(get = "pub")]
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
        write!(
            f,
            "Fence: {{ position: {}, health: {} }}",
            self.position, self.health
        )
    }
}

impl Display for Bullet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
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
        write!(
            f,
            "EnvironmentInfo: {{ MapSize: {}, Walls: [",
            self.map_size
        )?;
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

/// Enum class to represent all kinds of Buff. Some buff can be actively activated,
/// and they are also called skills, whose kinds are represented by [`SkillKind`].
///
/// Corresponding skills and buffs have the same name.
///
/// [`PartialEq<Self>`] and [`PartialEq<SkillKind>`] is implemented, so you can
/// compare a [`BuffKind`] with a [`SkillKind`]. Note that the inverted compare
/// is wrong!
///
/// Can be converted from String.
///
/// # Examples
///
/// Compare between [`BuffKind`] and [`SkillKind`]
/// ```
/// use thuai_8_agent_rust::agent::model::{BuffKind, SkillKind};
///
/// let buff = BuffKind::Missile;
/// let skill = SkillKind::Missile;
///
/// assert_eq!(buff, skill);
/// ```
///
/// Inverted compare is not offered! The following will cause compilation failure!
/// ```compile_fail
/// use thuai_8_agent_rust::agent::model::{BuffKind, SkillKind};
///
/// let buff = BuffKind::Missile;
/// let skill = SkillKind::Missile;
///
/// assert_eq!(skill, buff);
/// ```
///
/// Get [`BuffKind`] from [`String`].
/// ```
/// use thuai_8_agent_rust::agent::model::BuffKind;
/// use std::str::FromStr;
///
/// let buff = BuffKind::Reflect;
/// let buff_from_string = BuffKind::from_str("Reflect").unwrap();
///
/// assert_eq!(buff, buff_from_string);
/// ```
#[derive(Debug, EnumString, PartialEq, Clone, Display)]
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
    Gravity,
}

/// Type alias for AvailableBuffs, which is a [`Vec<T>`] where `T` is [`BuffKind`].
pub type AvailableBuffs = Vec<BuffKind>;

impl PartialEq<SkillKind> for BuffKind {
    fn eq(&self, other: &SkillKind) -> bool {
        other.clone() as u8 == self.clone() as u8
    }
}

// Player things

/// Enum class to represent the player's state of ArmorKnife, provided by the
/// buff [`BuffKind::Knife`].
///
/// [`PartialEq<Self>`] is implemented, so you can compare between [`ArmorKnifeState`]s.
///
/// Can be converted from String.
///
/// # Examples
///
/// Get [`ArmorKnifeState`] from [`String`].
/// ```
/// use thuai_8_agent_rust::agent::model::ArmorKnifeState;
/// use std::str::FromStr;
///
/// let state = ArmorKnifeState::Active;
/// let state_from_string = ArmorKnifeState::from_str("Active").unwrap();
///
/// assert_eq!(state, state_from_string);
/// ```
#[derive(Debug, PartialEq, EnumString, Clone, Display)]
pub enum ArmorKnifeState {
    NotOwned,
    Available,
    Active,
    Broken,
}

/// Enum class to represent all kinds of skills. Skills are provided by the buff
/// with the same name and can be actively activated.
///
/// Corresponding skills and buffs have the same name.
///
/// [`PartialEq<SkillKind>`] is implemented for both [`SkillKind`] and [`BuffKind`],
/// meaning that you can compare between [`BuffKind`] and [`SkillKind`]. Note that
/// inverted compare is not implemented.
///
/// Can be converted from String.
///
/// # Examples
///
/// Compare between [`BuffKind`] and [`SkillKind`]
/// ```
/// use thuai_8_agent_rust::agent::model::{BuffKind, SkillKind};
///
/// let buff = BuffKind::Construct;
/// let skill = SkillKind::Construct;
///
/// assert_eq!(buff, skill);
/// ```
///
/// Inverted compare is not offered! The following will cause compilation failure!
/// ```compile_fail
/// use thuai_8_agent_rust::agent::model::{BuffKind, SkillKind};
///
/// let buff = BuffKind::Flash;
/// let skill = SkillKind::Flash;
///
/// assert_eq!(skill, buff);
/// ```
///
/// Get [`BuffKind`] from [`String`].
/// ```
/// use thuai_8_agent_rust::agent::model::BuffKind;
/// use std::str::FromStr;
///
/// let buff = BuffKind::Reflect;
/// let buff_from_string = BuffKind::from_str("Reflect").unwrap();
///
/// assert_eq!(buff, buff_from_string);
/// ```
#[derive(Debug, PartialEq, EnumString, Clone, Display)]
pub enum SkillKind {
    BlackOut,
    SpeedUp,
    Flash,
    Destroy,
    Construct,
    Trap,
    Missile,
    Kamui,
}

/// Represent the weapon info of a player.
///
/// Fields should be get through getter method `field()`.
///
/// # Examples
/// ```
/// use thuai_8_agent_rust::agent::model::Weapon;
///
/// let weapon = Weapon::new(2.0, 3.0, false, false, 20, 10, 0);
///
/// assert_eq!(weapon.damage(), &20);
/// ```
#[derive(Debug, Clone, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Weapon {
    attack_speed: f64,
    bullet_speed: f64,
    is_laser: bool,
    anti_armor: bool,
    damage: u32,
    max_bullets: u32,
    current_bullets: u32,
}

/// Represent the armor info of a player.
///
/// Fields should be get through getter method `field()`.
///
/// # Examples
///
/// ```
/// use thuai_8_agent_rust::agent::model::{Armor, ArmorKnifeState};
///
/// let armor = Armor::new(false, false, 20, 0, 2.0, ArmorKnifeState::Active);
///
/// assert_eq!(armor.health(), &0);
///
/// ```
#[derive(Debug, Clone, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Armor {
    can_reflect: bool,
    gravity_field: bool,
    armor_value: u32,
    health: i32,
    dodge_rate: f64,
    knife: ArmorKnifeState,
}

/// Represent one skill with kind, cool down, etc.
///
/// Fields should be get through getter method `field()`.
///
/// # Examples
///
/// ```
/// use thuai_8_agent_rust::agent::model::{Skill, SkillKind};
///
/// let skill = Skill::new(SkillKind::BlackOut, 20, 10, true);
///
/// assert_eq!(skill.name(), &SkillKind::BlackOut);
/// ```
#[derive(Debug, Clone, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Skill {
    name: SkillKind,
    max_cool_down: u32,
    current_cool_down: u32,
    is_active: bool,
}

/// Player struct.
///
/// Fields should be get through getter method `field()`.
///
/// # Examples
///
/// ```
/// use thuai_8_agent_rust::agent::model::{
///     Player, Position, Weapon, Armor,
///     Skill, ArmorKnifeState, SkillKind
/// };
///
/// let player = Player::new(
///     "1919810".to_string(),
///     Position::new(2.0, 3.0, 0.0),
///     Weapon::new(1.0, 1.0, false, false, 10, 10, 0),
///     Armor::new(false, false, 10, 20, 1.0, ArmorKnifeState::NotOwned),
///     vec![Skill::new(SkillKind::Flash, 20, 10, true)]
/// );
///
/// assert_eq!(player.weapon(), &Weapon::new(1.0, 1.0, false, false, 10, 10, 0));
/// ```
#[derive(Debug, Clone, PartialEq, Getters)]
#[getset(get = "pub")]
pub struct Player {
    token: String,
    position: Position<f64>,
    weapon: Weapon,
    armor: Armor,
    skills: Vec<Skill>,
}

pub type Players = Vec<Player>;

impl Weapon {
    pub fn new(
        attack_speed: f64,
        bullet_speed: f64,
        is_laser: bool,
        anti_armor: bool,
        damage: u32,
        max_bullets: u32,
        current_bullets: u32,
    ) -> Weapon {
        Weapon {
            attack_speed,
            bullet_speed,
            is_laser,
            anti_armor,
            damage,
            max_bullets,
            current_bullets,
        }
    }
}

impl Armor {
    pub fn new(
        can_reflect: bool,
        gravity_field: bool,
        armor_value: u32,
        health: i32,
        dodge_rate: f64,
        knife: ArmorKnifeState,
    ) -> Armor {
        Armor {
            can_reflect,
            gravity_field,
            armor_value,
            health,
            dodge_rate,
            knife,
        }
    }
}

impl Skill {
    pub fn new(
        name: SkillKind,
        max_cool_down: u32,
        current_cool_down: u32,
        is_active: bool,
    ) -> Skill {
        Skill {
            name,
            max_cool_down,
            current_cool_down,
            is_active,
        }
    }
}

impl Player {
    pub fn new(
        token: String,
        position: Position<f64>,
        weapon: Weapon,
        armor: Armor,
        skills: Vec<Skill>,
    ) -> Player {
        Player {
            token,
            position,
            weapon,
            armor,
            skills,
        }
    }
}
