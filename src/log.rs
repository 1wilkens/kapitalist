use actix_web::{
    middleware::{Finished, Middleware, Started},
    HttpRequest, HttpResponse, Result,
};
use chrono::{DateTime, Utc};
use slog::{self, o, trace};
use slog_stdlog;

use crate::state::AppState;

struct StartTime(DateTime<Utc>);

pub struct SlogLogger {
    log: slog::Logger,
}

impl SlogLogger {
    pub fn new(log: impl Into<Option<slog::Logger>>) -> SlogLogger {
        use slog::Drain;

        SlogLogger {
            log: log
                .into()
                .unwrap_or(slog::Logger::root(slog_stdlog::StdLog.fuse(), o!())),
        }
    }

    fn log<S>(&self, req: &HttpRequest<S>, _resp: &HttpResponse) {
        if let Some(entry_time) = req.extensions().get::<StartTime>() {
            // Log request / response pair)
            trace!(self.log, "Handled request";
                "time" => %(Utc::now() - entry_time.0),
                "request_line" => format_args!("{} {} {:?}", req.method(), req.path(), req.version()),
                "remote_ip" => req.connection_info().remote().unwrap_or("-"));
        }
    }
}

// XXX: Can't do this for rustc reasons.. Maybe investigate later
//impl<S> Middleware<S> for SlogLogger {
impl Middleware<AppState> for SlogLogger {
    fn start(&self, req: &HttpRequest<AppState>) -> Result<Started> {
        req.extensions_mut().insert(StartTime(Utc::now()));
        Ok(Started::Done)
    }

    fn finish(&self, req: &HttpRequest<AppState>, resp: &HttpResponse) -> Finished {
        self.log(req, resp);
        Finished::Done
    }
}
