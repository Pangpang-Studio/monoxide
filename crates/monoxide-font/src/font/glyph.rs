macro_rules! glyph_mods {
    ( $( $vis:vis $mod:ident; )+ ) => {
        $(
            $vis mod $mod;
            pub use self::$mod::$mod;
        )+
    }
}

glyph_mods! {
    b;
    c;
    d;
    h;
    i;
    k;
    l;
    n;
    o;
    p;
    q;
    tofu;
}

use super::InputContext;

pub mod a;
