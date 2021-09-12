//! Subscribe topics

use std::sync::Arc;

use anyhow::Result;
use futures::channel::mpsc;
use rclrust_msg::_core::MessageT;

use crate::{
    internal::worker::{ReceiveWorker, WorkerMessage},
    node::Node,
    qos::QoSProfile,
};

pub mod rcl_wrapper;
pub use rcl_wrapper::RclSubscription;

pub mod invoker;
pub use invoker::{SubscriptionInvoker, SubscriptionInvokerBase};

/// Subscription
pub struct Subscription<T>
where
    T: MessageT,
{
    handle: Arc<RclSubscription>,
    worker: ReceiveWorker<Arc<T::Raw>>,
}

impl<T> Subscription<T>
where
    T: MessageT,
{
    pub(crate) fn new<F>(
        node: &Node,
        topic_name: &str,
        callback: F,
        qos: &QoSProfile,
    ) -> Result<Self>
    where
        T::Raw: 'static,
        F: Fn(Arc<T::Raw>) + Send + 'static,
    {
        let handle = Arc::new(RclSubscription::new::<T>(
            node.clone_handle(),
            topic_name,
            qos,
        )?);

        Ok(Self {
            handle,
            worker: ReceiveWorker::new(callback),
        })
    }

    /// Get the topic name which this subscritpion subscibes to.
    ///
    /// #  Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use rclrust::qos::QoSProfile;
    /// # use rclrust_msg::std_msgs::msg::Int32;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// # let ctx = rclrust::init()?;
    /// # let mut node = ctx.create_node("node")?;
    /// # let callback = |_| ();
    /// let subscription =
    ///     node.create_subscription::<Int32, _>("message", callback, &QoSProfile::default())?;
    /// assert_eq!(&subscription.topic_name().unwrap(), "/message");
    /// # Ok(())
    /// # }
    /// ```
    pub fn topic_name(&self) -> Option<String> {
        self.handle.topic_name()
    }

    /// Check if this publisher is valid or not. Normally, a return value should be `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use rclrust::qos::QoSProfile;
    /// # use rclrust_msg::std_msgs::msg::Int32;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// # let ctx = rclrust::init()?;
    /// # let mut node = ctx.create_node("node")?;
    /// # let callback = |_| ();
    /// let subscription =
    ///     node.create_subscription::<Int32, _>("message", callback, &QoSProfile::default())?;
    /// assert!(subscription.is_valid());
    /// # Ok(())
    /// # }
    /// ```
    pub fn is_valid(&self) -> bool {
        self.handle.is_valid()
    }

    /// Get how many publisher are publishing the topic which this subscription subscribes to.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use rclrust::qos::QoSProfile;
    /// # use rclrust_msg::std_msgs::msg::Int32;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// # let ctx = rclrust::init()?;
    /// # let mut node = ctx.create_node("node")?;
    /// # let callback = |_| ();
    /// let subscription =
    ///     node.create_subscription::<Int32, _>("message", callback, &QoSProfile::default())?;
    /// println!("{}", subscription.publisher_count()?);
    /// # Ok(())
    /// # }
    /// ```
    pub fn publisher_count(&self) -> Result<usize> {
        self.handle.publisher_count()
    }

    /// Get the actual QoS settings, after the defaults have been determined.
    ///
    /// # Examples
    ///
    /// ```
    /// # use anyhow::Result;
    /// # use rclrust::qos::QoSProfile;
    /// # use rclrust_msg::std_msgs::msg::Int32;
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// # let ctx = rclrust::init()?;
    /// # let mut node = ctx.create_node("node")?;
    /// # let callback = |_| ();
    /// let subscription =
    ///     node.create_subscription::<Int32, _>("message", callback, &QoSProfile::default())?;
    /// println!("{:?}", subscription.actual_qos().unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn actual_qos(&self) -> Option<QoSProfile> {
        self.handle.actual_qos()
    }

    pub(crate) fn create_invoker(&self) -> SubscriptionInvoker<T> {
        SubscriptionInvoker::new_from_target(self)
    }

    pub(crate) fn clone_handle(&self) -> Arc<RclSubscription> {
        Arc::clone(&self.handle)
    }

    pub(crate) fn clone_tx(&self) -> mpsc::Sender<WorkerMessage<Arc<T::Raw>>> {
        self.worker.clone_tx()
    }
}

#[cfg(test)]
mod test {
    use rclrust_msg::std_msgs::msg::Int32;

    use super::*;

    fn random_name() -> String {
        use rand::{distributions::Alphanumeric, thread_rng, Rng};

        thread_rng()
            .sample_iter(&Alphanumeric)
            .map(char::from)
            .filter(|c| c.is_alphabetic())
            .take(20)
            .collect()
    }

    #[tokio::test]
    async fn subscription_topic_name() -> Result<()> {
        let ctx = crate::init()?;
        let mut node = ctx.create_node(&random_name())?;
        let subscription =
            node.create_subscription::<Int32, _>("message", |_| (), &QoSProfile::default())?;
        assert_eq!(subscription.topic_name().unwrap(), "/message");
        Ok(())
    }

    #[tokio::test]
    async fn subscription_is_valid() -> Result<()> {
        let ctx = crate::init()?;
        let mut node = ctx.create_node(&random_name())?;
        let subscription =
            node.create_subscription::<Int32, _>("message", |_| (), &QoSProfile::default())?;
        assert!(subscription.is_valid());

        Ok(())
    }

    #[tokio::test]
    async fn subscription_publisher_count() -> Result<()> {
        let ctx = crate::init()?;
        let mut node = ctx.create_node(&random_name())?;
        let subscription =
            node.create_subscription::<Int32, _>("message", |_| (), &QoSProfile::default())?;

        assert_eq!(subscription.publisher_count()?, 0);

        let _pub = node.create_publisher::<Int32>("message", &QoSProfile::default())?;

        assert_eq!(subscription.publisher_count()?, 1);

        Ok(())
    }

    #[tokio::test]
    async fn subscription_actual_qos() -> Result<()> {
        use std::time::Duration;

        use crate::qos::LivelinessPolicy;

        let ctx = crate::init()?;
        let mut node = ctx.create_node(&random_name())?;
        let qos = QoSProfile::sensor_data()
            .deadline(Duration::from_secs(5))
            .lifespan(Duration::from_secs(10))
            .liveliness(LivelinessPolicy::Automatic)
            .liveliness_lease_duration(Duration::from_secs(15));
        let subscription = node.create_subscription::<Int32, _>("message", |_| (), &qos)?;
        assert_eq!(subscription.actual_qos().unwrap(), qos);

        Ok(())
    }
}
