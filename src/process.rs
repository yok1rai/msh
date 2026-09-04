use nix::{
    sys::wait::{WaitPidFlag, WaitStatus, waitpid},
    unistd::{ForkResult, Pid, execvp, fork},
};
use std::{
    collections::HashMap,
    error::Error,
    ffi::CString,
    sync::atomic::{AtomicBool, Ordering},
};

pub static CHILD_EXITED: AtomicBool = AtomicBool::new(false);

pub extern "C" fn sigchld_handler(_: i32) {
    CHILD_EXITED.store(true, Ordering::Relaxed);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JobState {
    Running,
    Stopped,
    Done,
}

#[derive(Debug)]
pub struct Job {
    pub id: usize,
    pub pid: Pid,
    pub command: String,
    pub state: JobState,
}

#[derive(Debug)]
pub struct JobTable {
    jobs: HashMap<usize, Job>,
    next_id: usize,
}

impl JobTable {
    pub fn new() -> Self {
        Self {
            jobs: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add(&mut self, pid: Pid, command: String) -> usize {
        let id = self.next_id;

        self.jobs.insert(
            id,
            Job {
                id,
                pid,
                command,
                state: JobState::Running,
            },
        );

        self.next_id += 1;

        id
    }

    pub fn remove(&mut self, id: usize) -> Option<Job> {
        self.jobs.remove(&id)
    }

    pub fn get(&self, id: usize) -> Option<&Job> {
        self.jobs.get(&id)
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut Job> {
        self.jobs.get_mut(&id)
    }

    pub fn get_by_pid(&self, pid: Pid) -> Option<&Job> {
        self.jobs.values().find(|job| job.pid == pid)
    }

    pub fn get_id_by_pid(&self, pid: Pid) -> Option<usize> {
        self.jobs
            .values()
            .find(|job| job.pid == pid)
            .map(|job| job.id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Job> {
        self.jobs.values()
    }
}

pub fn reap_children(jobs: &mut JobTable) {
    loop {
        match waitpid(Pid::from_raw(-1), Some(WaitPidFlag::WNOHANG)) {
            Ok(WaitStatus::Exited(pid, status)) => {
                if let Some(id) = jobs.get_id_by_pid(pid) {
                    if let Some(job) = jobs.remove(id) {
                        println!("\n[{}] Done {}", job.id, job.command);
                    }

                    println!("[PID {pid}] exited with status {status}");
                }
            }

            Ok(WaitStatus::Signaled(pid, signal, _)) => {
                if let Some(id) = jobs.get_id_by_pid(pid) {
                    if let Some(job) = jobs.remove(id) {
                        println!("\n[{}] Terminated by {} {}", job.id, signal, job.command);
                    }
                }
            }

            Ok(WaitStatus::StillAlive) => {
                break;
            }

            Ok(_) => {}

            Err(nix::errno::Errno::ECHILD) => {
                break;
            }

            Err(e) => {
                eprintln!("waitpid: {e}");
                break;
            }
        }
    }
}

pub fn run(mut command: Vec<String>, jobs: &mut JobTable) -> Result<(), Box<dyn Error>> {
    let bg = command.last().map(|arg| arg == "&").unwrap_or(false);

    if bg {
        command.pop();
    }

    if command.is_empty() {
        return Ok(());
    }

    let command_string = command.join(" ");

    let args: Vec<CString> = command
        .iter()
        .map(|arg| CString::new(arg.as_str()))
        .collect::<Result<_, _>>()?;

    match unsafe { fork()? } {
        ForkResult::Child => {
            execvp(&args[0], &args)?;

            unreachable!();
        }

        ForkResult::Parent { child } => {
            if bg {
                let id = jobs.add(child, command_string);

                println!("[{id}] {child}");
            } else {
                waitpid(child, None)?;
            }
        }
    }

    Ok(())
}

