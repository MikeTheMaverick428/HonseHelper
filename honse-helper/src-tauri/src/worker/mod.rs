use std::collections::HashMap;
use std::process::{Child, ChildStdin};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Mutex, MutexGuard};

/// Raw msgpack frame bytes received from the worker.
pub type RawWorkerFrame = Vec<u8>;

/// Represents a currently running worker process
pub struct RunningWorker {
    pub child: Child,
    pub stdin: ChildStdin,
    pub pid: u32,
}

/// Global worker state - manages the lifecycle of the sidecar process
#[derive(Default)]
pub struct WorkerState {
    pub inner: Mutex<Option<RunningWorker>>,
    pending: Mutex<HashMap<u64, Sender<RawWorkerFrame>>>,
    next_request_id: AtomicU64,
}

impl WorkerState {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(None),
            pending: Mutex::new(HashMap::new()),
            next_request_id: AtomicU64::new(1),
        }
    }

    fn lock_inner(&self) -> Result<MutexGuard<'_, Option<RunningWorker>>, String> {
        self.inner
            .lock()
            .map_err(|_| "state lock poisoned".to_string())
    }

    pub fn store_running(&self, worker: RunningWorker) -> Result<(), String> {
        let mut guard = self.lock_inner()?;
        if guard.is_some() {
            return Err("worker already running".to_string());
        }

        *guard = Some(worker);
        Ok(())
    }

    pub fn take_running(&self) -> Result<Option<RunningWorker>, String> {
        let mut guard = self.lock_inner()?;
        Ok(guard.take())
    }

    pub fn with_running_worker<T>(
        &self,
        action: impl FnOnce(&mut RunningWorker) -> Result<T, String>,
    ) -> Result<T, String> {
        let mut guard = self.lock_inner()?;
        let running = guard
            .as_mut()
            .ok_or_else(|| "worker not running".to_string())?;
        action(running)
    }

    pub fn get_pid_if_running(&self) -> Option<u32> {
        let mut guard = self.inner.lock().ok()?;
        let running = guard.as_mut()?;
        match running.child.try_wait() {
            Ok(Some(_)) => {
                let _ = guard.take();
                None
            }
            Ok(None) => Some(running.pid),
            Err(_) => None,
        }
    }

    pub fn is_running(&self) -> Result<bool, String> {
        let mut guard = self.lock_inner()?;
        match guard.as_mut() {
            Some(running) => match running.child.try_wait() {
                Ok(Some(_)) => {
                    guard.take();
                    Ok(false)
                }
                Ok(None) => Ok(true),
                Err(err) => Err(format!("failed to query worker status: {err}")),
            },
            None => Ok(false),
        }
    }

    pub fn clear_if_pid_matches(&self, pid: u32) {
        if let Ok(mut guard) = self.inner.lock() {
            if guard.as_ref().map(|worker| worker.pid) == Some(pid) {
                guard.take();
            }
        }
    }

    pub fn next_request_id(&self) -> u64 {
        self.next_request_id.fetch_add(1, Ordering::Relaxed)
    }

    pub fn register_pending(&self, request_id: u64) -> Result<Receiver<RawWorkerFrame>, String> {
        let (tx, rx) = mpsc::channel();
        let mut pending = self
            .pending
            .lock()
            .map_err(|_| "pending state lock poisoned".to_string())?;
        pending.insert(request_id, tx);
        Ok(rx)
    }

    pub fn resolve_pending_raw(&self, request_id: u64, frame: RawWorkerFrame) -> bool {
        if let Ok(mut pending) = self.pending.lock() {
            if let Some(tx) = pending.remove(&request_id) {
                let _ = tx.send(frame);
                return true;
            }
        }

        false
    }

    pub fn clear_pending(&self, request_id: u64) {
        if let Ok(mut pending) = self.pending.lock() {
            pending.remove(&request_id);
        }
    }
}
