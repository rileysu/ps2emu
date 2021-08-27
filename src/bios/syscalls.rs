fn reset_ee(reset_flag: u8) {
    //idk
}

fn set_gs_crt(interlaced: bool, display_mode: u8, frame: bool) {
    

    load_exec_ps2("rom0:OSDSYS", 1, "BootBrowser");
}