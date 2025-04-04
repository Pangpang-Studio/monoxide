use rquickjs::{Class, Ctx};

pub mod ast;
mod eval;
mod js;

#[cfg(test)]
mod test {
    use rquickjs::{Context, Runtime};

    #[test]
    fn test_basic() {
        let rt = Runtime::new().unwrap();
        let cx = Context::full(&rt).unwrap();
        cx.with(|cx| {
            super::js::import_globals(&cx);

            cx.eval::<(), _>(
                r"
new SpiroBuilder()
    .corner(0, 0)
    .g4(1, 1)
    .g2(2, 2)
    .flat(3, 3)
    .curl(4, 4)
    .anchor(5, 5)
            
            ",
            )
            .map_err(|e| cx.catch())
            .unwrap();
        });
    }
}
