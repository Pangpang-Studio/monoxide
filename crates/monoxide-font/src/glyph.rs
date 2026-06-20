mod a;
mod b;
mod c;
mod d;
mod digit;
mod e;
mod f;
mod g;
mod h;
mod i;
mod j;
mod k;
mod l;
mod m;
mod n;
mod o;
mod p;
mod q;
mod r;
mod s;
mod sym;
mod t;
mod u;
mod v;
mod w;
mod x;
mod y;
mod z;

use monoxide_script::ast::Glyph;

pub use self::{
    a::{a, a_cap},
    b::{b, b_cap},
    c::{c, c_cap},
    d::{d, d_cap},
    digit::{four, one, seven, zero},
    e::{e, e_cap},
    f::{f, f_cap},
    g::{g, g_cap},
    h::{h, h_cap},
    i::{i, i_cap},
    j::{j, j_cap},
    k::{k, k_cap},
    l::{l, l_cap},
    m::{m, m_cap},
    n::{n, n_cap},
    o::{o, o_cap},
    p::{p, p_cap},
    q::{q, q_cap},
    r::{r, r_cap},
    s::{s, s_cap},
    sym::{backslash, slash, space, tofu},
    t::{t, t_cap},
    u::{u, u_cap},
    v::{v, v_cap},
    w::{w, w_cap},
    x::{x, x_cap},
    y::{y, y_cap},
    z::{z, z_cap},
};
use crate::InputContext;

pub type GlyphFn = fn(&InputContext) -> Glyph;

pub const GLYPH_FNS: &[(char, GlyphFn)] = &[
    (' ', space),
    ('/', slash),
    ('0', zero),
    ('1', one),
    ('4', four),
    ('7', seven),
    ('A', a_cap),
    ('B', b_cap),
    ('C', c_cap),
    ('D', d_cap),
    ('F', f_cap),
    ('E', e_cap),
    ('G', g_cap),
    ('H', h_cap),
    ('I', i_cap),
    ('J', j_cap),
    ('K', k_cap),
    ('L', l_cap),
    ('M', m_cap),
    ('N', n_cap),
    ('O', o_cap),
    ('P', p_cap),
    ('Q', q_cap),
    ('R', r_cap),
    ('S', s_cap),
    ('T', t_cap),
    ('U', u_cap),
    ('V', v_cap),
    ('W', w_cap),
    ('X', x_cap),
    ('Y', y_cap),
    ('Z', z_cap),
    ('\\', backslash),
    ('a', a),
    ('b', b),
    ('c', c),
    ('d', d),
    ('e', e),
    ('f', f),
    ('g', g),
    ('h', h),
    ('i', i),
    ('j', j),
    ('k', k),
    ('l', l),
    ('m', m),
    ('n', n),
    ('o', o),
    ('p', p),
    ('q', q),
    ('r', r),
    ('s', s),
    ('t', t),
    ('u', u),
    ('v', v),
    ('w', w),
    ('x', x),
    ('y', y),
    ('z', z),
    (char::REPLACEMENT_CHARACTER, tofu),
];

#[cfg(test)]
mod test;
