//! Temporary local WMI event subscription. Never opens a process or enables debug privileges.
use crate::{Event, ProcessKey};
use windows::{
    Win32::System::{
        Com::*,
        RemoteDesktop::ProcessIdToSessionId,
        Rpc::{RPC_C_AUTHN_WINNT, RPC_C_AUTHZ_NONE},
        Threading::GetCurrentProcessId,
        Variant::VARIANT,
        Wmi::*,
    },
    core::{BSTR, IUnknown, Interface, w},
};
pub struct Events {
    enumerator: IEnumWbemClassObject,
    _services: IWbemServices,
}
struct Apartment;
impl Drop for Apartment {
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }
    }
}
pub fn listen(
    emit: impl FnMut(Event) -> bool,
    stop: &std::sync::atomic::AtomicBool,
) -> windows::core::Result<()> {
    listen_health(emit, || {}, stop)
}
pub fn listen_health(
    mut emit: impl FnMut(Event) -> bool,
    mut healthy: impl FnMut(),
    stop: &std::sync::atomic::AtomicBool,
) -> windows::core::Result<()> {
    // SAFETY: dedicated worker initializes and tears down its own MTA. COM interfaces
    // stay on this thread; all variant/string outputs are RAII-owned by windows-rs.
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED).ok()?;
        let _apartment = Apartment;
        let locator: IWbemLocator = CoCreateInstance(&WbemLocator, None, CLSCTX_INPROC_SERVER)?;
        let services = locator.ConnectServer(
            &BSTR::from("ROOT\\CIMV2"),
            &BSTR::new(),
            &BSTR::new(),
            &BSTR::new(),
            0,
            &BSTR::new(),
            None,
        )?;
        CoSetProxyBlanket(
            &services,
            RPC_C_AUTHN_WINNT,
            RPC_C_AUTHZ_NONE,
            None,
            RPC_C_AUTHN_LEVEL_PKT_PRIVACY,
            RPC_C_IMP_LEVEL_IMPERSONATE,
            None,
            EOAC_NONE,
        )?;
        let query = BSTR::from(
            "SELECT * FROM __InstanceOperationEvent WITHIN 1 WHERE TargetInstance ISA 'Win32_Process' AND (__CLASS = '__InstanceCreationEvent' OR __CLASS = '__InstanceDeletionEvent')",
        );
        let enumerator = services.ExecNotificationQuery(
            &BSTR::from("WQL"),
            &query,
            WBEM_FLAG_RETURN_IMMEDIATELY | WBEM_FLAG_FORWARD_ONLY,
            None,
        )?;
        let events = Events {
            enumerator,
            _services: services,
        };
        let mut session = 0;
        ProcessIdToSessionId(GetCurrentProcessId(), &mut session)?;
        while !stop.load(std::sync::atomic::Ordering::Relaxed) {
            let mut objects = [None];
            let mut returned = 0;
            let result = events.enumerator.Next(1000, &mut objects, &mut returned);
            result.ok()?;
            if returned == 0 && result.0 == WBEM_S_FALSE.0 {
                return Err(windows::core::Error::from_hresult(
                    windows::Win32::Foundation::E_ABORT,
                ));
            }
            healthy();
            if let Some(object) = objects[0].take() {
                let mut class = VARIANT::default();
                object.Get(w!("__CLASS"), 0, &mut class, None, None)?;
                let class = BSTR::try_from(&class)?.to_string();
                let mut target = VARIANT::default();
                object.Get(w!("TargetInstance"), 0, &mut target, None, None)?;
                let target: IWbemClassObject = IUnknown::try_from(&target)?.cast()?;
                let mut target_session = VARIANT::default();
                target.Get(w!("SessionId"), 0, &mut target_session, None, None)?;
                if u32::try_from(&target_session)? != session {
                    continue;
                }
                let mut value = VARIANT::default();
                target.Get(w!("ProcessId"), 0, &mut value, None, None)?;
                let pid = u32::try_from(&value)?;
                let text = |name: windows::core::PCWSTR| -> Option<String> {
                    let mut value = VARIANT::default();
                    target.Get(name, 0, &mut value, None, None).ok()?;
                    BSTR::try_from(&value).ok().map(|v| v.to_string())
                };
                {
                    let generation = text(w!("CreationDate")).unwrap_or_default();
                    let event = Event {
                        started: class == "__InstanceCreationEvent",
                        key: ProcessKey { pid, generation },
                        path: text(w!("ExecutablePath")),
                    };
                    if !emit(event) {
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}
