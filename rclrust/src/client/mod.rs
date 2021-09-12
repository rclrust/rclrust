use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use anyhow::Result;
use futures::channel::{mpsc, oneshot};
use rclrust_msg::_core::{FFIToRust, ServiceResponseRaw, ServiceT};

use crate::{
    internal::worker::{ReceiveWorker, WorkerMessage},
    node::Node,
    qos::QoSProfile,
};

pub mod rcl_wrapper;
pub use rcl_wrapper::RclClient;

pub mod invoker;
pub use invoker::{ClientInvoker, ClientInvokerBase};

type ChannelMessage<Srv> = (rcl_sys::rmw_request_id_t, ServiceResponseRaw<Srv>);

pub struct Client<Srv>
where
    Srv: ServiceT + 'static,
{
    handle: Arc<RclClient>,
    worker: ReceiveWorker<ChannelMessage<Srv>>,
    pendings: Arc<Mutex<HashMap<i64, oneshot::Sender<Srv::Response>>>>,
}

impl<Srv> Client<Srv>
where
    Srv: ServiceT,
{
    pub(crate) fn new(node: &Node, service_name: &str, qos: &QoSProfile) -> Result<Self> {
        let handle = Arc::new(RclClient::new::<Srv>(
            node.clone_handle(),
            service_name,
            qos,
        )?);

        let pendings = Arc::new(Mutex::new(
            HashMap::<i64, oneshot::Sender<Srv::Response>>::new(),
        ));

        let callback = {
            let pendings = Arc::clone(&pendings);

            move |(req_header, res): (rcl_sys::rmw_request_id_t, ServiceResponseRaw<Srv>)| {
                pendings
                    .lock()
                    .unwrap()
                    .remove(&req_header.sequence_number)
                    .unwrap_or_else(|| panic!("fail to find key in Client::process_requests"))
                    .send(unsafe { res.to_rust() })
                    .unwrap_or_else(|_| {
                        panic!("fail to send response via channel in Client::process_requests")
                    });
            }
        };

        Ok(Self {
            handle,
            worker: ReceiveWorker::new(callback),
            pendings,
        })
    }

    pub async fn send_request(&mut self, request: &Srv::Request) -> Result<Srv::Response> {
        let id = self.handle.send_request::<Srv>(request)?;
        let (tx, rx) = oneshot::channel::<Srv::Response>();
        self.pendings.lock().unwrap().insert(id, tx);

        Ok(rx.await?)
    }

    /// Get the service name.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use rclrust::qos::QoSProfile;
    /// # use rclrust_msgs::std_srvs::srv::Empty;
    /// #
    /// # #[tokio::main]
    /// 
    /// ```
    pub fn service_name(&self) -> Option<String> {
        self.handle.service_name()
    }

    pub fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }

    pub fn service_is_available(&self) -> Result<bool> {
        self.handle.service_is_available()
    }

    pub fn wait_service(&self) -> Result<()> {
        while !self.service_is_available()? {
            thread::sleep(Duration::from_millis(1));
        }
        Ok(())
    }

    pub(crate) fn create_invoker(&self) -> ClientInvoker<Srv> {
        ClientInvoker::new_from_target(self)
    }

    pub(crate) fn clone_handle(&self) -> Arc<RclClient> {
        Arc::clone(&self.handle)
    }

    pub(crate) fn clone_tx(&self) -> mpsc::Sender<WorkerMessage<ChannelMessage<Srv>>> {
        self.worker.clone_tx()
    }
}
