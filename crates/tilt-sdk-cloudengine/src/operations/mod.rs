use crate::client::ComputeClient;
use crate::models::Tasks;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tracing::info_span;
use uuid::Uuid;

const MAX_POLL_ATTEMPTS: u32 = 150;

pub struct Operation<'a> {
    task_id: Uuid,
    client: ComputeClient<'a>,
    poll_count: u32,
    last_task: Option<Tasks>,
}

impl<'a> Operation<'a> {
    pub fn new(task_id: Uuid, client: ComputeClient<'a>) -> Self {
        Self {
            task_id,
            client,
            poll_count: 0,
            last_task: None,
        }
    }
}

impl<'a> Future for Operation<'a> {
    type Output = Result<crate::client::Resource, OperationError>;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        let span = info_span!("operation_poll", task_id = %self.task_id);
        let _guard = span.enter();

        self.poll_count += 1;

        if self.poll_count > MAX_POLL_ATTEMPTS {
            return Poll::Ready(Err(OperationError::Timeout));
        }

        let task: crate::models::Tasks =
            match futures::executor::block_on(self.client.get_task(self.task_id)) {
                Ok(task) => task,
                Err(e) => {
                    tracing::debug!(?e, "Failed to poll task, will retry");
                    return Poll::Pending;
                }
            };

        self.last_task = Some(task.clone());

        match task.status {
            crate::models::TaskStatus::Successful => {
                tracing::debug!("Task completed successfully, fetching resource");
                let object_type = task.object_type;
                let object_id = task.object_id;
                match futures::executor::block_on(self.client.get_resource(&object_type, object_id))
                {
                    Ok(resource) => Poll::Ready(Ok(resource)),
                    Err(e) => {
                        let message =
                            format!("failed to fetch resource after task completed: {}", e);
                        tracing::debug!(?e, "Resource fetch failed");
                        Poll::Ready(Err(OperationError::ResourceFetchFailed(message)))
                    }
                }
            }
            crate::models::TaskStatus::Failed => {
                let message = task
                    .error
                    .clone()
                    .unwrap_or_else(|| "Task failed without error message".to_string());
                Poll::Ready(Err(OperationError::TaskFailed(message)))
            }
            crate::models::TaskStatus::Running | crate::models::TaskStatus::New => Poll::Pending,
            _ => Poll::Pending,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum OperationError {
    #[error("operation timed out after max polling attempts")]
    Timeout,
    #[error("operation is still running, call .await or .poll() again")]
    NotComplete,
    #[error("task failed: {0}")]
    TaskFailed(String),
    #[error("failed to fetch resource after task completed: {0}")]
    ResourceFetchFailed(String),
}
