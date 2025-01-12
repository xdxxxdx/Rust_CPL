//! Runner of your window manager
//!
//!
//! To actually run your window manager do the following things:
//!
//! # Choose which window manager to run
//!
//! Choose which assignment's window manager you want to run. The backend is
//! written with the window manager of assignment G (multiple workspaces) in
//! mind. If you have not implemented this window manager yet, you can still
//! run a window manager from a previous assignment by providing dummy (just
//! do nothing, or use the [`unimplemented!`] macro) implementations for the
//! missing traits. You can also comment out code in this file that uses
//! unimplemented functionality.
//!
//! [`unimplemented!`]: https://doc.rust-lang.org/std/macro.unimplemented.html
//!
//! Don't forget to change the [`WM`](type.WM.html) type to point to the name
//! of the window manager you want to run.
//!
//! # Start an X server and run your window manager
//!
//! To run the window manager, you need an X server (the window system) of
//! which the window manager can *manage the windows*. Nearly all Linux and
//! UNIX-like operating systems come with an X server. Even though there are X
//! server implementations for [Windows] and [macOS], and it is theoretically
//! possible to run the window manager on those platforms, there is still a
//! lot of fiddling required ([for the brave]). Therefore, we advise you to
//! use a Linux machine for this, or, when you do not have Linux installed, to
//! just download the latest version of [Ubuntu] and run it in [VirtualBox].
//! You can also use one of the workstations in the labs.
//!
//! [Windows]: http://www.straightrunning.com/XmingNotes/
//! [macOS]: https://www.xquartz.org/
//! [for the brave]: http://fanf.livejournal.com/142372.html
//! [Ubuntu]: https://www.ubuntu.com/download/desktop
//! [VirtualBox]: https://www.virtualbox.org/wiki/Downloads
//!
//! Remember that you can successfully complete the whole project without ever
//! actually running the window manager. That being said, actually running
//! your window manager can help you find bugs in your implementation and
//! demonstrate that some of the choices you made are in practice not so
//! intuitive after all.
//!
//! It is possible to let the window manager replace the window manager you
//! are actually using, but this is not recommend. When the window manager
//! crashes or has a bug, it would mean that, for example, you can't click on
//! a certain window anymore. Or maybe your (evil) window manager accidentally
//! closes all windows, which would not be very nice.
//!
//! Therefore we offer you two ways to run another X server in parallel with
//! the X server that is already running. This way, the X server that is
//! perhaps displaying the windows of your code editor and your browser will
//! not be affected when the window manager you wrote crashes.
//!
//! ## Nested X server (recommended)
//!
//! The first and recommended way is to nest a virtual X server within your
//! current X server. A window will appear that will show the nested X server.
//! It looks similar to a remote desktop.
//!
//! For this, you need to install the nested X server named Xephyr, e.g.
//!
//! ```bash
//! $ sudo apt install xserver-xephyr
//! ```
//!
//! This package is already installed on the workstation in the PC labs.
//!
//! You can start a nested X server with the following command (feel free to
//! change the resolution):
//!
//! ```bash
//! $ Xephyr -ac -br -noreset -screen 1024x768 :1
//! ```
//!
//! When you can't see the mouse pointer in the window that appeared, execute
//! the following command in a terminal (a terminal in your current X server
//! also works):
//!
//! ```bash
//! $ DISPLAY=:1 xsetroot -cursor_name left_ptr
//! ```
//!
//! Because we change the `DISPLAY` environment variable to `:1`, which is the
//! number of the display of the nested X server, the command is executed in
//! the nested X server and not the current X server (which has display number
//! `:0`).
//!
//! To start your window manager, run the following command in the `runner`
//! folder:
//!
//! ```bash
//! $ DISPLAY=:1 cargo run
//! ```
//!
//! This will use cargo to run the [`main`] function of this file.
//!
//! Some key presses might already be intercepted by your current window
//! manager or desktop environment, for example <kbd>Alt-Tab</kbd> is probably
//! already defined by your running window manager. In order to send
//! <kbd>Alt-Tab</kbd> and any other intercepted key presses to the nested X
//! server, focus the nested X server window and let it *capture* the input by
//! pressing <kbd>Ctrl-Shift</kbd>. Now all key presses will be sent directly
//! to the nested X server without being intercepted. You will also notice
//! that your mouse cursor cannot leave the window anymore. Press
//! <kbd>Ctrl-Shift</kbd> again to *uncapture* the input.
//!
//! ## X server on another virtual console
//!
//! An alternative way to run another X server in parallel is the following.
//! By default, the X server is running on the 7th [virtual console]. It is
//! possible to switch to another virtual console and run another X server on
//! it. First create the `~/.xinitrc` file (the script that will be executed
//! when the X server is started) with the following contents:
//!
//! ```bash
//! #!/bin/sh
//! xsetroot -cursor_name left_ptr
//! cd PATH/TO/THE/RUNNER/FOLDER
//! exec cargo run
//! ```
//!
//! The `xsetroot` command makes sure you can see the mouse pointer. Adjust
//! the path so that it points to the `runner` directory. The `exec` command
//! will use cargo to run the [`main`] function of this file. Make sure this
//! script is executable by running:
//!
//! ```bash
//! $ chmod +x ~/.xinitrc
//! ```
//!
//! Use <kbd>Ctrl-Alt-F1</kbd> to switch to the 1st virtual console (this will
//! look like a fullscreen terminal), log yourself in, and start a new X
//! server by executing the following command:
//!
//! ```bash
//! $ startx
//! ```
//!
//! A new X server will start and your window manager will *manage* its
//! windows. You can always switch back to your previous X server with
//! <kbd>Ctrl-Alt-F7</kbd>.
//!
//! [virtual console]: https://en.wikipedia.org/wiki/Virtual_console
//!
//! # Using the window manager
//!
//! Now that you have started another X server (nested or on another virtual
//! console) that is running your window manager, you can begin using it.
//!
//! For instance, try to spawn a terminal with <kbd>Super-Return</kbd>
//! (Windows key and the Enter key). Make it float by pressing
//! <kbd>Super-t</kbd> and move it around by holding down the Windows key and
//! the left mouse button. Have a look at the key and button bindings in the
//! [`main`] function in this file. Feel free to add/change/remove bindings.
//!
//! # Tips
//!
//! * When your window manager crashes, look at the log file `cplwm.log` that
//!   is generated either in the `runner` folder or in your home directory,
//!   depending on how you launch the window manager.
//!
//! * You can change and recompile the code of your window manager, the runner
//!   (e.g., when you have added new bindings), or even the backend, and
//!   reload the window manager by pressing <kbd>Super-Shift-r</kbd> without
//!   having to shut it down first or losing the layout of the windows.
//!
//! * You can use the [`spawn`](fn.spawn.html) function to launch new
//!   applications.
//!
//! [`main`]: fn.main.html
#![deny(missing_docs)]

extern crate cplwm_api;
extern crate cplwm_assignment;
#[macro_use]
extern crate cplwm_x11;
#[macro_use]
extern crate log;
extern crate simplelog;
extern crate x11_dl;

use log::LogLevelFilter;
use simplelog::{CombinedLogger, FileLogger, SharedLogger, TermLogger};

use std::fs::File;
use std::process;

use cplwm_api::wm::*;
use cplwm_api::types::{PrevOrNext, WorkspaceIndex};

use cplwm_x11::{X11Backend, X11Config, X11Error, X11Result};

// Import the `WMName` of the window manager you want to run
use cplwm_assignment::f_gaps::WMName;

/// The name of the window manager you want to run.
///
/// This type is used in the rest of this file.
///
/// This is to make it easier to change the window manager you want to run.
/// You can just change this type instead of replacing all the occurrences
/// throughout this file.
pub type WM = WMName;

/// Run a command.
///
/// This is just a helper that ignores the value in the `Ok` and converts it
/// to an `X11Result`. You can use this when defining key or button binding
/// commands. You can also use it to convert the error, e.g., a window manager
/// error, into an `X11Error`.
pub fn run<T, E: Into<X11Error>>(res: Result<T, E>) -> X11Result<()> {
    match res {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

/// Spawn a new process.
///
/// For example, launch an xterm with `spawn("xterm")`. The command can
/// contain arguments separated by spaces.
pub fn spawn(command: &str) -> X11Result<()> {
    run(process::Command::new("sh").arg("-c").arg(command).spawn())
}

/// Move the focused window to the given workspace.
///
/// Do nothing when no window is focused or when the index is that of the
/// currently active workspace.
fn move_focused_window_to_workspace(backend: &mut X11Backend<WM>,
                                    index: WorkspaceIndex)
                                    -> X11Result<()> {
    if let Some(w) = backend.get_wm().get_focused_window() {
    //    if index != backend.get_wm().get_current_workspace_index() {
            let wm = backend.get_wm_mut();
            let window_with_info = try!(wm.get_window_info(w));
            try!(wm.remove_window(w));
    //        try!(wm.switch_workspace(index));
            try!(wm.add_window(window_with_info));
 //       }
    }
    Ok(())
}


/// Start the window manager.
pub fn main() {

    // Log to both stdout and a file
    let log_file = File::create("cplwm.log").unwrap();
    let loggers: Vec<Box<SharedLogger>> = vec![TermLogger::new(LogLevelFilter::Trace).unwrap(),
                                               FileLogger::new(LogLevelFilter::Trace, log_file)];
    let _ = CombinedLogger::init(loggers);

    info!("Starting the window manager");
    let mut config = X11Config::default();

    // Feel free to add/remove key/button bindings or change other fields of
    // the `config`. Have a look at the documentation of the key_bindings!
    // macro.

    // Remember: Super = the Windows key
    config.key_bindings = key_bindings! { WM =>
        // Launch xterm
        (Super - XK_Return) => |_| spawn("xterm"),
        // Launch xclock
        (Super - XK_c) => |_| spawn("xclock"),
        // Quit
        (Super - Shift - XK_q) => |_| process::exit(0),
        // Restart
        (Super - Shift - XK_r) => |backend| { backend.restart(true); Ok(()) },
        // Close the focused window
        (Super - XK_k) => |backend| {
            if let Some(w) = backend.get_wm().get_focused_window() {
                backend.close_window(w)
            }
            Ok(())
        },
        // Float/sink the current window
        (Super - XK_t) => |backend| {
            if let Some(w) = backend.get_wm().get_focused_window() {
                try!(backend.get_wm_mut().toggle_floating(w));
            }
            Ok(())
        },
        // Focus the next window
        (Alt - XK_Tab) => |backend| {
            backend.get_wm_mut().cycle_focus(PrevOrNext::Next);
            Ok(())
        },
        // Focus the previous window
        (Alt - Shift - XK_Tab) => |backend| {
            backend.get_wm_mut().cycle_focus(PrevOrNext::Prev);
            Ok(())
        },
        // Swap with the next window
        (Super - XK_n) => |backend| {
            backend.get_wm_mut().swap_windows(PrevOrNext::Next);
            Ok(())
        },
        // Swap with the previous window
        (Super - XK_p) => |backend| {
            backend.get_wm_mut().swap_windows(PrevOrNext::Prev);
            Ok(())
        },
        // Swap with the master window
        (Super - XK_h) => |backend| {
            if let Some(w) = backend.get_wm().get_focused_window() {
                try!(backend.get_wm_mut().swap_with_master(w));
            }
            Ok(())
        },
        // Minimise the focused window
        (Super - XK_m) => |backend| {
            if let Some(w) = backend.get_wm().get_focused_window() {
                try!(backend.get_wm_mut().toggle_minimised(w));
            }
            Ok(())
        },
        // Unminimise the last minimised window
        (Super - Shift - XK_m) => |backend| {
            if let Some(w) = backend.get_wm().get_minimised_windows().last() {
                try!(backend.get_wm_mut().toggle_minimised(*w));
            }
            Ok(())
        },
        // Toggle fullscreen
        (Super - XK_f) => |backend| {
            if let Some(w) = backend.get_wm().get_focused_window() {
                try!(backend.get_wm_mut().toggle_fullscreen(w));
            }
            Ok(())
        },
        // Increase the gap
        (Super - XK_g) => |backend| {
            let current_gap = backend.get_wm().get_gap();
            backend.get_wm_mut().set_gap(current_gap + 1);
            Ok(())
        },
        // Decrease the gap
        (Super - Shift - XK_g) => |backend| {
            let current_gap = backend.get_wm().get_gap();
            if current_gap > 0 {
                backend.get_wm_mut().set_gap(current_gap - 1);
            }
            Ok(())
        },
        // We could use a loop for this
   //     (Super - XK_1) => |backend| run(backend.get_wm_mut().switch_workspace(0)),
   //        (Super - XK_2) => |backend| run(backend.get_wm_mut().switch_workspace(1)),
    //    (Super - XK_3) => |backend| run(backend.get_wm_mut().switch_workspace(2)),
   //     (Super - XK_4) => |backend| run(backend.get_wm_mut().switch_workspace(3)),
        (Super - Shift - XK_1) => |backend| move_focused_window_to_workspace(backend, 0),
        (Super - Shift - XK_2) => |backend| move_focused_window_to_workspace(backend, 1),
        (Super - Shift - XK_3) => |backend| move_focused_window_to_workspace(backend, 2),
        (Super - Shift - XK_4) => |backend| move_focused_window_to_workspace(backend, 3)
    };

    config.button_bindings = button_bindings! { WM =>
        // Move the current window
        (Super - LMB) => |backend, ev| backend.mouse_move_window(ev.subwindow),
        // Resize the current window
        (Super - RMB) => |backend, ev| backend.mouse_resize_window(ev.subwindow)
    };

    X11Backend::start(WM::new, config).unwrap();

    info!("The window manager has stopped");
}
