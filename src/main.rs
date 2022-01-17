use windows::{
    core::*,
    Win32::{
        Foundation::HWND,
        UI::{
            Input::KeyboardAndMouse::{GetKeyboardLayoutList, UnloadKeyboardLayout},
            TextServices::HKL,
            WindowsAndMessaging::*,
        },
    },
};

const KOREAN_HKL: isize = 68289554;
const DEFAULT_BUFFER_SIZE: usize = 10;

fn show_message_box(text: &str, caption: &str) {
    unsafe {
        MessageBoxW(
            HWND::default(),
            text,
            caption,
            MB_OK | MB_ICONINFORMATION,
        );
    }
}

fn main() -> Result<()> {
    unsafe {
        let mut lp_list: [HKL; DEFAULT_BUFFER_SIZE] = [HKL(0); DEFAULT_BUFFER_SIZE];
        let keyboard_layout_size = GetKeyboardLayoutList(
            DEFAULT_BUFFER_SIZE as i32, lp_list.as_mut_ptr(),
        );

        let mut size = 0;
        for lp in lp_list {
            if lp.0 != 0 && lp.0 != KOREAN_HKL {
                UnloadKeyboardLayout(lp);
                size += 1;
            }
        }

        show_message_box(
            &format!("{}개의 키보드 레이아웃 중 {}개를 삭제했습니다.", keyboard_layout_size, size),
            "성공",
        )
    }

    Ok(())
}