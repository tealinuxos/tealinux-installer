use log::{debug, error, info, trace, warn};
use serde::Serialize;
use specta::Type as SpectaType;
use std::error;

#[derive(Serialize, Clone, Copy, Debug, SpectaType)]
#[serde(rename_all = "camelCase")]
pub enum ErrorHandlerServerity {
    LOW,
    HIGH,
    VeryHigh,
}

#[derive(Serialize, Clone, Debug, SpectaType)]
#[serde(rename_all = "camelCase")]
pub struct ErrorHandlerEmitter {
    pub title: String,
    pub message: String,
    pub serverity: ErrorHandlerServerity,
}

#[derive(Debug)]
pub struct InternalErrorHandlerEmitter {
    pub error_handler_emitter: ErrorHandlerEmitter,
    pub error_ctx: Box<dyn std::error::Error + Send + Sync>,
}

pub trait ErrorHandler {
    fn new(
        title: &str,
        message: &str,
        serverity: ErrorHandlerServerity,
        _internal_error_ctx: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self;
    // fn record_log(self) -> Self;
    fn get_return(self) -> ErrorHandlerEmitter;
    // fn get_error_msg() -> String; /* serde formatted */
}

impl ErrorHandler for InternalErrorHandlerEmitter {
    fn new(
        title: &str,
        message: &str,
        serverity: ErrorHandlerServerity,
        error_ctx: Box<dyn std::error::Error + Send + Sync>,
    ) -> Self {
        let error_handler_emitter = ErrorHandlerEmitter {
            title: title.to_string(),
            message: message.to_string(),
            serverity,
        };

        /* record log */
        error!(
            "{}",
            format!(
                "title: \"{}\", msg: \"{}\", error_string: \"{}\"",
                title.to_string(),
                message.to_string(),
                error_ctx.to_string()
            )
        );

        return Self {
            error_handler_emitter,
            error_ctx,
        };
    }

    fn get_return(self) -> ErrorHandlerEmitter {
        return self.error_handler_emitter;
    }
}
