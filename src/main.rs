#![allow(dead_code)]
#![allow(unused_variables)]

// Races
struct Dwarf {
    name: String,
}
struct HalfOrc {
    name: String,
}
struct Elf {
    name: String,
}
struct Human {
    name: String,
}
struct HalfElf {
    name: String,
}

pub trait Constitution {
    fn constitution_bonus(&self) -> u8 {
        0
    }
}
impl Constitution for Dwarf {
    fn constitution_bonus(&self) -> u8 {
        2
    }
}
impl Constitution for HalfOrc {
    fn constitution_bonus(&self) -> u8 {
        1
    }
}
impl Constitution for Elf {}
impl Constitution for HalfElf {}
impl Constitution for Human {}

pub trait Elvish {}
impl Elvish for Elf {}
impl Elvish for HalfElf {}

pub fn speak_elvish<T: Elvish>(character: T) -> String {
    String::from("Yes")
}

// Spells
struct Cantrip {}
struct Enchantment {}
struct Transmutation {}
struct Necromancy {}

pub trait Cast {
    fn cast(&self);
}

impl Cast for Cantrip {
    fn cast(&self) {
        // Details for casting a Cantrip Spell
    }
}
impl Cast for Transmutation {
    fn cast(&self) {
        // Details for casting a Transmutation Spell
    }
}
impl Cast for Enchantment {
    fn cast(&self) {
        // Details for casting a Enchantment Spell
    }
}
impl Cast for Necromancy {
    fn cast(&self) {
        // Details for casting a Necromancy Spell
    }
}

struct SpellBook {
    pub spells: Vec<Box<dyn Cast>>,
}
impl SpellBook {
    pub fn cast_all(&self) {
        for spell in self.spells.iter() {
            spell.cast();
        }
    }
}

fn main() {
    let my_dwarf = Dwarf {
        name: String::from("NellDwarf"),
    };
    let my_half_orc = HalfOrc {
        name: String::from("NellOrc"),
    };
    let my_elf = Elf {
        name: String::from("NellElf"),
    };
    let my_human = Human {
        name: String::from("Nell"),
    };
    let my_half_elf = HalfElf {
        name: String::from("Nell"),
    };

    my_dwarf.constitution_bonus();
    my_half_orc.constitution_bonus();
    my_elf.constitution_bonus();
    my_human.constitution_bonus();
    my_half_elf.constitution_bonus();

    speak_elvish(my_elf);
    speak_elvish(my_half_elf);

    let spell_book = SpellBook {
        spells: vec![
            Box::new(Cantrip {}),
            Box::new(Transmutation {}),
            Box::new(Enchantment {}),
            Box::new(Necromancy {}),
        ],
    };
    spell_book.cast_all();
}
