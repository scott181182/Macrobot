use std::time::{ Duration, Instant };
use actix::{ Actor, StreamHandler, ActorContext, AsyncContext };
use actix_web::{ web, get, HttpRequest, HttpResponse, Error };
use actix_web_actors::ws;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before the lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[get("/ws/")]
async fn init_ws(
    r: HttpRequest,
    stream: web::Payload
) -> Result<HttpResponse, Error> {
    ws::start(MacrobotWS::new(), &r, stream)
}

/// Handles the string input for execution on server host
pub struct MacrobotWS {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant
}

impl Actor for MacrobotWS {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }

    // fn stopped() { ... }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MacrobotWS {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context
    ) {
        match msg {
            Ok(ws::Message::Ping(_)) => {
                self.hb = Instant::now();
            },
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now()
            },
            // Right now any message recieved is just returned
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            },
            _ => ctx.stop()
        }
    }
}

impl MacrobotWS {
    fn new() -> Self {
        Self { hb: Instant::now() }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                warn!("Websocket client timed out, disconnecting");
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}