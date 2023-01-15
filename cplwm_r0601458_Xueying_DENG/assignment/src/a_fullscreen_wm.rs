//! Fullscreen Window Manager
//!
//! Implement the [`WindowManager`] trait by writing a simple window manager
//! that displays every window fullscreen. When a new window is added, the
//! last window that was visible will become invisible.
//!
//! [`WindowManager`]: ../../cplwm_api/wm/trait.WindowManager.html
//!
//! Now have a look at the source code of this file, it contains a tutorial to
//! help you write the fullscreen window manager.
//!
//! You are free to remove the documentation in this file that is only part of
//! the tutorial or no longer matches the code after your changes.
//!
//! # Status
//!
//! **TODO**: Replace the question mark below with YES, NO, or PARTIAL to
//! indicate the status of this assignment. If you want to tell something
//! about this assignment to the grader, e.g., you have a bug you can't fix,
//! or you want to explain your approach, write it down after the comments
//! section.
//!
//! COMPLETED: YES
//!
//! COMMENTS:
//!
//!

// Because not all methods are implemented yet, some arguments are unused,
// which generates warnings. The annotation below disables this warning.
// Remove this annotation when you have implemented all methods, so you get
// warned about variables that you did not use by mistake.
#![allow(unused_variables)]

// We import std::error and std::format so we can say error::Error instead of
// std::error::Error, etc.
use std::error;
use std::fmt;

// Import some types and the WindowManager trait from the cplwm_api crate
// (defined in the api folder).
use cplwm_api::types::{FloatOrTile, PrevOrNext, Screen, Window, WindowLayout, WindowWithInfo};
use cplwm_api::wm::WindowManager;


/// You are free to choose the name for your window manager. As we will use
/// automated tests when grading your assignment, indicate here the name of
/// your window manager data type so we can just use `WMName` instead of
/// having to manually figure out your window manager name.
pub type WMName = FullscreenWM;


/// The FullscreenWM struct
///
/// The first thing to do when writing a window manager, is to define a struct
/// (or enum) that will contain the state of the window manager, e.g. the
/// managed windows along with their geometries, the focused window, etc.
///
/// Depending on the layout and the functionality the window manager provides,
/// this can vary from simple `Vec`s to trees, hashmaps, etc. You can have a
/// look at the [collections](https://doc.rust-lang.org/std/collections/) Rust
/// provides.
///
/// Remember that you are free to add additional dependencies to your project,
/// e.g., for another type of data structure. But this is certainly not
/// required. For more information, see the Hints & Tricks section of the
/// assignment.
///
/// # Example Representation
///
/// The fullscreen window manager that we are implementing is very simple: it
/// just needs to keep track of all the windows that were added and remember
/// which one is focused. It is not even necessary to remember the geometries
/// of the windows, as they will all be resized to the size of the screen.
///
/// A possible data structure to keep track of the windows is a simple `Vec`:
/// the last element in the vector is the window on top, which is also the
/// only window to display. Why not the first element? Because it is easier to
/// add an element to the end of a vector. This is convenient, as adding a new
/// window should also put it on top of the other windows.
///
/// Another thing we need to keep track of is the `Screen`, because we must
/// resize the windows the size of the screen. A `Screen` is passed via the
/// `new` method of the trait and the `resize_screen` method of the trait
/// updates the screen with a new one.
///
/// These two fields are enough to get started, which does not mean that they
/// are enough to correctly implement this window manager. As you will notice
/// in a short while, there is a problem with this representation. Feel free
/// to add/replace/remove fields.
///
/// To understand the `#derive[(..)]` line before the struct, read the
/// [Supertraits] section of the `WindowManager` trait.
///
/// [Supertraits]: ../../cplwm_api/wm/trait.WindowManager.html#supertraits
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct FullscreenWM {
    /// A vector of windows, the first one is on the bottom, the last one is

    /// on top, and also the only visible window.
    pub windows: Vec<Window>,
    /// We need to know which size the fullscreen window must be.
    pub screen: Screen,
    /// keep track of the type after add function
    pub windows_type: Vec<(Window, FloatOrTile)>,
    /// In oreder to meet the requirment of , when there is no window focused, but the windows are still under control.
    /// add a new variable to keep track of focused window
    pub focused_window: Option<Window>,
}


/// The errors that this window manager can return.
///
/// For more information about why you need this, read the documentation of
/// the associated [Error] type of the `WindowManager` trait.
///
/// In the code below, we would like to return an error when we are asked to
/// do something with a window that we do not manage, so we define an enum
/// `FullscreenWMError` with one variant: `UnknownWindow`.
///
/// Feel free to add or remove variants from this enum. You may also replace
/// it with a type or struct if you wish to do so.
///
/// [Error]: ../../cplwm_api/wm/trait.WindowManager.html#associatedtype.Error
#[derive(Debug)]
pub enum FullscreenWMError {
    /// This window is not known by the window manager.
    UnknownWindow(Window),
    /// This window is managed by the windows manager right now which can't be added again.
    ManagedWindow(Window),
}

// This code is explained in the documentation of the associated [Error] type
// of the `WindowManager` trait.
impl fmt::Display for FullscreenWMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FullscreenWMError::UnknownWindow(ref window) => write!(f, "Unknown window: {}", window),
            FullscreenWMError::ManagedWindow(ref window) => write!(f, "Managed window: {}", window),
        }
    }
}

// This code is explained in the documentation of the associated [Error] type
// of the `WindowManager` trait.
impl error::Error for FullscreenWMError {
    fn description(&self) -> &'static str {
        match *self {
            FullscreenWMError::UnknownWindow(_) => "Unknown window",
            FullscreenWMError::ManagedWindow(_) => "Managed window",
        }
    }
}

// Now we start implementing our window manager
impl WindowManager for FullscreenWM {
    /// We use `FullscreenWMError` as our `Error` type.
    type Error = FullscreenWMError;

    /// The constructor is straightforward.
    ///
    /// Track the given screen and make a new empty `Vec`.
    /// Add new window_type here, all variables should be initialted
    fn new(screen: Screen) -> FullscreenWM {
        FullscreenWM {
            windows: Vec::new(),
            screen: screen,
            windows_type: Vec::new(),
            focused_window: None,
        }
    }

    /// The `windows` field contains all the windows we manage.
    ///
    /// Why do we need a `clone` here?
    fn get_windows(&self) -> Vec<Window> {
        self.windows.clone()
    }

    /// The last window in the list is the focused one.
    ///
    /// Note that the `last` method of `Vec` returns an `Option`.
    fn get_focused_window(&self) -> Option<Window> {
        self.focused_window
    }

    /// To add a window, just push it onto the end the `windows` `Vec`.
    ///
    /// We could choose to return an error when the window is already managed
    /// by the window manager, but in this case we just do nothing. You are
    /// free to define another error to handle this case.
    ///
    /// Note that we completely ignore the information that comes along with
    /// the info, this *could* lead to issues in later assignments.

    /// **Invariant**: after adding a window using `add_window`, it must be
    /// focused according to `get_focused_window`.

    /// check the window is managed or not
    /// if it is managed return an error: ManagedWindwo
    /// else:add the window to windows
    ///     add the window and its type to windows_type
    ///     the last window in window is the focused one
    fn add_window(&mut self, window_with_info: WindowWithInfo) -> Result<(), Self::Error> {

        if !self.is_managed(window_with_info.window) {
            self.windows.push(window_with_info.window);
            self.windows_type.push((window_with_info.window, window_with_info.float_or_tile));
            self.focused_window = Some(window_with_info.window);
            Ok(())
        } else {
            Err(FullscreenWMError::ManagedWindow(window_with_info.window))
        }

    }

    /// To remove a window, just remove it from the `windows` `Vec`.
    ///
    /// First we look up the position (or index) of the window in `windows`,
    /// and then remove it unless the window does not occur in the `Vec`, in
    /// which case we return an error.
    fn remove_window(&mut self, window: Window) -> Result<(), Self::Error> {
        match self.windows.iter().position(|w| *w == window) {
            None => Err(FullscreenWMError::UnknownWindow(window)),
            Some(i) => {
                self.windows.remove(i);
                self.focused_window = self.windows.last().map(|w| *w);
                Ok(())
            }
        }
    }

    /// Now the most important part: calculating the `WindowLayout`.
    ///
    /// First we build a `Geometry` for a fullscreen window using the
    /// `to_geometry` method: it has the same width and height as the screen.
    ///
    /// Then we look at the last window, remember that the `last()` method of
    /// `Vec` returns an `Option`.
    ///
    /// * When the `Option` contains `Some(w)`, we know that there was at
    ///   least one window, and `w`, being the last window in the `Vec` should
    ///   be focused. As the other windows will not be visible, the `windows`
    ///   field of `WindowLayout` can just be a `Vec` with one element: the
    ///   one window along with the fullscreen `Geometry`.
    ///
    /// * When the `Option` is `None`, we know that there are no windows, so
    ///   we can just return an empty `WindowLayout`.
    ///
    fn get_window_layout(&self) -> WindowLayout {
        // scree to geometry ,the default x=0;y=0.
        let fullscreen_geometry = self.screen.to_geometry();
        match self.windows.last() {
            // If there is at least one window.
            Some(w) => {
                WindowLayout {
                    // The last window is focused ...
                    focused_window: self.focused_window,
                    // ... and should fill the screen. The other windows are
                    // simply hidden.
                    windows: vec![(*w, fullscreen_geometry)],
                }
            }
            // Otherwise, return an empty WindowLayout
            None => WindowLayout::new(),
        }
    }


    /// set the focused window
    /// if there is a window need to be focused, delete it from windows vec and push it to the vecotr.
    /// otherwise set the focused window to be None
    fn focus_window(&mut self, window: Option<Window>) -> Result<(), Self::Error> {

        match window {

            // the last window in the vector is the focused one.
            // when the windows in the list should be focused , we put it to the last of the vec.
            Some(i) => {
                match self.windows.iter().position(|w| *w == i) {
                    None => Err(FullscreenWMError::UnknownWindow(i)),
                    Some(w) => {
                        self.remove_window(i).unwrap();
                        self.windows.push(i);
                        self.focused_window = Some(i);
                        Ok(())
                    }
                }
            }
            // when there is no window should be focusd, the windows vec can't to be empty.
            None => {
                self.focused_window = None;
                Ok(())
            }

        }
    }


    /// cycle focus the window,
    fn cycle_focus(&mut self, dir: PrevOrNext) {
        // You will probably notice here that a `Vec` is not the ideal data
        // structure to implement this function. Feel free to replace the
        // `Vec` with another data structure.
        // do nothing if there is no window
        if self.windows.is_empty() == true {
            return ();
        } else {
            // if there is no window focused.
            if self.get_focused_window() == None {
                // focuse the last in the vector,not random one
                let temp_prev = self.windows.last().map(|i| *i);
                self.focus_window(temp_prev).unwrap();
            } else {
                match dir {
                    PrevOrNext::Prev => {
                        // get the length of the vec
                        let len = self.windows.len();
                        // remove the last number of the vec
                        let num = self.windows.remove(len - 1);
                        // put it to the beginning
                        self.windows.insert(0, num);

                        // make it foucused
                        let temp_prev = self.windows.last().map(|i| *i);
                        self.focus_window(temp_prev).unwrap();
                    }
                    PrevOrNext::Next => {
                        // get the length of the vec
                        let len = self.windows.len();
                        // remove the fist number of the vec
                        let num = self.windows.remove(0);
                        // put it to the end
                        self.windows.push(num);
                        // make it foucused
                        let temp_prev = self.windows.last().map(|i| *i);
                        self.focus_window(temp_prev).unwrap();
                    }

                }
            }

        }
    }

    /// get the information of the window.
    fn get_window_info(&self, window: Window) -> Result<WindowWithInfo, Self::Error> {
        match self.windows.iter().position(|w| *w == window) {
            None => Err(FullscreenWMError::UnknownWindow(window)),
            Some(i) => {
                let index = self.windows_type
                    .clone()
                    .iter()
                    .map(|w| w.0)
                    .collect::<Vec<_>>()
                    .iter()
                    .position(|w| *w == window)
                    .unwrap();
                // unwrap , we have checked it is in the list.

                Ok(WindowWithInfo {
                    window: window,
                    geometry: self.screen.to_geometry(),
                    float_or_tile: self.windows_type[i].1,
                    fullscreen: false,
                })
            }
        }
    }
    /// get the sreen of current windows manager
    fn get_screen(&self) -> Screen {
        self.screen
    }

    /// resize the window with provide screen size
    fn resize_screen(&mut self, screen: Screen) {
        self.screen = screen
    }
}


#[cfg(test)]
mod tests {

    // We have to import `FullscreenWM` from the super module.
    use super::FullscreenWM;
    // We have to repeat the imports we did in the super module.
    use cplwm_api::wm::WindowManager;
    use cplwm_api::types::*;

    // We define a static variable for the screen we will use in the tests.
    // You can just as well define it as a local variable in your tests.
    static SCREEN: Screen = Screen {
        width: 800,
        height: 600,
    };

    // We define a static variable for the geometry of a fullscreen window.
    // Note that it matches the dimensions of `SCREEN`.
    static SCREEN_GEOM: Geometry = Geometry {
        x: 0,
        y: 0,
        width: 800,
        height: 600,
    };

    // We define a static variable for some random geometry that we will use
    // when adding windows to a window manager.
    static SOME_GEOM: Geometry = Geometry {
        x: 10,
        y: 10,
        width: 100,
        height: 100,
    };


    // Now let's write our test.
    //
    // Note that tests are annotated with `#[test]`, and cannot take arguments
    // nor return anything.
    #[test]
    fn test_adding_and_removing_some_windows() {
        // Let's make a new `FullscreenWM` with `SCREEN` as screen.
        let mut wm = FullscreenWM::new(SCREEN);

        // Initially the window layout should be empty.
        assert_eq!(WindowLayout::new(), wm.get_window_layout());
        // `assert_eq!` is a macro that will check that the second argument,
        // the actual value, matches first value, the expected value.

        // Let's add a window
        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
        // Because `add_window` returns a `Result`, we use `unwrap`, which
        // tries to extract the `Ok` value from the result, but will panic
        // (crash) when it is an `Err`. You must be very careful when using
        // `unwrap` in your code. Here we can use it because we know for sure
        // that an `Err` won't be returned, and even if that were the case,
        // the panic will simply cause the test to fail.

        // The window should now be managed by the WM
        assert!(wm.is_managed(1));
        // and be present in the `Vec` of windows.
        assert_eq!(vec![1], wm.get_windows());
        // According to the window layout
        let wl1 = wm.get_window_layout();
        // it should be focused
        assert_eq!(Some(1), wl1.focused_window);
        // and fullscreen.
        assert_eq!(vec![(1, SCREEN_GEOM)], wl1.windows);

        // Let's add another window.
        wm.add_window(WindowWithInfo::new_tiled(2, SOME_GEOM)).unwrap();
        // It should now be managed by the WM.
        assert!(wm.is_managed(2));
        // The `Vec` of windows should now contain both windows 1 and 2.
        assert_eq!(vec![1, 2], wm.get_windows());
        // According to the window layout
        let wl2 = wm.get_window_layout();
        // window 2 should be focused
        assert_eq!(Some(2), wl2.focused_window);
        // and fullscreen.
        assert_eq!(vec![(2, SCREEN_GEOM)], wl2.windows);

        // Now let's remove window 2
        wm.remove_window(2).unwrap();
        // It should no longer be managed by the WM.
        assert!(!wm.is_managed(2));
        // The `Vec` of windows should now just contain window 1.
        assert_eq!(vec![1], wm.get_windows());
        // According to the window layout
        let wl3 = wm.get_window_layout();
        // window 1 should be focused again
        assert_eq!(Some(1), wl3.focused_window);
        // and fullscreen.
        assert_eq!(vec![(1, SCREEN_GEOM)], wl3.windows);
    }




    // ******************************************************************************************************
    // implementation test
    #[test]
    fn test_focused_windows() {
        let mut wm = FullscreenWM::new(SCREEN);
        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(2, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(3, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(4, SOME_GEOM)).unwrap();
        // when the window 1 is focused, then test
        wm.focus_window(Some(1)).unwrap();
        assert_eq!(Some(1), wm.get_focused_window());
        let wl4 = wm.get_window_layout();
        assert_eq!(Some(1), wl4.focused_window);

        // When no window should  be focused, `get_focused_window()` must return `None` afterwards.
        wm.focus_window(None).unwrap();
        assert_eq!(None, wm.get_focused_window());
        let wl5 = wm.get_window_layout();
        // The `focused_window` field of the `WindowLayout` must also be `None`.
        assert_eq!(None, wl5.focused_window);

        assert_eq!(vec![2, 3, 4, 1], wm.get_windows());

        // test cycle_focus.
        assert_eq!(vec![2, 3, 4, 1], wm.get_windows());
        wm.cycle_focus(PrevOrNext::Prev);
        assert_eq!(vec![2, 3, 4, 1], wm.get_windows());
        wm.cycle_focus(PrevOrNext::Prev);
        assert_eq!(vec![1, 2, 3, 4], wm.get_windows());
        let wl5 = wm.get_window_layout();
        // The `focused_window` field of the `WindowLayout` must also be `None`.
        assert_eq!(Some(4), wl5.focused_window);
        wm.cycle_focus(PrevOrNext::Next);
        assert_eq!(vec![2, 3, 4, 1], wm.get_windows());
        let wl5 = wm.get_window_layout();
        // The `focused_window` field of the `WindowLayout` must also be `None`.
        assert_eq!(Some(1), wl5.focused_window);
    }

    #[test]
    fn test_get_windows_info() {
        let mut wm = FullscreenWM::new(SCREEN);
        // test get_window_info

        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
        assert_eq!(Some(WindowWithInfo::new_tiled(1, SCREEN_GEOM)),
                   wm.get_window_info(1).ok());
        assert_eq!(true, wm.get_window_info(6).is_err());
        // test get_screen
        assert_eq!(SCREEN, wm.get_screen());


        // test reseize screen

        static NEW_SCREEN: Screen = Screen {
            width: 200,
            height: 300,
        };
        wm.resize_screen(NEW_SCREEN);

    }
}
