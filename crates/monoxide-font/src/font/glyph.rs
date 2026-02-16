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
    f;
    h;
    i;
    pub j;
    k;
    l;
    m;
    n;
    o;
    p;
    q;
    r;
    t;
    u;
    x;
    space;
    tofu;
}

use super::InputContext;

// TODO: Finish those modules and move them above.
pub mod a;
