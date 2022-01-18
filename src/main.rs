use windows::{core::IntoParam, Win32::{Foundation, UI::{WindowsAndMessaging as Messaging, Input::KeyboardAndMouse}}};
use std::{io::Write, sync::{Mutex, Arc}};

fn main() {
  // Print prompt and flush.
  print!(">");
  std::io::stdout().flush().unwrap(); 

  // Create a hook.
  let hook = start_hook(Messaging::WH_KEYBOARD_LL, Some(ll_keyboard_proc));

  // Receive input.
  let mut guess = String::new();
  std::io::stdin().read_line(&mut guess).expect("Error: Failed on 'read_line()'.");

  // Unhook.
  unhook(hook);
}

fn start_hook(idhook: Messaging::WINDOWS_HOOK_ID, lpfn: Messaging::HOOKPROC) -> Messaging::HHOOK {
  let hook = Arc::new(Mutex::<Option<Messaging::HHOOK>>::new(None));
  let cloned_hook = hook.clone();
  std::thread::spawn(move || unsafe {
    *cloned_hook.lock().unwrap() = Some(Messaging::SetWindowsHookExW(idhook, lpfn, None, 0));
    let mut msg = Messaging::MSG {
      hwnd: Foundation::HWND(0),
      message: 0,
      wParam: Foundation::WPARAM(0),
      lParam: Foundation::LPARAM(0),
      time: 0,
      pt: Foundation::POINT { x: 0, y: 0 },
    };
    loop {
      let pm = Messaging::GetMessageW(&mut msg, Foundation::HWND(0), 0, 0);
      if pm.0 == 0 { break }
      Messaging::TranslateMessage(&msg);
      Messaging::DispatchMessageW(&msg);
    }
  });
  loop { if let Some(hook) = *hook.lock().unwrap() { return hook }}
}

fn unhook<'a, Param0: IntoParam<'a, Messaging::HHOOK>>(hook: Param0) -> i32 {
  unsafe { Messaging::UnhookWindowsHookEx(hook).0 }
}

unsafe extern "system" fn ll_keyboard_proc(n_code: i32, wp: Foundation::WPARAM, lp: Foundation::LPARAM) -> Foundation::LRESULT {
  if n_code != 0 {
    return Messaging::CallNextHookEx(None, n_code, wp, lp);
  }
  let kbs = &*(lp.0 as *const Messaging::KBDLLHOOKSTRUCT);
  if (kbs.flags & Messaging::LLKHF_INJECTED) != 0 {
    return Messaging::CallNextHookEx(None, n_code, wp, lp);
  }
  if kbs.vkCode == KeyboardAndMouse::VK_E as u32 {
    return Messaging::CallNextHookEx(None, n_code, wp, lp);
  }
  let is_key_down = (wp.0 as u32) == Messaging::WM_KEYDOWN || (wp.0 as u32) == Messaging::WM_SYSKEYDOWN;
  KeyboardAndMouse::SendInput(1, &KeyboardAndMouse::INPUT {
    r#type: KeyboardAndMouse::INPUT_KEYBOARD, 
    Anonymous: KeyboardAndMouse::INPUT_0 {
      ki: KeyboardAndMouse::KEYBDINPUT {
        wVk: if kbs.vkCode == ('Z' as u32) { 'A' as u16 } else { (kbs.vkCode + 1) as u16 },
        dwFlags: if is_key_down { 0 } else { KeyboardAndMouse::KEYEVENTF_KEYUP },
        time: kbs.time,
        dwExtraInfo: kbs.dwExtraInfo,
        wScan: kbs.scanCode as u16
      }
    }
  }, std::mem::size_of::<KeyboardAndMouse::INPUT>() as i32);
  Foundation::LRESULT(-1)
}
