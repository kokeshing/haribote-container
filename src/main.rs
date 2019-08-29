extern crate libc;
extern crate nix;

use std::os::unix::io::AsRawFd;
use std::fs;
use std::fs::File;
use std::path::Path;
use nix::sched::*;
use nix::unistd::*;
use nix::unistd::{execv, fchdir, fork, ForkResult, pivot_root};
use nix::sys::wait::*;
use nix::mount::{mount, MsFlags, umount2, MntFlags};
use std::ffi::CString;


fn main() {
    std::env::set_var("RUST_BACKTRACE", "1"); // デバッグ情報を吐くように

    unshare(CloneFlags::CLONE_NEWPID | CloneFlags::CLONE_NEWNS).expect("Failed to unshare CLONE_NEWNS");
    println!("Unshared CLONE_NEWPID CLONE_NEWNS");

    match fork() {
        Ok(ForkResult::Parent { child, .. }) => {
            match waitpid(child, None).expect("Failed to wait_pid") {
                WaitStatus::Exited(pid, status) => {
                    println!("Stopped container pid:{:?} status:{:?}", pid, status)
                }
                WaitStatus::Signaled(pid, status, _) => {
                    println!("container received signaled: pid={:?}, status={:?}", pid, status)
                }
                _ => println!("container exit!"),
            }
        }
        Ok(ForkResult::Child) => {
            // new rootfsとなるところのバインドマウント
            let path = "/var/lib/haribote_container/haribote/rootfs";
            mount(Some(path), path, Some("rootfs"), MsFlags::MS_BIND, None::<&str>).expect("Failed to bind mount container / onto itself");
            mount(Some(""), "/", None::<&str>, MsFlags::MS_SLAVE | MsFlags::MS_REC, None::<&str>).expect("Failed to setup rootfs for");


            // hostnameの設定
            sethostname("haribote_container").expect("Failed to set the hostname to haribote_container.");
            println!("[SUCCESS] set hostname to haribote_container");


            // procfs や sysfsのマウント
            // fs::create_dir_all("proc").unwrap_or_else(|why| println!("{:?}", why.kind()));
            mount(Some("proc"), "/var/lib/haribote_container/haribote/rootfs/proc", Some("proc"),
                  MsFlags::MS_NODEV | MsFlags::MS_NOEXEC | MsFlags::MS_NOSUID, None::<&str>).expect("mount porcfs failed.");
            mount(Some("sysfs"), "/var/lib/haribote_container/haribote/rootfs/sys", Some("sysfs"),
                  MsFlags::MS_RDONLY, None::<&str>).expect("mount porcfs failed.");


            // pivot root
            let oldroot = File::open("/").expect("Failed to open old root dir");
            let newroot = File::open("/var/lib/haribote_container/haribote/rootfs").expect("Failed to open new root dir");

            fchdir(newroot.as_raw_fd()).expect("Failed to change to new root dir");
            pivot_root(".", ".").expect("Failed to pivot_root()");

            fchdir(oldroot.as_raw_fd()).expect("Failed to enter old root dir");
            mount(Some(""), ".", Some(""), MsFlags::MS_SLAVE | MsFlags::MS_REC, None::<&str>).expect("Failed to make old root rslave");
            umount2(".", MntFlags::MNT_DETACH).expect("Failed to detach old root dir");

            fchdir(newroot.as_raw_fd()).expect("Failed to re-enter to new root dir");
            println!("[SUCCESS] pivot_root");

            // start process
            let bin = CString::new("/bin/bash").unwrap();
            let arg = CString::new("-l").unwrap();

            match execv(&bin, &[bin.clone(), arg]) {
                Ok(_) => println!("[SUCCESS] execv"),
                Err(e) => println!("Error executing execv: {}", e),
            }
        }
        Err(_) => println!("Fork failed"),
    }
}