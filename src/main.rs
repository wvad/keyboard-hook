use windows::Win32::{Foundation::*, UI::{WindowsAndMessaging::*, Input::KeyboardAndMouse::*}};
use std::io::Write;

mod windows_hook;

fn main() {
  // Print prompt and flush.
  print!(">");
  std::io::stdout().flush().unwrap(); 

  // Create a hook.
  let hook = windows_hook::set(WH_KEYBOARD_LL, Some(ll_keyboard_proc));

  // Receive input.
  let mut guess = String::new();
  std::io::stdin().read_line(&mut guess).expect("Error: Failed on 'read_line()'.");

  // Unhook.
  windows_hook::unhook(hook);
}

unsafe extern "system" fn ll_keyboard_proc(n_code: i32, wp: WPARAM, lp: LPARAM) -> LRESULT {
  if n_code != 0 {
    return CallNextHookEx(None, n_code, wp, lp);
  }
  let kbs = &*(lp.0 as *const KBDLLHOOKSTRUCT);
  if (kbs.flags & LLKHF_INJECTED) != 0 {
    return CallNextHookEx(None, n_code, wp, lp);
  }
  let is_key_down = (wp.0 as u32) == WM_KEYDOWN || (wp.0 as u32) == WM_SYSKEYDOWN;
  SendInput(1, &INPUT {
    r#type: INPUT_KEYBOARD,
    Anonymous: INPUT_0 {
      ki: KEYBDINPUT {
        wVk: if kbs.vkCode == ('Z' as u32) { 'A' as u16 } else { (kbs.vkCode + 1) as u16 },
        dwFlags: if is_key_down { 0 } else { KEYEVENTF_KEYUP },
        time: kbs.time,
        dwExtraInfo: kbs.dwExtraInfo,
        wScan: kbs.scanCode as u16
      }
    }
  }, std::mem::size_of::<INPUT>() as i32);
  LRESULT(-1)
}
