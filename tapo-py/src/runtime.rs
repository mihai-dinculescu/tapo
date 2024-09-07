pub fn tokio() -> &'static tokio::runtime::Runtime {
    use std::sync::OnceLock;
    use tokio::runtime::Runtime;
    static RT: std::sync::OnceLock<Runtime> = OnceLock::new();
    RT.get_or_init(|| Runtime::new().expect("Failed to create tokio runtime"))
}

#[macro_export]
macro_rules! call_handler_constructor {
    ($self:ident, $constructor:path, $ip_address:expr) => {{
        let client = $self.client.clone();
        let handler = $crate::runtime::tokio()
            .spawn(async move {
                $constructor(client, $ip_address)
                    .await
                    .map_err($crate::errors::ErrorWrapper)
            })
            .await
            .map_err(anyhow::Error::from)
            .map_err($crate::errors::ErrorWrapper::from)??;

        handler
    }};
}

#[macro_export]
macro_rules! call_handler_method {
    ($handler:expr, $method:path) => (call_handler_method!($handler, $method,));
    ($handler:expr, $method:path, discard_result) => (call_handler_method!($handler, $method, discard_result,));
    ($handler:expr, $method:path, $($param:expr),*) => {{
        let result = $crate::runtime::tokio()
            .spawn(async move {
                let result = $method($handler, $($param),*)
                    .await
                    .map_err($crate::errors::ErrorWrapper)?;

                Ok::<_, $crate::errors::ErrorWrapper>(result)
            })
            .await
            .map_err(anyhow::Error::from)
            .map_err($crate::errors::ErrorWrapper::from)??;

        Ok::<_, PyErr>(result)
    }};
    ($handler:expr, $method:path, discard_result, $($param:expr),*) => {{
        let result = $crate::runtime::tokio()
            .spawn(async move {
                $method($handler, $($param),*)
                    .await
                    .map_err($crate::errors::ErrorWrapper)?;

                Ok::<_, $crate::errors::ErrorWrapper>(())
            })
            .await
            .map_err(anyhow::Error::from)
            .map_err($crate::errors::ErrorWrapper::from)??;

        Ok(result)
    }};
}
