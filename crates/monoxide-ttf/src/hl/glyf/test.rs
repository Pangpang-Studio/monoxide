#![cfg(test)]
use crate::hl::glyf::{QuadBezier, decode, encode};

#[test]
fn test_points_iter_1() {
    let mut line1 = QuadBezier::builder((0, 0));
    line1
        .line_to((100, 0))
        .line_to((100, 100))
        .line_to((0, 100))
        .line_to((0, 0))
        .close();
    let line1 = line1.build();

    let points: Vec<_> = line1.iter().collect();
    assert_eq!(
        points,
        vec![
            (true, (0, 0)),
            (true, (100, 0)),
            (true, (100, 100)),
            (true, (0, 100)),
        ]
    );
}

#[test]
fn test_points_iter_2() {
    let mut line2 = QuadBezier::builder((0, 0));
    line2
        .quad_to((50, 0), (100, 0))
        .quad_to((100, 50), (100, 100))
        .quad_to((50, 100), (0, 100))
        .quad_to((0, 50), (0, 0))
        .close();
    let line2 = line2.build();

    let points: Vec<_> = line2.iter().collect();
    assert_eq!(
        points,
        vec![
            (true, (0, 0)),
            (false, (50, 0)),
            (true, (100, 0)),
            (false, (100, 50)),
            (true, (100, 100)),
            (false, (50, 100)),
            (true, (0, 100)),
            (false, (0, 50)),
        ]
    );
}

#[test]
fn test_encode_decode() {
    let mut line1 = QuadBezier::builder((0, 0));
    line1
        .line_to((100, 0))
        .line_to((100, 100))
        .line_to((0, 100))
        .line_to((0, 0))
        .close();
    let line1 = line1.build();
    let mut line2 = QuadBezier::builder((0, 0));
    line2
        .quad_to((50, 0), (100, 0))
        .quad_to((100, 50), (100, 100))
        .quad_to((50, 100), (0, 100))
        .quad_to((0, 50), (0, 0))
        .close();
    let line2 = line2.build();
    let outlines = vec![line1, line2];

    let glyph = encode(&outlines).unwrap();

    println!("encoded: {:?}", &glyph);

    let decoded = decode(&glyph).unwrap();

    assert_eq!(outlines, decoded, "Encode-decode test failed.");
}

#[test]
fn test_encode_decode_large_coordinates() {
    let mut line1 = QuadBezier::builder((123, 456));
    line1
        .line_to((2048, 123))
        .quad_to((2048, 2048), (123, 2048))
        .quad_to((0, 2048), (0, -523))
        .quad_to((0, 0), (123, 456))
        .close();
    let line1 = line1.build();
    let outlines = vec![line1];

    let glyph = encode(&outlines).unwrap();

    println!("encoded: {:?}", &glyph);

    let decoded = decode(&glyph).unwrap();

    assert_eq!(outlines, decoded, "Encode-decode test failed.");
}
