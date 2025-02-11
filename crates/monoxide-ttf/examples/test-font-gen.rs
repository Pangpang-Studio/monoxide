use std::time::SystemTime;

use monoxide_ttf::{
    hl::{self, cmap::SeqMapping},
    model::{
        cmap,
        glyf::{
            self,
            simple::{Coord, OutlineFlag, SimpleGlyph},
        },
        head::{self, HeaderFlags, MacStyle},
        hhea, hmtx, name, os2,
    },
};

fn main() {
    // Here we create a very simple test font that maps the entire Unicode Basic Multilingual Plane
    // to a single glyph that is a square with side length 1024 units.

    let head = head::Table {
        font_revision: 0,
        checksum_adjustment: 0,
        flags: HeaderFlags::empty(),
        units_per_em: 1024,
        created: SystemTime::now(),
        modified: SystemTime::now(),
        x_min: 0,
        y_min: 0,
        x_max: 1024,
        y_max: 1024,
        mac_style: MacStyle::REGULAR,
        lowest_rec_ppem: 0,
    };

    let hhea = hhea::Table {
        ascender: 1024,
        descender: -256,
        line_gap: 0,
        advance_width_max: 1024,
        min_left_side_bearing: 0,
        min_right_side_bearing: 0,
        x_max_extent: 1024,
        caret_slope_rise: 1,
        caret_slope_run: 0,
        caret_offset: 0,
        metric_data_format: 0,
        number_of_hmetrics: 1,
    };

    let hmtx = hmtx::Table {
        metrics: vec![hmtx::LongHorizontalMetric {
            advance_width: 1024,
            left_side_bearing: 0,
        }],
        left_side_bearings: vec![],
    };

    let cmap_hl = hl::cmap::Table {
        subtables: vec![vec![SeqMapping {
            start_code: 65,
            len: 1,
            glyph_id: 0,
        }]],
        mapping: vec![(hl::cmap::Encoding::Unicode, 0)],
    };
    let cmap = cmap::Table::from_raw(cmap_hl);

    let name = name::Table {
        records: [(
            name::Lang("en-US".into()),
            vec![
                name::NameRecord {
                    name_id: name::NameId::FullFontName,
                    value: "Test Font".into(),
                },
                name::NameRecord {
                    name_id: name::NameId::FontFamilyName,
                    value: "Test".into(),
                },
                name::NameRecord {
                    name_id: name::NameId::UniqueFontIdentifier,
                    value: "pangpangstudio.test_font".into(),
                },
            ],
        )]
        .into_iter()
        .collect(),
    };

    let os2 = os2::Table {
        x_avg_char_width: 1024,
        us_weight_class: 400,
        usage_permission: os2::UsagePermissionKind::EditableEmbedding,
        fs_type: os2::FsTypeUpper::Nothing,
        y_subscript_x_size: 512,
        y_subscript_y_size: 512,
        y_subscript_x_offset: 0,
        y_subscript_y_offset: 128,
        y_superscript_x_size: 512,
        y_superscript_y_size: 512,
        y_superscript_x_offset: 0,
        y_superscript_y_offset: 768,
        y_strikeout_size: 50,
        y_strikeout_position: 258,
        s_family_class: os2::SFamilyClass::NoClassification,
        panose_classification: os2::PanroseClassification {
            family_type: 2,
            serif_style: 0,
            weight: 5,
            proportion: 3,
            contrast: 0,
            stroke_variation: 0,
            arm_style: 0,
            letterform: 0,
            midline: 0,
            x_height: 0,
        },
        unicode_range1: 0,
        unicode_range2: 0,
        unicode_range3: 0,
        unicode_range4: 0,
        ach_vend_id: *b"TEST",
        fs_selection: os2::FsSelectionKind::Regular,
        first_char_index: 0,
        last_char_index: 0xffff,
        s_typo_ascender: 1024,
        s_typo_descender: -256,
        s_typo_line_gap: 0,
        us_win_ascent: 1024,
        us_win_descent: 256,
        ul_code_page_range1: 0,
        ul_code_page_range2: 0,
        sx_height: 0,
        s_cap_height: 0,
        us_default_char: 0,
        us_break_char: 0,
        us_max_context: 0,
    };

    let glyf = glyf::Table {
        glyphs: vec![glyf::Glyph::Simple(SimpleGlyph {
            common: glyf::GlyphCommon {
                x_min: 0,
                y_min: 0,
                x_max: 1024,
                y_max: 1024,
            },
            end_points_of_countours: vec![3],
            instructions: vec![],
            flags: vec![glyf::simple::FlagOrRepeat::Repeat(OutlineFlag::ON_CURVE, 4)],
            x_coords: vec![
                Coord::Long(0),
                Coord::Long(1024),
                Coord::Long(1024),
                Coord::Long(0),
            ],
            y_coords: vec![
                Coord::Long(0),
                Coord::Long(0),
                Coord::Long(1024),
                Coord::Long(1024),
            ],
        })],
    };
    let loca = hl::loca::glyf_to_loca(&glyf);
    let maxp = hl::maxp::glyf_to_maxp(&glyf);

    let tt_tables = monoxide_ttf::model::TrueTypeTables { glyf, loca, maxp };

    let f = monoxide_ttf::model::FontFile {
        head,
        hhea,
        hmtx,
        cmap,
        name,
        os2,
        outline: monoxide_ttf::model::Outline::TrueType(tt_tables),
    };

    let out_path = std::env::args().nth(1).unwrap();
    let mut file = std::fs::File::create(out_path).unwrap();
    f.write(&mut file).unwrap();
}
