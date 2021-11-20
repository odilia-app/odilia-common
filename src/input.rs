#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct KeyBinding {
    pub key: char,
    pub mods: Modifiers,
    pub repeat: u8,
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
