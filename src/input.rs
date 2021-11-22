use crate::modes::ScreenReaderMode;
use crate::errors::KeyFromStrError;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct KeyBinding {
    pub key: char,
    pub mods: Modifiers,
    pub repeat: u8,
    pub mode: ScreenReaderMode,
}

impl FromStr for KeyBinding {
    type Err = KeyFromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use KeyFromStrError as E;
        let mode = ScreenReaderMode::CommandMode;

        let mut parts = s.split('+').rev();
        let key_and_repeat = parts.next().ok_or(E::EmptyString)?.trim_end();
        let mut subparts = key_and_repeat.split(':');

        let key = subparts.next().ok_or(E::NoKey)?;
        let mut chars = key.chars();
        let key = chars.next().ok_or(E::NoKey)?;
        if chars.next().is_some() {
            return Err(E::InvalidKey);
        }

        let repeat: u8 = subparts
            .next()
            .map(|r| r.parse())
            .unwrap_or(Ok(1))
            .map_err(|_| E::InvalidRepeat)?;

        let mut mods = Modifiers::empty();
        for m in parts {
            // Yeh ... it's not pretty ... but would an if chain really look any better?
            mods |= match m {
                m if m.eq_ignore_ascii_case("Odilia") => Modifiers::ODILIA,
                m if m.eq_ignore_ascii_case("Applications") => Modifiers::APPLICATIONS,

                m if m.eq_ignore_ascii_case("LeftControl") => Modifiers::CONTROL_L,
                m if m.eq_ignore_ascii_case("RightControl") => Modifiers::CONTROL_R,
                m if m.eq_ignore_ascii_case("Control") => Modifiers::CONTROL,

                m if m.eq_ignore_ascii_case("LeftAlt") => Modifiers::ALT_L,
                m if m.eq_ignore_ascii_case("RightAlt") => Modifiers::ALT_R,
                m if m.eq_ignore_ascii_case("Alt") => Modifiers::ALT,

                m if m.eq_ignore_ascii_case("LeftShift") => Modifiers::SHIFT_L,
                m if m.eq_ignore_ascii_case("RightShift") => Modifiers::SHIFT_R,
                m if m.eq_ignore_ascii_case("Shift") => Modifiers::SHIFT,

                m if m.eq_ignore_ascii_case("LeftMeta") => Modifiers::META_L,
                m if m.eq_ignore_ascii_case("RightMeta") => Modifiers::META_R,
                m if m.eq_ignore_ascii_case("Meta") => Modifiers::META,

                _ => return Err(E::InvalidModifier),
            };
        }

        Ok(Self { key, mods, repeat, mode })
    }
}

bitflags::bitflags! {
    #[derive(Default)]
    pub struct Modifiers: u16 {
        /// Usually capslock, insert, or kp-insert
        const ODILIA = 1 << 0;

        const CONTROL_L = 1 << 1;
        const CONTROL_R = 1 << 2;
        const CONTROL = 1 << 1 | 1 << 2;

        const ALT_L = 1 << 3;
        const ALT_R = 1 << 4;
        const ALT = 1 << 3 | 1 << 4;

        const SHIFT_L = 1 << 5;
        const SHIFT_R = 1 << 6;
        const SHIFT = 1 << 5 | 1 << 6;

        const META_L = 1 << 7;
        const META_R = 1 << 8;
        const META = 1 << 7 | 1 << 8;

        const APPLICATIONS = 1 << 9;
    }
}

impl Modifiers {
    // Using `self` instead of `&self` here is fine, `Self` is `Copy`.
    //
    #[inline]
    pub fn control(self) -> bool {
        self.intersects(Self::CONTROL)
    }

    #[inline]
    pub fn alt(self) -> bool {
        self.intersects(Self::ALT)
    }

    #[inline]
    pub fn shift(self) -> bool {
        self.intersects(Self::SHIFT)
    }

    #[inline]
    pub fn meta(self) -> bool {
        self.intersects(Self::META)
    }
    // Are these two useful? Almost certainly not! Am I going to leave them in? You bet ya!
    // You never know, addon developers are clever!

    #[inline]
    pub fn left(self) -> bool {
        self.intersects(Self::CONTROL_L | Self::ALT_L | Self::SHIFT_L | Self::META_L)
    }

    #[inline]
    pub fn right(self) -> bool {
        self.intersects(Self::CONTROL_R | Self::ALT_R | Self::SHIFT_R | Self::META_R)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_key_binding() {
        // simple
        let kb: KeyBinding = "Odilia+h".parse().unwrap();
        assert_eq!(kb.key, 'h');
        assert_eq!(kb.mods, Modifiers::ODILIA);
        assert_eq!(kb.repeat, 1);
        // Complex
        let kb: KeyBinding = "Control+Shift+Alt+Meta+Applications+Odilia+s:3"
            .parse()
            .unwrap();
        assert_eq!(kb.key, 's');
        assert_eq!(kb.mods, Modifiers::all());
        assert_eq!(kb.repeat, 3);
        // Left only
        let kb: KeyBinding = "LeftControl+LeftShift+LeftAlt+LeftMeta+.:2"
            .parse()
            .unwrap();
        assert_eq!(kb.key, '.');
        assert_eq!(
            kb.mods,
            Modifiers::CONTROL_L | Modifiers::ALT_L | Modifiers::SHIFT_L | Modifiers::META_L
        );
        assert_eq!(kb.repeat, 2);
        let kb: KeyBinding = "RightControl+RightShift+RightAlt+RightMeta+.:2"
            .parse()
            .unwrap();
        assert_eq!(kb.key, '.');
        assert_eq!(
            kb.mods,
            Modifiers::CONTROL_R | Modifiers::ALT_R | Modifiers::SHIFT_R | Modifiers::META_R
        );
        assert_eq!(kb.repeat, 2);
    }
}
