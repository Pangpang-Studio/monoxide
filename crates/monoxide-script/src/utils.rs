use rquickjs::{
    class::JsClass,
    function::{FromParam, ParamRequirement},
    Class, Ctx, FromJs, IntoJs, Value,
};

/// A helper type to aid method chaining in classes.
pub struct ChainThis<'js, T> {
    value: Value<'js>,
    _marker: std::marker::PhantomData<&'js T>,
}

impl<'js, T> FromParam<'js> for ChainThis<'js, T> {
    fn param_requirement() -> rquickjs::function::ParamRequirement {
        ParamRequirement::none()
    }

    fn from_param<'a>(
        params: &mut rquickjs::function::ParamsAccessor<'a, 'js>,
    ) -> rquickjs::Result<Self> {
        Ok(Self {
            value: params.this(),
            _marker: std::marker::PhantomData,
        })
    }
}

impl<'js, T> IntoJs<'js> for ChainThis<'js, T> {
    fn into_js(self, _cx: &Ctx<'js>) -> rquickjs::Result<Value<'js>> {
        Ok(self.value)
    }
}

impl<T> std::fmt::Debug for ChainThis<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChainThis")
            .field("T", &std::any::type_name::<T>())
            .field("value", &self.value)
            .finish()
    }
}

impl<'js, T: JsClass<'js>> ChainThis<'js, T> {
    pub fn with<F, R>(&self, cx: &Ctx<'js>, f: F) -> rquickjs::Result<R>
    where
        F: FnOnce(&T) -> R,
    {
        let instance = Class::<T>::from_js(cx, self.value.clone())?;
        let borrowed = instance.try_borrow()?;
        Ok(f(&borrowed))
    }

    pub fn with_mut<F, R>(&self, cx: &Ctx<'js>, f: F) -> rquickjs::Result<R>
    where
        F: FnOnce(&mut T) -> R,
    {
        let instance = Class::<T>::from_js(cx, self.value.clone())?;
        let mut borrowed = instance.try_borrow_mut()?;
        Ok(f(&mut borrowed))
    }
}
