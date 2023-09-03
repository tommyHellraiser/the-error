
use crate::{SystemErrorCodes, TheErrorType};

impl From<tokio::time::error::Error> for TheErrorType {
    fn from(value: tokio::time::error::Error) -> Self {
        Self {
            error_type: SystemErrorCodes::TimerError,
            error_content: value.to_string()
        }
    }
}

impl From<tokio::io::Error> for TheErrorType {
    fn from(value: tokio::io::Error) -> Self {
        Self {
            error_type: SystemErrorCodes::FileError,
            error_content: value.to_string()
        }
    }
}

impl From<tokio::io::ErrorKind> for TheErrorType {
    fn from(value: tokio::io::ErrorKind) -> Self {
        let error_content = value.to_string();
        let error_type = match value {
            tokio::io::ErrorKind::NotFound => {SystemErrorCodes::NotFound}
            tokio::io::ErrorKind::PermissionDenied => {SystemErrorCodes::PermissionDenied}
            tokio::io::ErrorKind::ConnectionRefused => {SystemErrorCodes::ConnectionRefused}
            tokio::io::ErrorKind::ConnectionReset => {SystemErrorCodes::ConnectionReset}
            tokio::io::ErrorKind::ConnectionAborted => {SystemErrorCodes::ConnectionAborted}
            tokio::io::ErrorKind::NotConnected => {SystemErrorCodes::NotConnected}
            tokio::io::ErrorKind::AddrInUse => {SystemErrorCodes::AddrInUse}
            tokio::io::ErrorKind::AddrNotAvailable => {SystemErrorCodes::AddrNotAvailable}
            tokio::io::ErrorKind::BrokenPipe => {SystemErrorCodes::BrokenPipe}
            tokio::io::ErrorKind::AlreadyExists => {SystemErrorCodes::AlreadyExists}
            tokio::io::ErrorKind::WouldBlock => {SystemErrorCodes::WouldBlock}
            tokio::io::ErrorKind::InvalidInput => {SystemErrorCodes::InvalidInput}
            tokio::io::ErrorKind::InvalidData => {SystemErrorCodes::InvalidData}
            tokio::io::ErrorKind::TimedOut => {SystemErrorCodes::TimedOut}
            tokio::io::ErrorKind::WriteZero => {SystemErrorCodes::WriteZero}
            tokio::io::ErrorKind::Interrupted => {SystemErrorCodes::Interrupted}
            tokio::io::ErrorKind::Unsupported => {SystemErrorCodes::Unsupported}
            tokio::io::ErrorKind::UnexpectedEof => {SystemErrorCodes::UnexpectedEof}
            tokio::io::ErrorKind::OutOfMemory => {SystemErrorCodes::OutOfMemory}
            tokio::io::ErrorKind::Other => {SystemErrorCodes::Other}
            _ => {SystemErrorCodes::UnstableErrorType }
        };

        Self {
            error_type,
            error_content
        }
    }
}

impl<T> From<tokio::sync::SetError<T>> for TheErrorType {
    fn from(value: tokio::sync::SetError<T>) -> Self {
        Self {
            error_type: SystemErrorCodes::InitializeError,
            error_content: value.to_string()
        }
    }
}

impl From<tokio::sync::TryLockError> for TheErrorType {
    fn from(value: tokio::sync::TryLockError) -> Self {
        Self {
            error_type: SystemErrorCodes::MutexError,
            error_content: value.to_string()
        }
    }
}

impl From<tokio::sync::TryAcquireError> for TheErrorType {
    fn from(value: tokio::sync::TryAcquireError) -> Self {
        Self {
            error_type: SystemErrorCodes::SemaphoreError,
            error_content: value.to_string()
        }
    }
}

impl From<tokio::sync::AcquireError> for TheErrorType {
    fn from(value: tokio::sync::AcquireError) -> Self {
        Self {
            error_type: SystemErrorCodes::SemaphoreError,
            error_content: value.to_string()
        }
    }
}

impl From<tokio::sync::broadcast::error::RecvError> for TheErrorType {
    fn from(value: tokio::sync::broadcast::error::RecvError) -> Self {
        let error_content = value.to_string();
        let error_type = match value {
            tokio::sync::broadcast::error::RecvError::Closed => {SystemErrorCodes::ConnectionClosed}
            tokio::sync::broadcast::error::RecvError::Lagged(_) => {SystemErrorCodes::SlowConnection}
        };

        Self {
            error_type,
            error_content
        }
    }
}

impl From<tokio::sync::broadcast::error::TryRecvError> for TheErrorType {
    fn from(value: tokio::sync::broadcast::error::TryRecvError) -> Self {
        Self {
            error_type: SystemErrorCodes::SyncError,
            error_content: value.to_string()
        }
    }
}

impl<T> From<tokio::sync::broadcast::error::SendError<T>> for TheErrorType {
    fn from(value: tokio::sync::broadcast::error::SendError<T>) -> Self {
        Self {
            error_type: SystemErrorCodes::SyncError,
            error_content: value.to_string()
        }
    }
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for TheErrorType {
    fn from(value: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Self {
            error_type: SystemErrorCodes::SyncError,
            error_content: value.to_string()
        }
    }
}

impl From<tokio::sync::mpsc::error::TryRecvError> for TheErrorType {
    fn from(value: tokio::sync::mpsc::error::TryRecvError) -> Self {
        Self {
            error_type: SystemErrorCodes::SyncError,
            error_content: value.to_string()
        }
    }
}

impl<T> From<tokio::sync::mpsc::error::TrySendError<T>> for TheErrorType {
    fn from(value: tokio::sync::mpsc::error::TrySendError<T>) -> Self {
        Self {
            error_type: SystemErrorCodes::SyncError,
            error_content: value.to_string()
        }
    }
}

impl<T> From<tokio::sync::mpsc::error::SendTimeoutError<T>> for TheErrorType {
    fn from(value: tokio::sync::mpsc::error::SendTimeoutError<T>) -> Self {
        Self {
            error_type: SystemErrorCodes::SyncError,
            error_content: value.to_string()
        }
    }
}

impl From<tokio::sync::oneshot::error::RecvError> for TheErrorType {
    fn from(value: tokio::sync::oneshot::error::RecvError) -> Self {
        Self {
            error_type: SystemErrorCodes::SyncError,
            error_content: value.to_string()
        }
    }
}

impl From<tokio::sync::oneshot::error::TryRecvError> for TheErrorType {
    fn from(value: tokio::sync::oneshot::error::TryRecvError) -> Self {
        Self {
            error_type: SystemErrorCodes::SyncError,
            error_content: value.to_string()
        }
    }
}

impl<T> From<tokio::sync::watch::error::SendError<T>> for TheErrorType {
    fn from(value: tokio::sync::watch::error::SendError<T>) -> Self {
        Self {
            error_type: SystemErrorCodes::SyncError,
            error_content: value.to_string()
        }
    }
}

impl From<tokio::sync::watch::error::RecvError> for TheErrorType {
    fn from(value: tokio::sync::watch::error::RecvError) -> Self {
        Self {
            error_type: SystemErrorCodes::ConnectionClosed,
            error_content: value.to_string()
        }
    }
}

impl From<tokio::task::JoinError> for TheErrorType {
    fn from(value: tokio::task::JoinError) -> Self {
        Self {
            error_type: SystemErrorCodes::TaskError,
            error_content: value.to_string()
        }
    }
}

impl From<tokio::net::tcp::ReuniteError> for TheErrorType {
    fn from(value: tokio::net::tcp::ReuniteError) -> Self {
        Self {
            error_type: SystemErrorCodes::SocketError,
            error_content: value.to_string()
        }
    }
}

impl From<tokio::runtime::TryCurrentError> for TheErrorType {
    fn from(value: tokio::runtime::TryCurrentError) -> Self {
        Self {
            error_type: SystemErrorCodes::ThreadError,
            error_content: value.to_string()
        }
    }
}
