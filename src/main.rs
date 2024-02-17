extern crate ncurses;

use ncurses::*;
use std::process::Command;


fn main() {
    initscr();
    noecho();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    handle_control();
}

fn handle_control(){
    let options = [ "poweroff", "reboot", "zzz", "quit"];
    let mut hlitem: usize = 0;

    loop {
        for i in 0..4 {
            mv(i as i32, 0);
            clrtoeol();
            if i == hlitem{
                attron(A_REVERSE());
                addstr(options[i].as_ref());
                attroff(A_REVERSE());
            }
            else {
                addstr(options[i].as_ref());
            }
        }
        match getch(){
            //k
            107 => {
                if hlitem > 0{
                    hlitem-=1;
                }
            },
            //j
            106 => {
                hlitem+=1;
                if hlitem >= 4{
                    hlitem = 3;
                }
            },
            //enter
            10 => {
                endwin();
                let cmd = match hlitem{
                    3 => return,
                    _ => options[hlitem],
                };
                loop {
                    if Command::new("doas").arg(cmd).spawn().expect("perhaps doas isn't installed").wait().expect("power program failed").success(){
                        return;
                    }
                    else{
                        println!("Password failed try again");
                    }
                }
            },
            _ => (),
        }
    }
}
