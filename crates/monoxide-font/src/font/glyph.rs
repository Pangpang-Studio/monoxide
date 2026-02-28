macro_rules! glyph_mods {
    ( $( $vis:vis $mod:ident; )+ ) => {
        $(
            $vis mod $mod;
            pub use self::$mod::$mod;
        )+
    }
}

glyph_mods! {
    pub a;
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
    v;
    w;
    x;
    y;
    z;
    space;
    tofu;
}

use super::InputContext;
