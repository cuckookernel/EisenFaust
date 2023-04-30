use std::fmt::{Debug, Formatter, Error};

use bitflags::bitflags;

bitflags! {
    /// Represents key modifiers (shift, control, alt).
    ///
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize, PartialEq))]
    pub struct KeyModifiers: u8 {
        const SHIFT = 0b0000_0001;
        const CTRL = 0b0000_0010;
        const ALT = 0b0000_0100;
    }
}

impl Debug for KeyModifiers {
    fn fmt(&self, f: &mut Formatter<'_> ) -> Result<(), Error> {
        let mut parts = vec!();

        if self.contains(KeyModifiers::SHIFT) { parts.push("shift") }
        if self.contains(KeyModifiers::CTRL) { parts.push("ctrl") }
        if self.contains(KeyModifiers::ALT) { parts.push("alt") }

        let concat = parts.join("+");

        write!(f, "{}", concat )
    }
}