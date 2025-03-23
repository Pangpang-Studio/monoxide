use rquickjs::Class;
use spiro::{SpiroCP, SpiroCpTy};

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

#[rquickjs::methods]
impl SpiroBuilder {
    #[qjs(constructor)]
    fn ctor() -> Self {
        Self::new()
    }

    pub fn corner(this: Class<'_, Self>, x: f64, y: f64) -> rquickjs::Result<Class<'_, Self>> {
        let mut this_ = this.borrow_mut();
        this_.push_pt(x, y, SpiroCpTy::Corner);
        drop(this_);
        Ok(this)
    }

    pub fn g4(this: Class<'_, Self>, x: f64, y: f64) -> rquickjs::Result<Class<'_, Self>> {
        let mut this_ = this.borrow_mut();
        this_.push_pt(x, y, SpiroCpTy::G4);
        drop(this_);
        Ok(this)
    }

    pub fn g2(this: Class<'_, Self>, x: f64, y: f64) -> rquickjs::Result<Class<'_, Self>> {
        let mut this_ = this.borrow_mut();
        this_.push_pt(x, y, SpiroCpTy::G2);
        drop(this_);
        Ok(this)
    }

    pub fn flat(this: Class<'_, Self>, x: f64, y: f64) -> rquickjs::Result<Class<'_, Self>> {
        let mut this_ = this.borrow_mut();
        this_.push_pt(x, y, SpiroCpTy::Left);
        drop(this_);
        Ok(this)
    }

    pub fn curl(this: Class<'_, Self>, x: f64, y: f64) -> rquickjs::Result<Class<'_, Self>> {
        let mut this_ = this.borrow_mut();
        this_.push_pt(x, y, SpiroCpTy::Right);
        drop(this_);
        Ok(this)
    }

    pub fn anchor(this: Class<'_, Self>, x: f64, y: f64) -> rquickjs::Result<Class<'_, Self>> {
        let mut this_ = this.borrow_mut();
        this_.push_pt(x, y, SpiroCpTy::Anchor);
        drop(this_);
        Ok(this)
    }

    pub fn handle(this: Class<'_, Self>, x: f64, y: f64) -> rquickjs::Result<Class<'_, Self>> {
        let mut this_ = this.borrow_mut();
        this_.push_pt(x, y, SpiroCpTy::Handle);
        drop(this_);
        Ok(this)
    }

    pub fn end(this: Class<'_, Self>, x: f64, y: f64) -> rquickjs::Result<Class<'_, Self>> {
        let mut this_ = this.borrow_mut();
        this_.push_pt(x, y, SpiroCpTy::End);
        drop(this_);
        Ok(this)
    }

    pub fn open(this: Class<'_, Self>, x: f64, y: f64) -> rquickjs::Result<Class<'_, Self>> {
        let mut this_ = this.borrow_mut();
        this_.push_pt(x, y, SpiroCpTy::Open);
        drop(this_);
        Ok(this)
    }

    pub fn end_open(this: Class<'_, Self>, x: f64, y: f64) -> rquickjs::Result<Class<'_, Self>> {
        let mut this_ = this.borrow_mut();
        this_.push_pt(x, y, SpiroCpTy::EndOpen);
        drop(this_);
        Ok(this)
    }
}
