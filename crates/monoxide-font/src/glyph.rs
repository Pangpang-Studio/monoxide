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
    digit::{eight, five, four, nine, one, seven, six, three, two, zero},
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
    sym::{
        apostrophe, asterisk, backslash, backtick, caret, colon, comma, curly_bracket_l,
        curly_bracket_r, dash, dollar, dot, equal, exclamation, greater, hash, less, pipe, plus,
        quotes, round_bracket_l, round_bracket_r, semicolon, slash, space, square_bracket_l,
        square_bracket_r, tofu, underscore,
    },
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
    ('!', exclamation),
    ('"', quotes),
    ('#', hash),
    ('$', dollar),
    ('\'', apostrophe),
    ('(', round_bracket_l),
    (')', round_bracket_r),
    ('*', asterisk),
    ('+', plus),
    (',', comma),
    ('-', dash),
    ('.', dot),
    ('/', slash),
    ('0', zero),
    ('1', one),
    ('2', two),
    ('3', three),
    ('4', four),
    ('5', five),
    ('6', six),
    ('7', seven),
    ('8', eight),
    ('9', nine),
    (':', colon),
    (';', semicolon),
    ('<', less),
    ('=', equal),
    ('>', greater),
    ('A', a_cap),
    ('B', b_cap),
    ('C', c_cap),
    ('D', d_cap),
    ('E', e_cap),
    ('F', f_cap),
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
    ('[', square_bracket_l),
    ('\\', backslash),
    (']', square_bracket_r),
    ('^', caret),
    ('_', underscore),
    ('`', backtick),
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
    ('{', curly_bracket_l),
    ('|', pipe),
    ('}', curly_bracket_r),
    // Cyrillic uppercase
    ('А', a_cap),
    ('В', b_cap),
    ('Е', e_cap),
    ('К', k_cap),
    ('М', m_cap),
    ('Н', h_cap),
    ('О', o_cap),
    ('Р', p_cap),
    ('С', c_cap),
    ('Т', t_cap),
    ('Х', x_cap),
    ('Э', cyrl_e_cap),
    // Cyrillic lowercase
    ('а', a),
    ('е', e),
    ('о', o),
    ('р', p),
    ('с', c),
    ('у', y),
    ('х', x),
    ('э', cyrl_e),
    // Extended Cyrillic uppercase
    ('І', i_cap),
    ('Ј', j_cap),
    ('Ѕ', s_cap),
    ('Ԛ', q_cap),
    ('Ԝ', w_cap),
    ('Ү', y_cap),
    // Extended Cyrillic lowercase
    ('і', i),
    ('ј', j),
    ('ѕ', s),
    ('һ', h),
    ('ԛ', q),
    ('ԝ', w),
    (char::REPLACEMENT_CHARACTER, tofu),
];

pub const WHITESPACES: &[char] = &[
    '\u{0009}', // horizontal tab
    '\u{000A}', // line feed
    '\u{000B}', // vertical tab
    '\u{000C}', // form feed
    '\u{000D}', // carriage return
    '\u{0020}', // space
    '\u{0085}', // next line
    '\u{00A0}', // no-break space
    '\u{1680}', // ogham space mark
    '\u{180E}', // mongolian vowel separator
    '\u{2000}', // en quad
    '\u{2001}', // em quad
    '\u{2002}', // en space
    '\u{2003}', // em space
    '\u{2004}', // three-per-em space
    '\u{2005}', // four-per-em space
    '\u{2006}', // six-per-em space
    '\u{2007}', // figure space
    '\u{2008}', // punctuation space
    '\u{2009}', // thin space
    '\u{200A}', // hair space
    '\u{2028}', // line separator
    '\u{2029}', // paragraph separator
    '\u{202F}', // narrow no-break space
    '\u{205F}', // medium mathematical space
    '\u{3000}', // ideographic space
];

#[cfg(test)]
mod test;
