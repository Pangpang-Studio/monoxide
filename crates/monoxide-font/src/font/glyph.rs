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
    b::b,
    c::c,
    d::d,
    digit::{one, zero},
    e::e,
    f::f,
    g::g,
    h::h,
    i::i,
    j::{j, j_cap},
    k::k,
    l::l,
    m::m,
    n::n,
    o::{o, o_cap},
    p::p,
    q::q,
    r::r,
    s::s,
    sym::{backslash, slash, space, tofu},
    t::{t, t_cap},
    u::u,
    v::v,
    w::w,
    x::x,
    y::y,
    z::z,
};
use crate::InputContext;

pub type GlyphFn = fn(&InputContext) -> Glyph;

pub const GLYPH_FNS: &[(char, GlyphFn)] = &[
    (' ', space),
    ('/', slash),
    ('0', zero),
    ('1', one),
    ('A', a_cap),
    ('J', j_cap),
    ('O', o_cap),
    ('T', t_cap),
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
