#[macro_export]
macro_rules! template {
    ($tmpl:expr, $name:expr, { $($key:ident => $value:expr),+ }) => {
        {
            let data = context! { 
                $( $key => $value,)+
            };
            
            Result::<String, AppError>::Ok($tmpl.get_template($name)
                .map_err(|e| {
                    AppError::OopsError{err: format!("{e}")}
                })?
                .render(data)
                .map_err(|e| {
                    AppError::OopsError{err: format!("{e}")}
                })?)
        }
    };
}