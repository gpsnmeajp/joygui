//! This is a mini program of joystick handling in windows.

use windows::Win32::Media::Multimedia::*;

#[derive(Debug, Default)]
/// JoystickData for each joystick.
pub struct JoystickData {
    /// axis position
    pub axis_x: f32,

    /// axis position
    pub axis_y: f32,

    /// axis position
    pub axis_z: f32,

    /// axis position
    pub axis_r: f32,

    /// axis position
    pub axis_u: f32,

    /// axis position
    pub axis_v: f32,

    /// POV is a 32-bit value representing the POV of the joystick.
    /// value is in the range 0 to 36000. ( x100 degles )
    /// None is not input
    pub pov: Option<u32>,

    /// buttons a list of joystick buttons.
    /// index 0 is button 1
    /// index 31 is button 32
    pub buttons: [bool; 32],
}

#[derive(Debug)]
/// Error type for joystick handling.
pub enum JoystickError {
    /// Unknown error. ( Unexpected error )
    Unknown,

    /// Request not completed
    Nocando,

    /// Invalid parameter.
    Params,

    /// Joystick is unplugged
    Unplugged,
}

/// 32bit value to normalized floating value.
fn to1(v:u32) -> f32 {
    ((((v as i64) - 32767) as f32) / 32767.0).clamp(-1.0, 1.0)
}

/// Get joystick data.
/// 
/// * `joystick_id` - joystick id. (0-15)
pub fn update(joystick_id: u32) -> Result<JoystickData, JoystickError> {
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

    let pji: *mut JOYINFOEX = &mut joyinfoex as *mut JOYINFOEX;
    let ret: u32;
    unsafe {
        ret = joyGetPosEx(joystick_id, pji);
    }

    if ret == JOYERR_NOERROR {
        let mut buttons: [bool; 32] = Default::default();
        for i in 0..32 {
            buttons[i] = (joyinfoex.dwButtons >> i)&1 != 0;
        }

        let data = JoystickData {
            axis_x: to1(joyinfoex.dwXpos),
            axis_y: to1(joyinfoex.dwYpos),
            axis_z: to1(joyinfoex.dwZpos),
            axis_r: to1(joyinfoex.dwRpos),
            axis_u: to1(joyinfoex.dwUpos),
            axis_v: to1(joyinfoex.dwVpos),
            pov: if joyinfoex.dwPOV == 65535 { None } else { Some(joyinfoex.dwPOV) },
            buttons: buttons,
        };

        return Ok(data);
    } else {
        return Err(match ret {
            JOYERR_NOCANDO => JoystickError::Nocando,
            JOYERR_PARMS => JoystickError::Params,
            JOYERR_UNPLUGGED => JoystickError::Unplugged,
            _ => JoystickError::Unknown,
        });
    }
}
