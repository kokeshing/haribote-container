extern crate libc;
extern crate nix;

use nix::mount::{mount, umount2, MntFlags, MsFlags};
use nix::sched::*;
use nix::sys::wait::*;
use nix::unistd::*;
use nix::unistd::{execv, fchdir, fork, pivot_root, ForkResult};
use std::ffi::CString;
use std::fs::File;
use std::os::unix::io::AsRawFd;

static CONTAINER_ROOT: &str =
    "/var/lib/haribote_container/haribote/rootfs";
static CONTAINER_PROC: &str =
    "/var/lib/haribote_container/haribote/rootfs/proc";
static CONTAINER_SYS: &str =
    "/var/lib/haribote_container/haribote/rootfs/sys";

fn main() {
    // 名前空間の分離
    unshare(
        CloneFlags::CLONE_NEWPID
            | CloneFlags::CLONE_NEWNS
            | CloneFlags::CLONE_NEWUTS,
    )
    .expect("Failed to unshare CLONE_NEWNS");
    println!("Unshared CLONE_NEWPID CLONE_NEWNS");

    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            match waitpid(child, None).expect("Failed to wait_pid")
            {
                WaitStatus::Exited(pid, status) => println!(
                    "Stopped container\npid:{:?} status:{:?}",
                    pid, status
                ),
                WaitStatus::Signaled(pid, status, _) => println!(
                    "Received signaled\npid:{:?}, status:{:?}",
                    pid, status
                ),
                _ => println!("Exit container"),
            }
        }
        Ok(ForkResult::Child) => {
            // マウント操作の準備
            mount(
                Some(CONTAINER_ROOT),
                CONTAINER_ROOT,
                Some("rootfs"),
                MsFlags::MS_BIND,
                None::<&str>,
            )
            .expect("Failed to bind mount container / onto itself");
            mount(
                Some(""),
                "/",
                None::<&str>,
                MsFlags::MS_SLAVE | MsFlags::MS_REC,
                None::<&str>,
            )
            .expect("Failed to remount \"/\" MS_REC | MS_SLAVE");

            // hostnameの設定
            sethostname("haribote_container").expect(
                "Failed to set the hostname to haribote_container",
            );
            println!(
                "[SUCCESS] set hostname to haribote_container"
            );

            // procfsやsysfsのマウント
            mount(
                Some("proc"),
                CONTAINER_PROC,
                Some("proc"),
                MsFlags::MS_NODEV
                    | MsFlags::MS_NOEXEC
                    | MsFlags::MS_NOSUID,
                None::<&str>,
            )
            .expect("Failed to mount porcfs");
            mount(
                Some("sysfs"),
                CONTAINER_SYS,
                Some("sysfs"),
                MsFlags::MS_RDONLY,
                None::<&str>,
            )
            .expect("Failed to mount sysfs");
            println!("[SUCCESS] mount procfs and sysfs");

            // pivot root
            {
                let oldroot = File::open("/")
                    .expect("Failed to open old root dir");
                let newroot = File::open(CONTAINER_ROOT)
                    .expect("Failed to open new root dir");

                fchdir(newroot.as_raw_fd())
                    .expect("Failed to change to new root dir");
                pivot_root(".", ".")
                    .expect("Failed to pivot_root()");

                fchdir(oldroot.as_raw_fd())
                    .expect("Failed to enter old root dir");
                mount(
                    Some(""),
                    ".",
                    Some(""),
                    MsFlags::MS_SLAVE | MsFlags::MS_REC,
                    None::<&str>,
                )
                .expect("Failed to make old root rslave");
                umount2(".", MntFlags::MNT_DETACH)
                    .expect("Failed to detach old root dir");

                fchdir(newroot.as_raw_fd())
                    .expect("Failed to re-enter to new root dir");
                println!("[SUCCESS] pivot_root");
            }

            // start process
            let bin = CString::new("/bin/bash").unwrap();
            let arg = CString::new("-l").unwrap();

            match execv(&bin, &[bin.clone(), arg]) {
                Ok(_) => println!("[SUCCESS] execv"),
                Err(e) => println!("Failed to execv: {}", e),
            }
        }
        Err(_) => println!("Failed to fork"),
    }
}
