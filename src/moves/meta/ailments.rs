use crate::enums::*;
use crate::FromVeekun;

/// Aka status condition; an ailment caused by a move.
///
/// > [*[From
/// > Bulbapedia:]*](https://bulbapedia.bulbagarden.net/wiki/Status_condition)
/// > Status conditions (Japanese: 状態異常 abnormal condition), also referred
/// > to as status problems or status ailments, affect a Pokémon's ability to
/// > battle. There are three kinds of status. The first are non-volatile, the
/// > second are volatile, and the third lasts while a Pokémon is in battle. The
/// > Pokérus is a similar but unrelated concept.
#[EnumRepr(type = "i8")]
pub enum Ailment {
    /// Some special ailment: used by Tri Attack, Telekinesis, and Smack Down.
    Unknown = -1,
    /// No ailment.
    None,
    /// A paralyzed Pokémon has a 25% chance of not being able to move, and its
    /// speed is decreased to 25% of its normal value.
    Paralysis,
    /// A sleeping Pokémon cannot move. Sleep normally lasts 2-5 turns.
    Sleep,
    /// A frozen Pokémon cannot move while frozen, but has a 20% chance of
    /// thawing each time it attempts to move.
    Freeze,
    /// A burned Pokémon takes 1/8 of its max HP at the end of each turn, and
    /// damage it does with phyisical moves is halved.
    Burn,
    /// A poisoned Pokémon takes 1/8 of its max HP at the end of each turn.
    ///
    /// A *badly* poisoned Pokémon takes *n*/16 of its max HP at the end of each
    /// turn, where *n* is the number of turns since the poisoning, starting at
    /// 1\. If a badly poisoned Pokémon is switched out, *n* resets to 1. At the
    /// end of a battle, bad poisoning becomes regular poisoning.
    Poison,
    /// A confused Pokémon will sometimes hurt itself when attempting to attack.
    ///
    /// > [*[From
    /// > Bulbapedia:]*](https://bulbapedia.bulbagarden.net/wiki/Status_condition#Confusion)
    /// > The confused condition causes a Pokémon to sometimes hurt itself in
    /// > its confusion instead of executing a selected move.  [The chance to
    /// > hurt itself is 50%.] The damage is done as if the Pokémon attacked
    /// > itself with a 40-power typeless physical attack (without the
    /// > possibility of a critical hit).
    /// >
    /// > Confusion wears off after 1-4 attacking turns. This means that turns
    /// > recharging, such as after using Hyper Beam, and turns unable to
    /// > attack, such as from paralysis, will not lower the remaining number of
    /// > turns of confusion. However, a sleeping Pokémon may hurt itself in
    /// > confusion if using a move such as Snore or Sleep Talk. Multi-turn
    /// > attacks such as Fly and Dive require confusion to be checked both
    /// > turns, further reducing the chance of a successful attack.
    Confusion,
    /// An infatuated Pokémon has a 50% chance of not being able to attack.
    Infatuation,
    /// A trapped Pokémon cannot switch out.
    Trap,
    /// A Pokémon under the effect of Nightmare takes 1/4 of its max HP at the
    /// end of each turn. The nightmare ends when the Pokémon wakes up.
    Nightmare,
    /// A tormented Pokémon cannot use the same move consecutively.
    Torment = 12,
    /// A Pokémon cannot use a move that has been disabled. Disable lasts 4
    /// turns.
    Disable,
    /// A drowsy Pokémon will fall asleep next turn if it does not switch out.
    Yawn,
    /// A Pokémon under the effect of Heal Block cannot heal for five turns.
    HealBlock,
    /// One of the Pokémon's types (Ghost or Dark) has its immunities disabled.
    NoTypeImmunity = 17,
    /// A seeded Pokémon takes 1/8 of its max HP at the end of each turn,
    /// healing the Pokémon in the position of the seeder the same amount.
    LeechSeed,
    /// A embargoed Pokémon cannot use items for 5 turns.
    Embargo,
    /// A Pokémon under the effect of Perish Song will faint in three turns
    /// unless switched out.
    PerishSong,
    /// A rooted Pokémon restores 1/16 of its max HP at the end of each turn,
    /// but cannot switch out.
    Ingrain,
}

impl Ailment {
    /// True if the ailment does not persist on switching out.
    pub fn volatile(self) -> bool {
        match self.repr() {
            1 ..= 5 => false,
            _ => true,
        }
    }
}

impl Default for Ailment {
    fn default() -> Self { Ailment::Unknown }
}

impl FromVeekun for Ailment {
    type Intermediate = i8;

    fn from_veekun(value: i8) -> Option<Self> {
        Ailment::from_repr(value)
    }
}
