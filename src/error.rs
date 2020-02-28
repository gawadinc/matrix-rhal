/// Error handling.
use std::{error::Error as StdError, fmt};

#[derive(Debug)]
pub enum Error {
    /// Some unspecified error.
    Any(Box<dyn StdError + Send + Sync + 'static>),
    /// MATRIX Device could not be identified.
    UnknownDevice,
    /// Could not initialize the MATRIX Bus.
    UnableToStartBus,
    /// MATRIX Kernel modules have not been installed.
    KernelModulesNotInstalled,
    /// The mutex for the Bus could not be grabbed. (currently unused)
    PoisonedBusMutex,
}

impl<'a> fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::UnknownDevice => write!(f, "Unable to identify MATRIX device."),
            Error::UnableToStartBus => write!(f, "Could not start the MATRIX bus."),
            Error::PoisonedBusMutex => write!(f, "Mutex for MATRIX bus is unreachable."),
            Error::KernelModulesNotInstalled => {
                write!(f, "The MATRIX Kernel Modules have not been installed. In order to work, this library requires them!")
            }
            _ => write!(f, "TODO: ADD ERROR DESCRIPTION!"),
        }
    }
}

use nix;
impl From<nix::Error> for Error {
    fn from(error: nix::Error) -> Self {
        // TODO: add match statement for different nix errors
        // match error {}

        Error::UnableToStartBus
    }
}

use std::sync::MutexGuard;
use std::sync::PoisonError;
impl From<PoisonError<MutexGuard<'_, ()>>> for Error {
    fn from(_: PoisonError<MutexGuard<()>>) -> Self {
        Error::PoisonedBusMutex
    }
}
