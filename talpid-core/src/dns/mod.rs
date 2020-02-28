use std::{net::IpAddr, path::Path};

#[cfg(target_os = "macos")]
#[path = "macos.rs"]
mod imp;

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod imp;

#[cfg(windows)]
#[path = "windows/mod.rs"]
mod imp;

#[cfg(target_os = "android")]
#[path = "android.rs"]
mod imp;

pub use self::imp::Error as OsError;

/// Errors that can happen in the Linux DNS monitor
#[derive(err_derive::Error, Debug)]
pub enum Error {
    /// Notification error
    #[error(display = "An observer failed")]
    Notification,

    /// Implementation-specific error
    #[error(display = "Internal DNS monitor error")]
    Internal(#[error(source)] OsError),
}

/// Used to notify observers of DNS server changes.
pub enum DnsMonitorUpdate<'a> {
    /// System DNS was set.
    Set {
        /// Name of the network interface.
        interface: &'a str,
        /// List of DNS servers.
        servers: &'a [IpAddr],
    },
    /// System DNS was reset.
    Reset,
}

/// Sets and monitors system DNS settings. Makes sure the desired DNS servers are being used.
pub struct DnsMonitor {
    inner: imp::DnsMonitor,
    observers: Vec<Box< dyn Fn(&DnsMonitorUpdate<'_>) -> Result<(), Error> >>
}

impl DnsMonitor {
    /// Returns a new `DnsMonitor` that can set and monitor the system DNS.
    pub fn new(cache_dir: impl AsRef<Path>) -> Result<Self, Error> {
        Ok(DnsMonitor {
            inner: imp::DnsMonitor::new(cache_dir).map_err(Error::Internal)?,
            observers: Vec::new(),
        })
    }

    /// Register for notifications of DNS server changes.
    pub fn observe<F>(&mut self, callback: F)
        where F: 'static + Fn(&DnsMonitorUpdate<'_>) -> Result<(), Error>
    {
        self.observers.push(Box::new(callback));
    }

    fn notify(&self, state: DnsMonitorUpdate<'_>) -> Result<(), Error> {
        for callback in &self.observers {
            callback(&state)?;
        }
        Ok(())
    }

    /// Set DNS to the given servers. And start monitoring the system for changes.
    pub fn set(&mut self, interface: &str, servers: &[IpAddr]) -> Result<(), Error> {
        log::info!(
            "Setting DNS servers to {}",
            servers
                .iter()
                .map(|ip| ip.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        );
        self.inner.set(interface, servers).map_err(Error::Internal)?;

        self.notify(DnsMonitorUpdate::Set {
            interface,
            servers
        })
    }

    /// Reset system DNS settings to what it was before being set by this instance.
    pub fn reset(&mut self) -> Result<(), Error> {
        log::info!("Resetting DNS");
        self.inner.reset().map_err(Error::Internal)?;
        self.notify(DnsMonitorUpdate::Reset)
    }
}

trait DnsMonitorT: Sized {
    type Error: std::error::Error;

    fn new(cache_dir: impl AsRef<Path>) -> Result<Self, Self::Error>;

    fn set(&mut self, interface: &str, servers: &[IpAddr]) -> Result<(), Self::Error>;

    fn reset(&mut self) -> Result<(), Self::Error>;
}
