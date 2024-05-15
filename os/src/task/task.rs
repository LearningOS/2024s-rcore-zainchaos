//! Types related to task management

use crate::config::MAX_SYSCALL_NUM;
use super::TaskContext;

/// The information of a task
#[derive(Copy, Clone)]
pub struct TaskInfo {
    /// The number of times each syscall is called
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// The time when the task was first sheduled
    pub time: usize,
}

impl TaskInfo {
    /// Create a new `TaskInfo`
    pub fn new() -> Self {
        TaskInfo {
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: 0,
        }
    }
}

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The task information
    pub task_info: TaskInfo,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
