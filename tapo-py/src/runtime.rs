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
                    .map_err(ErrorWrapper)
            })
            .await
            .map_err(anyhow::Error::from)
            .map_err(ErrorWrapper::from)??;

        handler
    }};
}

#[macro_export]
macro_rules! call_handler_method {
    ($self:expr, $method:path) => (call_handler_method!($self, $method,));
    ($self:expr, $method:path, discard_result) => (call_handler_method!($self, $method, discard_result,));
    ($self:expr, $method:path, $($param:expr),*) => {{
        let handler = $self.handler.clone();
        let result = $crate::runtime::tokio()
            .spawn(async move {
                let mut handler = handler.lock().await;

                let result = $method(&mut handler, $($param),*)
                    .await
                    .map_err(ErrorWrapper)?;

                Ok::<_, ErrorWrapper>(result)
            })
            .await
            .map_err(anyhow::Error::from)
            .map_err(ErrorWrapper::from)??;

        Ok::<_, PyErr>(result)
    }};
    ($self:expr, $method:path, discard_result, $($param:expr),*) => {{
        let handler = $self.handler.clone();
        let result = $crate::runtime::tokio()
            .spawn(async move {
                let mut handler = handler.lock().await;

                $method(&mut handler, $($param),*)
                    .await
                    .map_err(ErrorWrapper)?;

                Ok::<_, ErrorWrapper>(())
            })
            .await
            .map_err(anyhow::Error::from)
            .map_err(ErrorWrapper::from)??;

        Ok(result)
    }};
}
