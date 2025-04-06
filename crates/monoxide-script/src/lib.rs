pub mod ast;
pub mod eval;
pub mod js;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FontParamSettings {
    width: f64,
    x_height: f64,
    descender: f64,
    cap_height: f64,
}

#[cfg(test)]
mod test {
    use rquickjs::{
        loader::{BuiltinResolver, ModuleLoader},
        Context, Module, Runtime, Value,
    };

    use crate::js::{insert_globals, ContextAttachment};

    #[rquickjs::function]
    fn print(v: Value<'_>) {
        if let Some(o) = v.as_object() {
            dbg!(o);
            for p in o.props::<String, Value>() {
                let (k, v) = p.unwrap();
                dbg!(k, v);
            }
        } else {
            dbg!(v);
        }
    }

    #[test]
    fn test_basic() {
        let rt = Runtime::new().unwrap();
        let cx = Context::full(&rt).unwrap();

        let mut module_resolver = BuiltinResolver::default();
        module_resolver.add_module("monoxide");
        let mut module_loader = ModuleLoader::default();
        module_loader.add_module("monoxide", crate::js::MonoxideModule);
        rt.set_loader(module_resolver, module_loader);

        cx.with(|cx| {
            let cx_att = ContextAttachment::new(
                cx.clone(),
                crate::FontParamSettings {
                    width: 0.5,
                    x_height: 0.6,
                    descender: 0.2,
                    cap_height: 1.,
                },
            )
            .expect("Cannot create attachment");
            cx.store_userdata(cx_att).unwrap();

            insert_globals(cx.clone()).unwrap();

            cx.globals().set("print", js_print).unwrap();

            let m = Module::evaluate(
                cx.clone(),
                "font",
                r"
import { bezier, spiro, settings, glyph } from 'monoxide'


let g = glyph.simple(b => {
    b.add(
        bezier(0, 0)
            .lineTo(1, 0)
            .lineTo(1, settings.width)
            .lineTo(0, settings.width)
            .build())
})
glyph.assignChar(g, 'c')

// spiro()
//     .corner(0, 0)
//     .g4(1, 1)
//     .g2(2, 2)
//     .flat(3, 3)
//     .curl(4, 4)
//     .anchor(5, 5)
//     .build()
            ",
            )
            .map_err(|_| cx.catch())
            .unwrap();
            let _: () = m.finish().unwrap();

            let ud = cx.userdata::<ContextAttachment>().unwrap();
            let font = ud.take();
            assert_eq!(font.glyphs.len(), 1);
            assert_eq!(font.cmap.len(), 1);
        });
    }
}
