use std::{fmt, sync::Arc};

use anyhow::Result;
use futures::channel::mpsc;
use rclrust_msg::_core::ServiceT;

use super::{ChannelMessage, RclService, Service};
use crate::{error::RclRustError, internal::worker::WorkerMessage, rclrust_debug, Logger};

pub trait ServiceInvokerBase: fmt::Debug {
    fn handle(&self) -> &RclService;
    fn invoke(&mut self) -> Result<()>;
}

pub struct ServiceInvoker<Srv>
where
    Srv: ServiceT,
{
    handle: Arc<RclService>,
    tx: Option<mpsc::Sender<WorkerMessage<ChannelMessage<Srv>>>>,
}

impl<Srv> ServiceInvoker<Srv>
where
    Srv: ServiceT,
{
    pub fn new_from_target(target: &Service<Srv>) -> Self {
        Self {
            handle: target.clone_handle(),
            tx: Some(target.clone_tx()),
        }
    }

    fn stop(&mut self) {
        self.tx.take();
    }
}

impl<Srv> fmt::Debug for ServiceInvoker<Srv>
where
    Srv: ServiceT,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ServiceInvoker {{{:?}}}", self.handle)
    }
}

impl<Srv> ServiceInvokerBase for ServiceInvoker<Srv>
where
    Srv: ServiceT,
{
    fn handle(&self) -> &RclService {
        &self.handle
    }

    fn invoke(&mut self) -> Result<()> {
        if let Some(ref mut tx) = self.tx {
            let req = match self.handle.take_request::<Srv>() {
                Ok(v) => v,
                Err(e) => {
                    return if let Some(RclRustError::RclServiceTakeFailed(_)) =
                        e.downcast_ref::<RclRustError>()
                    {
                        rclrust_debug!(
                            Logger::new("rclrust"),
                            "`rcl_wait()` indicate that request is ready, however which incorrect. I know this happens when I use Cyclone DDS."
                        );
                        Ok(())
                    } else {
                        Err(e)
                    };
                }
            };

            match tx.try_send(WorkerMessage::Message(req)) {
                Ok(_) => (),
                Err(e) if e.is_disconnected() => self.stop(),
                Err(_) => {
                    return Err(RclRustError::MessageQueueIsFull {
                        type_: "Service",
                        name: self.handle.service_name().expect("service should be valid"),
                    }
                    .into())
                }
            }
        }

        Ok(())
    }
}
