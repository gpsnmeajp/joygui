use windows::Win32::Media::Multimedia::*;

#[derive(Debug, Default)]
pub struct JoystickData {
    pub axis_x: f32,
    pub axis_y: f32,
    pub axis_z: f32,
    pub axis_r: f32,
    pub axis_u: f32,
    pub axis_v: f32,
    pub pov: Option<u32>,
    pub buttons: [bool; 32],
}

#[derive(Debug)]
pub enum JoystickError {
    Unknown,
    Nocando,
    Params,
    Unplugged,
}

fn to1(v:u32) -> f32 {
    ((((v as i64) - 32767) as f32) / 32767.0).clamp(-1.0, 1.0)
}

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
