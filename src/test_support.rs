use std::ffi::OsString;
use std::sync::{Mutex, MutexGuard};

static ENV_LOCK: Mutex<()> = Mutex::new(());

/// Serializes process-wide environment changes and restores the previous value.
pub(crate) struct EnvVarGuard {
    key: &'static str,
    previous: Option<OsString>,
    _lock: MutexGuard<'static, ()>,
}

impl EnvVarGuard {
    pub(crate) fn set(key: &'static str, value: impl AsRef<std::ffi::OsStr>) -> Self {
        let lock = ENV_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let previous = std::env::var_os(key);
        // SAFETY: all tests that mutate process environment use ENV_LOCK.
        unsafe { std::env::set_var(key, value) };
        Self {
            key,
            previous,
            _lock: lock,
        }
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        // SAFETY: ENV_LOCK remains held until this method returns.
        unsafe {
            match &self.previous {
                Some(value) => std::env::set_var(self.key, value),
                None => std::env::remove_var(self.key),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EnvVarGuard;

    #[test]
    fn restores_previous_value_during_unwind() {
        const KEY: &str = "SYSLENZ_TEST_ENV_GUARD";
        // SAFETY: this key is private to this test.
        unsafe { std::env::set_var(KEY, "before") };

        let result = std::panic::catch_unwind(|| {
            let _guard = EnvVarGuard::set(KEY, "during");
            assert_eq!(std::env::var(KEY).as_deref(), Ok("during"));
            panic!("exercise guard cleanup");
        });

        assert!(result.is_err());
        assert_eq!(std::env::var(KEY).as_deref(), Ok("before"));
        // SAFETY: the guard has released the process-environment lock.
        unsafe { std::env::remove_var(KEY) };
    }
}
