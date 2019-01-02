use actix_web::{
    middleware::{Finished, Middleware, Started},
    HttpRequest, HttpResponse, Result,
};
use chrono::{DateTime, Utc};
use slog::{self, o, trace};
use slog_stdlog;

use crate::state::AppState;

struct StartTime(DateTime<Utc>);

pub struct SlogMiddleware {
    log: slog::Logger,
}

impl SlogMiddleware {
    pub fn new(log: impl Into<Option<slog::Logger>>) -> SlogMiddleware {
        use slog::Drain;

        SlogMiddleware {
            log: log
                .into()
                .unwrap_or_else(|| slog::Logger::root(slog_stdlog::StdLog.fuse(), o!())),
        }
    }

    fn log<S>(&self, req: &HttpRequest<S>, resp: &HttpResponse) {
        if let Some(entry_time) = req.extensions().get::<StartTime>() {
            // Log request / response pair)
            trace!(self.log, "Handled request at {ep}", ep = req.path();
                "duration" => %(Utc::now() - entry_time.0),
                "request_line" => format_args!("{} {}{} {:?}", req.method(), req.path(), req.query_string(), req.version()),
                "response_body" => ?&resp.body(),
                "status_code" => %&resp.status(),
                "remote_ip" => req.connection_info().remote().unwrap_or("-"));
        }
    }
}

// XXX: Can't do this for rustc reasons.. Maybe investigate later
//impl<S> Middleware<S> for SlogMiddleware {
impl Middleware<AppState> for SlogMiddleware {
    fn start(&self, req: &HttpRequest<AppState>) -> Result<Started> {
        trace!(self.log, "Received request at {ep}", ep = req.path();
            "request_line" => format_args!("{} {}{} {:?}", req.method(), req.path(), req.query_string(), req.version()),
            "remote_ip" => req.connection_info().remote().unwrap_or("-"));
        req.extensions_mut().insert(StartTime(Utc::now()));
        Ok(Started::Done)
    }

    fn finish(&self, req: &HttpRequest<AppState>, resp: &HttpResponse) -> Finished {
        self.log(req, resp);
        Finished::Done
    }
}
