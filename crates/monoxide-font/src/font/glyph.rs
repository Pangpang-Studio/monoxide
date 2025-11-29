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
    e;
    h;
    i;
    k;
    l;
    m;
    n;
    o;
    p;
    q;
    u;
    x;
    space;
    tofu;
}

use super::InputContext;

pub mod a;
