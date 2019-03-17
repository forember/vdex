use crate::enums::*;
use crate::FromVeekun;

/// The effect of a move or set of moves.
///
/// Some effects are shared among several moves, whereas others are unique to a
/// single move.
#[EnumRepr(type = "u16")]
pub enum Effect {
    // Generic
    RegularDamage = 1,
    SleepTarget,
    ChancePoisonTarget,
    HealUserHalfInflicted,
    ChanceBurnTarget,
    ChanceFreezeTarget,
    ChanceParalyzeTarget,
    FaintUser,
    // Unique
    DreamEater,
    MirrorMove,
    // Generic
    RaiseUserAttack,
    RaiseUserDefense,
    RaiseUserSpecialAttack = 14,
    RaiseUserEvasion = 17,
    NeverMisses,
    LowerTargetAttack,
    LowerTargetDefense,
    LowerTargetSpeed,
    LowerTargetAccuracy = 24,
    LowerTargetEvasion,
    // Unique
    Haze,
    Bide,
    // Generic
    Hit2To3TurnsThenConfuseUser,
    SwitchOutTarget,
    Hit2To5Times,
    Conversion, // Unique
    ChanceFlinchTarget,
    HealUserByHalfMaxHP,
    // Unique
    Toxic,
    PayDay,
    LightScreen,
    TriAttack,
    Rest,
    OneHitKO, // Generic
    RazorWind,
    SuperFang,
    DragonRage,
    // Generic
    SixteenthHP2To5Turns,
    IncreasedCritical,
    HitTwice,
    HalfRecoilIfMiss,
    // Unique
    Mist,
    FocusEnergy,
    // Generic
    QuarterRecoil,
    ConfuseTarget,
    RaiseUserAttack2,
    RaiseUserDefense2,
    RaiseUserSpeed2,
    RaiseUserSpecialAttack2,
    RaiseUserSpecialDefense2,
    Transform = 58, // Unique
    LowerTargetAttack2,
    LowerTargetDefense2,
    LowerTargetSpeed2,
    LowerTargetSpecialDefense2 = 63,
    Reflect = 66, // Unique
    PoisonTarget,
    ParalyzeTarget,
    ChanceLowerTargetAttack,
    ChanceLowerTargetDefense,
    ChanceLowerTargetSpeed,
    ChanceLowerTargetSpecialAttack,
    ChanceLowerTargetSpecialDefense,
    ChanceLowerTargetAccuracy,
    SkyAttack = 76, // Unique
    ChanceConfuseTarget,
    // Unique
    Twineedle,
    VitalThrow,
    Substitute,
    RechargeNextTurn, // Generic
    Rage,
    Mimic,
    Metronome,
    LeechSeed,
    Splash,
    Disable,
    UserLevelDamage, // Generic
    Psywave,
    Counter,
    Encore,
    PainSplit,
    Snore,
    Conversion2,
    GuaranteeNextMoveHit, // Generic
    Sketch,
    SleepTalk = 98,
    DestinyBond,
    MoreDamageWhenLessUserHP, // Generic
    Spite,
    FalseSwipe,
    // Generic
    CurePartyStatus,
    Fast,
    TripleKick, // Unique
    TakeTargetItem,
    PreventTargetLeaving,
    // Unique
    Nightmare,
    Minimize,
    Curse,
    PreventHitUser = 112, // Generic
    Spikes,
    ResetTargetEvadeDisableGhostImmunity, // Generic
    PerishSong,
    Sandstorm,
    Endure,
    DoubleEachSuccessiveUseMod5Turns, // Generic
    Swagger,
    FuryCutter,
    Attract,
    Return,
    Present,
    Frustration,
    Safeguard,
    ChanceBurnTargetThawUser, // Generic
    Magnitude,
    BatonPass,
    Pursuit,
    RapidSpin,
    Sonicboom,
    HealUserByHalfMaxHPWeather = 133, // Generic
    HiddenPower = 136,
    RainDance,
    SunnyDay,
    // Generic
    ChanceRaiseUserDefense,
    ChanceRaiseUserAttack,
    ChanceRaiseUserAllStats,
    // Unique
    BellyDrum = 143,
    PsychUp,
    MirrorCoat,
    SkullBash,
    Twister,
    Earthquake,
    HitTargetInTwoTurns, // Generic
    Gust,
    ChanceFlinchTargetDoubleMinimized, // Generic
    Solarbeam,
    Thunder,
    Teleport,
    BeatUp,
    Fly,
    DefenseCurl,
    FakeOut = 159,
    Uproar,
    Stockpile,
    SpitUp,
    Swallow,
    Hail = 165,
    Torment,
    Flatter,
    WillOWisp,
    Memento,
    Facade,
    FocusPunch,
    Smellingsalt,
    TargetUserThisTurn, // Generic
    NaturePower,
    Charge,
    Taunt,
    HelpingHand,
    SwapItems, // Generic
    RolePlay,
    Wish,
    Assist,
    Ingrain,
    Superpower,
    MagicCoat,
    Recycle,
    DoubleDamageIfUserHit, // Generic
    BrickBreak,
    Yawn,
    KnockOff,
    Endeavor,
    MoreDamageWhenMoreUserHP, // Generic
    SkillSwap,
    Imprison,
    Refresh,
    Grudge,
    Snatch,
    MoreDamageWhenTargetHeavier, // Generic
    SecretPower,
    ThirdRecoil, // Generic
    TeeterDance,
    BlazeKick,
    MudSport,
    PoisonFange,
    WeatherBall,
    // Generic
    LowerUserSpecialAttack2AfterDamage,
    LowerTargetAttackDefense,
    RaiseUserDefenseSpecialDefense,
    SkyUppercut, // Unique
    RaiseUserAttackDefense,
    IncreasedCriticalChancePoisonTarget,
    WaterSport, // Unique
    RaiseUserSpecialAttackSpecialDefense,
    RaiseUserAttackSpeed,
    // Unique
    Camouflage,
    Roost,
    Gravity,
    MiracleEye,
    WakeUpSlap,
    HammerArm,
    GyroBall,
    HealingWish,
    Brine,
    NaturalGift,
    Feint,
    DoubleIfTargetBerry, // Generic
    Tailwind,
    Accupressure,
    MetalBurst,
    UserSwitchOutAfterAttack, // Generic
    CloseCombat,
    Payback,
    Assurance,
    Embargo,
    Fling,
    PsychoShift,
    TrumpCard,
    HealBlock,
    MoreDamageWhenMoreTargetHP, // Generic
    PowerTrick,
    GastroAcid,
    LuckyChant,
    MeFirst,
    Copycat,
    PowerSwap,
    GuardSwap,
    Punishment,
    LastResort,
    WorrySeed,
    SuckerPunch,
    ToxicSpokes,
    HeartSwap,
    AquaRing,
    MagnetRise,
    FlareBlitz,
    Struggle,
    Dive,
    Dig,
    Surf,
    Defog,
    TrickRoom,
    Blizzard,
    Whirlpool,
    VoltTackle,
    Bounce,
    Captivate = 266,
    StealthRock,
    Chatter,
    PlateDriveType, // Generic
    HeadSmash,
    LunarDance,
    SeedFlare,
    ShadowForce,
    FireFang,
    IceFang,
    ThunderFang,
    ChanceRaiseUserSpecialAttack, // Generic
    HoneClaws,
    WideGuard,
    GuardSplit,
    PowerSplit,
    WonderRoom,
    UseTargetDefenseNotSpecial, // Generic
    Venoshock,
    Autotomize,
    Telekinesis,
    MagicRoom,
    SmackDown,
    AlwaysCritical, // Generic
    FlameBurst,
    QuiverDance,
    MoreDamageWithUserTargetWeightRatio, // Generic
    Synchronoise,
    ElectroBall,
    Soak,
    FlameCharge,
    AcidSpray,
    FoulPlay,
    SimpleBeam,
    Entrainment,
    AfterYou,
    Round,
    EchoedVoice,
    IgnoresTargetStatModifiers, // Generic
    ClearSmog,
    StoredPower,
    QuickGuard,
    AllySwitch,
    ShellSmash,
    HealPulse,
    Hex,
    SkyDrop,
    ShiftGear,
    SwitchOutTargetAfterDamage, // Generic
    Incinerate,
    Quash,
    Growth,
    Acrobatics,
    ReflectType,
    Retaliate,
    FinalGambit,
    TailGlow,
    Coil,
    Bestow,
    WaterPledge,
    FirePledge,
    GrassPledge,
    WorkUp,
    CottonGuard,
    RelicSong,
    Glaciate,
    FreezeShock,
    IceBurn,
    VCreate = 335,
    FusionFlare,
    FusionBolt,
    Hurricane,
}

impl Default for Effect {
    fn default() -> Self { Effect::Splash }
}

impl FromVeekun for Effect {
    type Intermediate = u16;

    fn from_veekun(value: u16) -> Option<Self> {
        Effect::from_repr(value)
    }
}
