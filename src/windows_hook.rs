
use windows::{core::IntoParam, Win32::{Foundation::*, UI::WindowsAndMessaging::*}};
use std::sync::{Mutex, Arc};

pub fn set(idhook: WINDOWS_HOOK_ID, lpfn: HOOKPROC) -> HHOOK {
  let hook = Arc::new(Mutex::new(None));
  let cloned_hook = hook.clone();
  std::thread::spawn(move || unsafe {
    *cloned_hook.lock().unwrap() = Some(SetWindowsHookExW(idhook, lpfn, None, 0));
    let mut msg = MSG {
      hwnd: HWND(0),
      message: 0,
      wParam: WPARAM(0),
      lParam: LPARAM(0),
      time: 0,
      pt: POINT { x: 0, y: 0 },
    };
    loop {
      let pm = GetMessageW(&mut msg, HWND(0), 0, 0);
      if pm.0 == 0 { break }
      TranslateMessage(&msg);
      DispatchMessageW(&msg);
    }
  });
  loop { if let Some(hook) = *hook.lock().unwrap() { return hook }}
}

pub fn unhook<'a, Param0: IntoParam<'a, HHOOK>>(hook: Param0) -> i32 {
  unsafe { UnhookWindowsHookEx(hook).0 }
}