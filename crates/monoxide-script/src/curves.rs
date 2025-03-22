use rquickjs::Ctx;
use spiro::{SpiroCP, SpiroCpTy};

use crate::utils::ChainThis;

#[rquickjs::class]
#[derive(rquickjs::JsLifetime)]
pub struct SpiroBuilder {
    points: Vec<SpiroCP>,
}

impl<'js> rquickjs::class::Trace<'js> for SpiroBuilder {
    fn trace<'a>(&self, _tracer: rquickjs::class::Tracer<'a, 'js>) {}
}

impl SpiroBuilder {
    pub fn new() -> Self {
        Self { points: Vec::new() }
    }

    pub fn push_pt(&mut self, x: f64, y: f64, kind: SpiroCpTy) {
        self.points.push(SpiroCP { x, y, ty: kind });
    }
}

impl Default for SpiroBuilder {
    fn default() -> Self {
        Self::new()
    }
}

type ChainSpiroBuilder<'js> = ChainThis<'js, SpiroBuilder>;

#[rquickjs::methods]
impl SpiroBuilder {
    #[qjs(constructor)]
    fn ctor() -> Self {
        Self::new()
    }

    pub fn corner<'js>(
        this: ChainSpiroBuilder<'js>,
        cx: Ctx<'js>,
        x: f64,
        y: f64,
    ) -> rquickjs::Result<ChainSpiroBuilder<'js>> {
        this.with_mut(&cx, |mut_this| mut_this.push_pt(x, y, SpiroCpTy::Corner))?;
        Ok(this)
    }
    pub fn g4<'js>(
        this: ChainSpiroBuilder<'js>,
        cx: Ctx<'js>,
        x: f64,
        y: f64,
    ) -> rquickjs::Result<ChainSpiroBuilder<'js>> {
        this.with_mut(&cx, |mut_this| mut_this.push_pt(x, y, SpiroCpTy::G4))?;
        Ok(this)
    }

    pub fn g2<'js>(
        this: ChainSpiroBuilder<'js>,
        cx: Ctx<'js>,
        x: f64,
        y: f64,
    ) -> rquickjs::Result<ChainSpiroBuilder<'js>> {
        this.with_mut(&cx, |mut_this| mut_this.push_pt(x, y, SpiroCpTy::G2))?;
        Ok(this)
    }

    pub fn left<'js>(
        this: ChainSpiroBuilder<'js>,
        cx: Ctx<'js>,
        x: f64,
        y: f64,
    ) -> rquickjs::Result<ChainSpiroBuilder<'js>> {
        this.with_mut(&cx, |mut_this| mut_this.push_pt(x, y, SpiroCpTy::Left))?;
        Ok(this)
    }

    pub fn right<'js>(
        this: ChainSpiroBuilder<'js>,
        cx: Ctx<'js>,
        x: f64,
        y: f64,
    ) -> rquickjs::Result<ChainSpiroBuilder<'js>> {
        this.with_mut(&cx, |mut_this| mut_this.push_pt(x, y, SpiroCpTy::Right))?;
        Ok(this)
    }

    pub fn anchor<'js>(
        this: ChainSpiroBuilder<'js>,
        cx: Ctx<'js>,
        x: f64,
        y: f64,
    ) -> rquickjs::Result<ChainSpiroBuilder<'js>> {
        this.with_mut(&cx, |mut_this| mut_this.push_pt(x, y, SpiroCpTy::Anchor))?;
        Ok(this)
    }

    pub fn handle<'js>(
        this: ChainSpiroBuilder<'js>,
        cx: Ctx<'js>,
        x: f64,
        y: f64,
    ) -> rquickjs::Result<ChainSpiroBuilder<'js>> {
        this.with_mut(&cx, |mut_this| mut_this.push_pt(x, y, SpiroCpTy::Handle))?;
        Ok(this)
    }

    pub fn end<'js>(
        this: ChainSpiroBuilder<'js>,
        cx: Ctx<'js>,
        x: f64,
        y: f64,
    ) -> rquickjs::Result<ChainSpiroBuilder<'js>> {
        this.with_mut(&cx, |mut_this| mut_this.push_pt(x, y, SpiroCpTy::End))?;
        Ok(this)
    }

    pub fn open<'js>(
        this: ChainSpiroBuilder<'js>,
        cx: Ctx<'js>,
        x: f64,
        y: f64,
    ) -> rquickjs::Result<ChainSpiroBuilder<'js>> {
        this.with_mut(&cx, |mut_this| mut_this.push_pt(x, y, SpiroCpTy::Open))?;
        Ok(this)
    }

    pub fn end_open<'js>(
        this: ChainSpiroBuilder<'js>,
        cx: Ctx<'js>,
        x: f64,
        y: f64,
    ) -> rquickjs::Result<ChainSpiroBuilder<'js>> {
        this.with_mut(&cx, |mut_this| mut_this.push_pt(x, y, SpiroCpTy::EndOpen))?;
        Ok(this)
    }
}
