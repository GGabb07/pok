pub const IMPLEMENTED_PKMN: usize = 9;

#[rustfmt::skip]
pub const PKMN_NAMES: [&str; IMPLEMENTED_PKMN] = [
    "Turtwig", "Grotle", "Torterra",
    "Chimchar", "Monferno", "Infernape",
    "Piplup", "Prinplup", "Empoleon",
];

type Ps = u8;
type Att = u8;
type Def = u8;
type SpAtt = u8;
type SpDef = u8;
type Speed = u8;

#[rustfmt::skip]
pub const PKMN_STATS: [(Ps, Att, Def, SpAtt, SpDef, Speed); IMPLEMENTED_PKMN] = [
    (55,68,64,45,55,31),(75,89,85,55,65,36),(95,109,105,75,85,56),
    (44,58,44,58,44,61),(64,78,52,78,52,81),(76,104,71,104,71,108),
    (53,51,53,61,56,40),(64,66,68,81,76,50),(84,86,88,111,101,60),
];
