use talpid_types::ErrorExt;
use std::{
    fs,
    io::{self, BufRead, BufReader, Write},
    num,
    path::PathBuf,
};

const NETCLS_PATH: &str = "/sys/fs/cgroup/net_cls/";
pub const NETCLS_CLASSID: u32 = 0x4d9f41;
const CGROUP_NAME: &str = "mullvad-exclusions";

#[derive(err_derive::Error, Debug)]
#[error(no_from)]
pub enum Error {
    /// Unable to create cgroup.
    #[error(display = "Unable to create cgroup for excluded processes")]
    CreateCGroup(#[error(source)] io::Error),

    /// Unable to set class ID for cgroup.
    #[error(display = "Unable to set cgroup class ID")]
    SetCGroupClassId(#[error(source)] io::Error),

    /// Unable to add PID to cgroup.procs.
    #[error(display = "Unable to add PID to cgroup.procs")]
    AddCGroupPid(#[error(source)] io::Error),

    /// Unable to read cgroup.procs.
    #[error(display = "Unable to obtain PIDs from cgroup.procs")]
    ListCGroupPids(#[error(source)] io::Error),
}

fn create_cgroup() -> Result<(), Error> {
    let mut exclusions_dir = PathBuf::from(NETCLS_PATH);
    exclusions_dir.push(CGROUP_NAME);

    if !exclusions_dir.exists() {
        fs::create_dir(exclusions_dir.clone()).map_err(Error::CreateCGroup)?;
    }

    let mut classid_file = PathBuf::from(exclusions_dir);
    classid_file.push("net_cls.classid");
    fs::write(classid_file, NETCLS_CLASSID.to_string().as_bytes())
        .map_err(Error::SetCGroupClassId)?;

    Ok(())
}

fn add_pid(pid: i32) -> Result<(), Error> {
    let mut exclusions_file = PathBuf::from(NETCLS_PATH);
    exclusions_file.push(CGROUP_NAME);
    exclusions_file.push("cgroup.procs");

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(exclusions_file)
        .map_err(Error::AddCGroupPid)?;

    file.write_all(pid.to_string().as_bytes())
        .map_err(Error::AddCGroupPid)?;

    Ok(())
}

// TODO: remove pids

fn list_pids(pid: i32) -> Result<Vec<i32>, Error> {
    // TODO: manage child PIDs somehow?

    let mut exclusions_file = PathBuf::from(NETCLS_PATH);
    exclusions_file.push(CGROUP_NAME);
    exclusions_file.push("cgroup.procs");

    let file = fs::File::open(exclusions_file)
        .map_err(Error::ListCGroupPids)?;

    let result: Result<Vec<i32>, io::Error> = BufReader::new(file)
        .lines()
        .map(|line| {
            line.and_then(|v|
                v.parse().map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
            )
        })
        .collect();
    result.map_err(Error::ListCGroupPids)
}
