use core::time;

use windows::{core::*, Win32::Media::Multimedia::*};

/*
https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Media/Multimedia/fn.joyGetPosEx.html
https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Media/Multimedia/struct.JOYINFOEX.html
*/

fn main() -> Result<()> {
    while true {
        unsafe {
            //joyGetPosEx(ujoyid:u32, pji:*mut JOYINFOEX) ->u32
            let nums: u32 = joyGetNumDevs();
            println!("Devices: {num}", num = nums);

            let mut joyinfoex: JOYINFOEX = JOYINFOEX::default();
            joyinfoex.dwSize = std::mem::size_of::<JOYINFOEX>() as u32;
            joyinfoex.dwFlags = (JOY_RETURNBUTTONS
                | JOY_RETURNCENTERED
                | JOY_RETURNPOV
                | JOY_RETURNR
                | JOY_RETURNU
                | JOY_RETURNV
                | JOY_RETURNX
                | JOY_RETURNY
                | JOY_RETURNZ) as u32;

            const ujoyid: u32 = JOYSTICKID1;
            let pji: *mut JOYINFOEX = &mut joyinfoex as *mut JOYINFOEX;
            println!("{:?}", pji);
            let ret: u32 = joyGetPosEx(ujoyid, pji);

            if ret == JOYERR_NOERROR {
                let dw_buttons = joyinfoex.dwButtons;
                let dw_xpos = joyinfoex.dwXpos;
                let dw_ypos = joyinfoex.dwYpos;
                let dw_zpos = joyinfoex.dwZpos;
                let dw_rpos = joyinfoex.dwRpos;
                let dw_upos = joyinfoex.dwUpos;
                let dw_vpos = joyinfoex.dwVpos;
                let dw_pov = joyinfoex.dwPOV;

                println!("{id} OK", id = ujoyid);
                println!("Buttons: {:?}", dw_buttons);
                println!("X: {:?}", dw_xpos);
                println!("Y: {:?}", dw_ypos);
                println!("Z: {:?}", dw_zpos);
                println!("R: {:?}", dw_rpos);
                println!("U: {:?}", dw_upos);
                println!("V: {:?}", dw_vpos);
                println!("POV: {:?}", dw_pov);
            } else {
                match ret {
                    JOYERR_NOCANDO => println!("{id} JOYERR_NOCANDO", id = ujoyid),
                    JOYERR_PARMS => println!("{id} JOYERR_PARMS", id = ujoyid),
                    JOYERR_UNPLUGGED => println!("{id} JOYERR_UNPLUGGED", id = ujoyid),
                    _ => println!("{id} ?", id = ujoyid),
                }
            }
        }
        std::thread::sleep(time::Duration::from_millis(1000));
    }

    Ok(())
}
