//! Optional: Gaps
//!
//! Extend your window manager with support for gaps, i.e. the ability to add
//! some space between the different tiles. See the documentation of the
//! [`GapSupport`] trait for the precise requirements.
//!
//! Make a copy of your tiling window manager from assignment B and let it
//! implement the [`GapSupport`] trait. You are not required to let this
//! window manager implement all the previous traits.
//!
//! [`GapSupport`]: ../../cplwm_api/wm/trait.GapSupport.html
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
//! ...
//!

#![allow(unused_variables)]
// Add imports here
use std::error;
use std::fmt;
use cplwm_api::types::{FloatOrTile, GapSize, Geometry, PrevOrNext, Screen, Window, WindowLayout,
                       WindowWithInfo, WorkspaceIndex};
use cplwm_api::wm::WindowManager;
use cplwm_api::wm::TilingSupport;
use cplwm_api::wm::FloatSupport;
use cplwm_api::wm::MinimiseSupport;
use cplwm_api::wm::FullscreenSupport;
use cplwm_api::wm::GapSupport;
use cplwm_api::wm::MultiWorkspaceSupport;
/// FloatscreenWM
pub type WMName = FloatscreenWM;




/// Stuct of Tile screen windows manager
#[derive(RustcDecodable, RustcEncodable, Debug, Clone)]
pub struct FloatscreenWM {
    /// The vectors of the windows, the first one is on the master tile
    pub windows: Vec<Window>,
    /// The full screen windows,contains all full_screen windows, but only the last one is visible. when you close all full screen windows.
    /// you will go back to the previous look, when you set the first full screen window.
    /// 1st is current full screen window
    /// 2nd is it proviously geometry
    /// 3rd is its type
    /// last one is the provious focused window , before that one is set to be full screen.
    pub full_screen_windows: Vec<(Window, Geometry, FloatOrTile, Option<Window>)>,
    /// The vectors of the focus window, when it is empty then there is no focused window, which only can contain 1 object at maximum.
    pub focused_window: Option<Window>,
    /// all float windows that are under controlled
    pub float_windows: Vec<(Window, Geometry)>,
    /// all tiling windows that are under controlled
    pub tile_windows: Vec<Window>,
    /// we need to know the current size of the screen.
    pub screen: Screen,
    /// original windows with the geometry
    pub original_windows: Vec<(Window, Geometry)>,
    /// minimised windows with the geometry
    pub minimised_windows: Vec<(Window, Geometry, FloatOrTile)>,
    /// gap
    pub gap: GapSize,
}



#[derive(Debug)]
/// The error for tile screen
pub enum FloatscreenWMError {
    /// This window is not known by the window manager.
    UnknownWindow(Window),
    /// This window is not a tile window.
    NotATileWindow(Window),
    /// This window is not a float window.
    NotAFloatWindow(Window),
    /// This window is managed by the windows manager right now which can't be added again.
    ManagedWindow(Window),
}

/// Display fuction for Tile Screen Error
impl fmt::Display for FloatscreenWMError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FloatscreenWMError::UnknownWindow(ref window) => {
                write!(f, "Unknown window: {}", window)
            }
            FloatscreenWMError::NotATileWindow(ref window) => {
                write!(f, "Not a Tile window: {}", window)
            }
            FloatscreenWMError::NotAFloatWindow(ref window) => {
                write!(f, "Not a Float window: {}", window)
            }
            FloatscreenWMError::ManagedWindow(ref window) => {
                write!(f, "Managed window: {}", window)
            }
        }
    }
}

/// Impliment of error::Error for Tile Screen Eror
impl error::Error for FloatscreenWMError {
    fn description(&self) -> &'static str {
        match *self {
            FloatscreenWMError::UnknownWindow(_) => "Unknown window",
            FloatscreenWMError::NotATileWindow(_) => "Not a Tile window",
            FloatscreenWMError::NotAFloatWindow(_) => "Not a Float window",
            FloatscreenWMError::ManagedWindow(_) => "Managed window",

        }
    }
}


impl WindowManager for FloatscreenWM {
    /// use `FloatscreenWMError` as  `Error` type.
    type Error = FloatscreenWMError;

    /// initiate a new windows manager
    fn new(screen: Screen) -> FloatscreenWM {
        FloatscreenWM {
            windows: Vec::new(),
            focused_window: None,
            full_screen_windows: Vec::new(),
            float_windows: Vec::new(),
            tile_windows: Vec::new(),
            screen: screen,
            original_windows: Vec::new(),
            minimised_windows: Vec::new(),
            gap: 0,
        }
    }

    /// get all windows which are managed by windows manager
    fn get_windows(&self) -> Vec<Window> {
        self.windows.clone()
    }

    /// get current focused window ,if no window is foucsed ,the function returns None
    fn get_focused_window(&self) -> Option<Window> {
        self.focused_window
    }

    /// add a new window to the windows manager.
    fn add_window(&mut self, window_with_info: WindowWithInfo) -> Result<(), Self::Error> {
        if !self.is_managed(window_with_info.window) {
            self.windows.push(window_with_info.window);
            self.original_windows.push((window_with_info.window, window_with_info.geometry));

            // check whether the new screen should be displayed full screen

            if window_with_info.fullscreen == true {
                // get current focused window
                let t_f = self.focused_window;
                self.full_screen_windows.push((window_with_info.window,
                                               window_with_info.geometry,
                                               window_with_info.float_or_tile,
                                               t_f));
                self.focused_window = Some(window_with_info.window);
                Ok(())
            } else {
                if window_with_info.float_or_tile == FloatOrTile::Float {
                    self.float_windows.push((window_with_info.window, window_with_info.geometry));
                    // When the window added ,it should be the focused one
                    self.focused_window = Some(window_with_info.window);
                    Ok(())
                } else {

                    self.tile_windows.push(window_with_info.window);
                    // When the window added ,it should be the focused one
                    self.focused_window = Some(window_with_info.window);
                    Ok(())
                }
            }
        } else {
            Err(FloatscreenWMError::ManagedWindow(window_with_info.window))
        }

    }


    /// remove the window from the window manager
    /// if the window is the focused one ,then set the focused window None

    fn remove_window(&mut self, window: Window) -> Result<(), Self::Error> {
        match self.windows.iter().position(|w| *w == window) {
            None => Err(FloatscreenWMError::UnknownWindow(window)),
            Some(i) => {
                self.windows.remove(i);
                self.original_windows.remove(i);
                if self.focused_window == Some(window) {
                    self.focused_window = None;
                }
                // if the window is minimised now
                if self.minimised_windows
                    .clone()
                    .iter()
                    .map(|w| w.0)
                    .collect::<Vec<_>>()
                    .contains(&window) {
                    let index = self.minimised_windows
                        .clone()
                        .iter()
                        .map(|w| w.0)
                        .collect::<Vec<_>>()
                        .iter()
                        .position(|w| *w == window)
                        .unwrap();
                    self.minimised_windows.remove(index);
                    Ok(())

                } else {
                    // if the window is tile
                    if self.tile_windows.contains(&window) {
                        let index = self.tile_windows.iter().position(|w| *w == window).unwrap();
                        self.tile_windows.remove(index);
                        Ok(())
                    } else {
                        // if the window is full screen
                        if self.full_screen_windows
                            .clone()
                            .iter()
                            .map(|w| w.0)
                            .collect::<Vec<_>>()
                            .contains(&window) {
                            let index = self.full_screen_windows
                                .clone()
                                .iter()
                                .map(|w| w.0)
                                .collect::<Vec<_>>()
                                .iter()
                                .position(|w| *w == window)
                                .unwrap();
                            self.full_screen_windows.remove(index);
                            Ok(())
                        } else {
                            // if the window is float
                            let index = self.float_windows
                                .clone()
                                .iter()
                                .map(|w| w.0)
                                .collect::<Vec<_>>()
                                .iter()
                                .position(|w| *w == window)
                                .unwrap();
                            self.float_windows.remove(index);
                            Ok(())
                        }

                    }
                }
            }
        }
    }


    /// get the layout of windows which are managed
    fn get_window_layout(&self) -> WindowLayout {
        let fullscreen_geometry = self.screen.to_geometry();


        // get the gap
        let gap = self.get_gap();

        // If there is no full screen window, then we do it as usual

        if self.full_screen_windows.is_empty() == true {
            // First deal with tile windows
            match self.tile_windows.len() {
                // If there is no windows, return an empty WindowLayout
                0 => {
                    // if float window is also empty
                    if self.float_windows.is_empty() == true {
                        WindowLayout::new()
                    } else {
                        let mut tempwindows = Vec::new();
                        for i in &self.float_windows {
                            tempwindows.push(*i);
                        }
                        WindowLayout {
                            // the focus window is fucosed
                            focused_window: self.focused_window,

                            windows: tempwindows,
                        }
                    }

                }

                // If there is only one window, the one should be the focused on and take the whole screen:)
                1 => {

                    let mut tempwindows = vec![(self.tile_windows.first().map(|w| *w).unwrap(),
                                                Geometry {
                                                   x: gap as i32,
                                                   y: gap as i32,
                                                   width: fullscreen_geometry.width - gap * 2,
                                                   height: fullscreen_geometry.height - gap * 2,
                                               })];
                    for i in &self.float_windows {
                        tempwindows.push(*i);
                    }
                    WindowLayout {
                        // the focus window is fucosed
                        focused_window: self.focused_window,

                        windows: tempwindows,
                    }

                }
                2 => {
                    let mut tempwindows = Vec::new();
                    let mut count = 1;
                    for i in &self.tile_windows {
                        let cal = (count - 1) * (self.screen.width - gap * 3) / 2 +
                                  (count - 1) * gap + gap;
                        tempwindows.push((*i,
                                          Geometry {
                            x: cal as i32,
                            y: gap as i32,
                            width: (self.screen.width - gap * 3) / 2,
                            height: self.screen.height - gap * 2,
                        }));
                        count = count + 1;
                    }
                    for i in &self.float_windows {
                        tempwindows.push(*i);
                    }

                    WindowLayout {
                        focused_window: self.focused_window,
                        windows: tempwindows,
                    }
                }
                _ => {
                    let num = self.tile_windows.len() as u32;
                    let mut tempwindows = Vec::new();
                    let mut count = 1;
                    for i in &self.tile_windows {
                        let cal_w = (self.screen.width - gap * 3) / 2 + 2 * gap;
                        let cal_h = self.screen.height -
                                    (((self.screen.height - (gap * num)) / (num - 1)) + gap) *
                                    (count - 1);
                        if count > 1 {
                            tempwindows.push((*i,
                                              Geometry {
                                x: cal_w as i32,
                                y: cal_h as i32,
                                width: (self.screen.width - gap * 3) / 2,
                                height: (self.screen.height - gap - gap) / (num - 1),
                            }));
                        } else {
                            tempwindows.push((*i,
                                              Geometry {
                                x: gap as i32,
                                y: gap as i32,
                                width: (self.screen.width - gap * 3) / 2,
                                height: self.screen.height - gap * 2,
                            }));
                        }
                        count = count + 1;
                    }
                    for i in &self.float_windows {
                        tempwindows.push(*i);
                    }
                    WindowLayout {
                        focused_window: self.focused_window,
                        windows: tempwindows,
                    }
                }
            }

        } else {
            WindowLayout {
                // the focus window is fucosed
                focused_window: self.focused_window,

                windows: vec![(self.full_screen_windows.last().unwrap().0,
                               self.get_screen().to_geometry())],
            }
        }




    }



    /// set a new fouced window
    /// if the new focused window is in the float windows list,put it at the end of the float window vector
    fn focus_window(&mut self, window: Option<Window>) -> Result<(), Self::Error> {

        match window {
            Some(i) => {
                match self.windows.iter().position(|w| *w == i) {
                    None => Err(FloatscreenWMError::UnknownWindow(i)),
                    Some(w) => {
                        // if new focused window is in the float windows list
                        // put it to the end of the stack
                        if self.tile_windows.contains(&window.unwrap()) {
                            self.focused_window = window;
                        } else {
                            let index = self.float_windows
                                .clone()
                                .iter()
                                .map(|w| w.0)
                                .collect::<Vec<_>>()
                                .iter()
                                .position(|w| *w == window.unwrap())
                                .unwrap();
                            let temp = self.float_windows[index];
                            self.float_windows.remove(index);
                            self.float_windows.push(temp);
                            self.focused_window = window;
                        }
                        Ok(())

                    }
                }

            }
            None => {
                self.focused_window = window;
                Ok(())
            }
        }
    }


    /// cycle focus the window ,when this is no window focused right now ,focus a random one
    fn cycle_focus(&mut self, dir: PrevOrNext) {
        if self.windows.is_empty() == true {
            return ();
        } else {
            if self.get_focused_window() == None {
                // focuse the last in the vector,not random one
                let temp_prev = self.windows.last().map(|i| *i);
                self.focus_window(temp_prev).unwrap();
            } else {
                match dir {
                    PrevOrNext::Prev => {
                        // get the length of the vec
                        let index = self.windows
                            .iter()
                            .position(|&w| w == self.focused_window.unwrap())
                            .unwrap();
                        if index == 0 {
                            let temp_prev = self.windows.last().map(|i| *i);
                            self.focus_window(temp_prev).unwrap();
                        } else {
                            let temp_prev = self.windows.get(index - 1).map(|i| *i);
                            self.focus_window(temp_prev).unwrap();
                        }
                    }
                    PrevOrNext::Next => {
                        let index = self.windows
                            .iter()
                            .position(|&w| w == self.focused_window.unwrap())
                            .unwrap();
                        let len = self.windows.len();

                        if index == len - 1 {
                            let temp_prev = self.windows.first().map(|i| *i);
                            self.focus_window(temp_prev).unwrap();
                        } else {
                            let temp_prev = self.windows.get(index + 1).map(|i| *i);
                            self.focus_window(temp_prev).unwrap();
                        }

                    }
                }
            }
        }

    }


    /// get the information of the window that is provided .
    fn get_window_info(&self, window: Window) -> Result<WindowWithInfo, Self::Error> {

        match self.windows.iter().position(|w| *w == window) {
            None => Err(FloatscreenWMError::UnknownWindow(window)),
            Some(i) => {

                let mut temp_geometry = Geometry {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 0,
                };
                // locate the geometry for the window we want.
                for w in self.get_window_layout().windows {
                    if w.0 == window {
                        temp_geometry = w.1;
                        break;
                    } else {
                        continue;
                    }
                }
                let temp_tile_or_float: FloatOrTile;
                if self.tile_windows.contains(&window) {
                    temp_tile_or_float = FloatOrTile::Tile;
                } else {
                    temp_tile_or_float = FloatOrTile::Float;
                }

                let temp_full_screen: bool;
                if temp_geometry == self.get_screen().to_geometry() {

                    temp_full_screen = true;
                } else {
                    temp_full_screen = false;
                }
                Ok(WindowWithInfo {
                    window: window,
                    geometry: temp_geometry,
                    float_or_tile: temp_tile_or_float,
                    fullscreen: temp_full_screen,
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

/// Implementation of TilingSupport
impl TilingSupport for FloatscreenWM {
    /// get the master window of current windows manager
    /// The first one in the vector is the master window
    fn get_master_window(&self) -> Option<Window> {
        // The first one in the vector is the master window
        self.tile_windows.first().map(|w| *w)

    }

    /// swap the provided window with current master tile window
    /// if the user asks a float window swap with the master of tile ,
    /// we make the master tile window float and make the float window becomes the master tile one
    fn swap_with_master(&mut self, window: Window) -> Result<(), Self::Error> {
        match self.tile_windows.iter().position(|w| *w == window) {
            None => {
                if self.float_windows
                    .clone()
                    .iter()
                    .map(|w| w.0)
                    .collect::<Vec<_>>()
                    .contains(&window) {
                    // get current window in the master tile
                    let temp_prev = self.tile_windows.first().map(|i| *i).unwrap();
                    // make it float
                    self.toggle_floating(temp_prev).unwrap();
                    // put the window to the master tile

                    self.tile_windows.insert(0, window);
                    // make the master one focused.
                    let temp_prev = self.tile_windows.first().map(|i| *i);
                    self.focus_window(temp_prev).unwrap();
                    Ok(())
                } else {
                    // otherwise return an error ,
                    Err(FloatscreenWMError::UnknownWindow(window))
                }

            }
            Some(i) => {
                // if the windows is in the master tile now
                // then it should be focused.
                if i == 0 {
                    let temp_prev = self.tile_windows.first().map(|i| *i);
                    self.focus_window(temp_prev).unwrap();
                    Ok(())
                } else {
                    // put the window to the front, I mean , put it to the master tile :)
                    // put the window in the master tile to the position .
                    let temp_first = self.tile_windows.first().map(|i| *i).unwrap();
                    self.tile_windows[i] = temp_first;
                    self.tile_windows[0] = window;

                    // and make it focused
                    let temp_prev = self.tile_windows.first().map(|i| *i);
                    self.focus_window(temp_prev).unwrap();
                    Ok(())
                }
            }

        }
    }



    /// Swap the focused window with the one in the next or previous tile.
    /// if current focused is a tile window
    /// The following will happen
    fn swap_windows(&mut self, dir: PrevOrNext) {
        if self.focused_window == None {
            return ();
        } else {
            // If current focused one is a tile window
            if self.tile_windows.contains(&self.focused_window.unwrap()) == true {

                match dir {
                    PrevOrNext::Prev => {
                        match self.tile_windows.len() {

                            0 | 1 => {
                                return ();
                            }
                            _ => {
                                // get index of current focused window
                                let index = self.tile_windows
                                    .iter()
                                    .position(|&w| w == self.focused_window.unwrap())
                                    .unwrap();
                                match index {
                                    // if current focused window is in the master tile
                                    0 => {
                                        // get the current focused window
                                        let temp_focused = self.focused_window.unwrap();
                                        // get the one need to be swaped with it
                                        let temp_last =
                                            self.tile_windows.last().map(|i| *i).unwrap();
                                        let len = self.tile_windows.len();
                                        // swap two windows
                                        self.tile_windows[0] = temp_last;
                                        self.tile_windows[len - 1] = temp_focused;

                                    }

                                    _ => {
                                        // get the current focused window
                                        let temp_focused = self.tile_windows[index];
                                        // get the one need to be swaped with it
                                        let temp_swap = self.tile_windows[index - 1];
                                        let len = self.tile_windows.len();
                                        // swap two windows
                                        self.tile_windows[index] = temp_swap;
                                        self.tile_windows[index - 1] = temp_focused;
                                    }

                                }
                            }

                        }
                    }
                    PrevOrNext::Next => {
                        match self.tile_windows.len() {

                            0 | 1 => {
                                return ();
                            }
                            _ => {
                                // get index of current focused window
                                let index = self.tile_windows
                                    .iter()
                                    .position(|&w| w == self.focused_window.unwrap())
                                    .unwrap();
                                let len = self.tile_windows.len();
                                // if current focused window at the last of windows vector
                                if index == len - 1 {
                                    // get the current focused window
                                    let temp_focused = self.focused_window.unwrap();
                                    // get the one need to be swaped with it
                                    let temp_first = self.tile_windows.first().map(|i| *i).unwrap();
                                    let len = self.tile_windows.len();
                                    // swap two windows
                                    self.tile_windows[0] = temp_focused;
                                    self.tile_windows[len - 1] = temp_first;

                                } else {
                                    // get the current focused window
                                    let temp_focused = self.tile_windows[index];
                                    // get the one need to be swaped with it
                                    let temp_swap = self.tile_windows[index + 1];
                                    let len = self.tile_windows.len();
                                    // swap two windows
                                    self.tile_windows[index] = temp_swap;
                                    self.tile_windows[index + 1] = temp_focused;
                                }


                            }

                        }

                    }
                }

            } else {
                return ();
            }
        }
    }
}


/// Implementation of TilingSupport
impl FloatSupport for FloatscreenWM {
    /// get the windows are managed which is tiling window
    fn get_floating_windows(&self) -> Vec<Window> {

        let mut tempwindows = vec![];
        for i in &self.float_windows {
            tempwindows.push(i.0);
        }
        return tempwindows;
    }

    /// check the window is float or not
    fn is_floating(&self, window: Window) -> bool {
        self.get_floating_windows().contains(&window)
    }

    /// toggle the window, the given window is floating, let it *sink*, if it is not floating,
    fn toggle_floating(&mut self, window: Window) -> Result<(), Self::Error> {
        match self.windows.iter().position(|w| *w == window) {
            None => Err(FloatscreenWMError::UnknownWindow(window)),
            Some(i) => {
                if self.tile_windows.contains(&window) {
                    let index = self.tile_windows.iter().position(|w| *w == window).unwrap();
                    self.tile_windows.remove(index);
                    let index2 = self.windows.iter().position(|w| *w == window).unwrap();
                    // When a non-floating window starts to float, its original geometry
                    // (passed to `add_window`) should be restored.
                    let temp_geometry = self.original_windows[index2].1;
                    self.float_windows.push((window, temp_geometry));
                    Ok(())
                } else {
                    let index = self.float_windows
                        .clone()
                        .iter()
                        .map(|w| w.0)
                        .collect::<Vec<_>>()
                        .iter()
                        .position(|w| *w == window)
                        .unwrap();
                    self.float_windows.remove(index);
                    self.tile_windows.push(window);
                    Ok(())
                }

            }
        }
    }

    /// Resize/move the given floating window according to the given geometry.
    fn set_window_geometry(&mut self,
                           window: Window,
                           new_geometry: Geometry)
                           -> Result<(), Self::Error> {
        match self.windows.iter().position(|w| *w == window) {
            None => Err(FloatscreenWMError::UnknownWindow(window)),
            Some(i) => {
                if self.float_windows
                    .clone()
                    .iter()
                    .map(|w| w.0)
                    .collect::<Vec<_>>()
                    .contains(&window) {
                    let index = self.float_windows
                        .clone()
                        .iter()
                        .map(|w| w.0)
                        .collect::<Vec<_>>()
                        .iter()
                        .position(|w| *w == window)
                        .unwrap();
                    self.float_windows[index].1 = new_geometry;
                    Ok(())
                } else {
                    Err(FloatscreenWMError::NotAFloatWindow(window))
                }
            }
        }
    }
}

/// Implementation of MinimiseSupport
impl MinimiseSupport for FloatscreenWM {
    /// Return a vector of all the minimised windows.
    fn get_minimised_windows(&self) -> Vec<Window> {
        let mut tempwindows = vec![];
        for i in &self.minimised_windows {
            tempwindows.push(i.0);
        }
        return tempwindows;
    }

    /// Return `true` if the given window is minimised.
    fn is_minimised(&self, window: Window) -> bool {
        self.get_minimised_windows().contains(&window)
    }

    /// Minimise the given window, or when it is already minimised, unminise
    /// it.
    fn toggle_minimised(&mut self, window: Window) -> Result<(), Self::Error> {
        if self.is_minimised(window) == false {
            match self.windows.iter().position(|w| *w == window) {
                None => Err(FloatscreenWMError::UnknownWindow(window)),
                Some(i) => {
                    if self.full_screen_windows
                        .clone()
                        .iter()
                        .map(|w| w.0)
                        .collect::<Vec<_>>()
                        .contains(&window) {
                        let index = self.full_screen_windows
                            .clone()
                            .iter()
                            .map(|w| w.0)
                            .collect::<Vec<_>>()
                            .iter()
                            .position(|w| *w == window)
                            .unwrap();
                        let temp_geo = self.get_window_info(window).unwrap().geometry;
                        self.minimised_windows.push((window, temp_geo, FloatOrTile::Tile));
                        self.full_screen_windows.remove(index);
                        Ok(())
                    } else {
                        if self.tile_windows.contains(&window) {
                            let index =
                                self.tile_windows.iter().position(|w| *w == window).unwrap();
                            let temp_geo = self.get_window_info(window).unwrap().geometry;
                            self.minimised_windows.push((window, temp_geo, FloatOrTile::Tile));
                            self.tile_windows.remove(index);
                            Ok(())

                        } else {
                            let index = self.float_windows
                                .clone()
                                .iter()
                                .map(|w| w.0)
                                .collect::<Vec<_>>()
                                .iter()
                                .position(|w| *w == window)
                                .unwrap();
                            let temp_geo = self.get_window_info(window).unwrap().geometry;
                            self.minimised_windows.push((window, temp_geo, FloatOrTile::Float));
                            self.float_windows.remove(index);
                            Ok(())
                        }
                    }
                }
            }
        } else {
            match self.windows.iter().position(|w| *w == window) {
                None => Err(FloatscreenWMError::UnknownWindow(window)),
                Some(i) => {
                    let index = self.minimised_windows
                        .clone()
                        .iter()
                        .map(|w| w.0)
                        .collect::<Vec<_>>()
                        .iter()
                        .position(|w| *w == window)
                        .unwrap();
                    if self.minimised_windows[index].2 == FloatOrTile::Tile {
                        self.minimised_windows.remove(index);
                        self.tile_windows.push(window);
                        Ok(())

                    } else {
                        self.float_windows.push((window, self.minimised_windows[index].1));
                        self.minimised_windows.remove(index);
                        Ok(())
                    }
                }
            }
        }


    }
}

/// Implementation of FullscreenSupport
impl FullscreenSupport for FloatscreenWM {
    /// Return the current fullscreen, if any.
    fn get_fullscreen_window(&self) -> Option<Window> {
        if self.full_screen_windows.is_empty() == true {
            return None;
        } else {
            return Some(self.full_screen_windows.last().unwrap().0);
        }
    }


    /// Make the given window fullscreen, or when it is already fullscreen,
    /// undo it.
    /// when call with a minimised window as argument should first unminimise the window.
    fn toggle_fullscreen(&mut self, window: Window) -> Result<(), Self::Error> {

        // **Note**: methods of other traits like `focus_window`, `focus_window`,
        // `toggle_fullscreen`, ... called with a minimised window as argument should
        // first unminimise the window.
        if self.minimised_windows
            .clone()
            .iter()
            .map(|w| w.0)
            .collect::<Vec<_>>()
            .contains(&window) {
            self.toggle_minimised(window).unwrap();
            Ok(())

            // Err(FloatscreenWMError::AMinimizedWindow(window))
        } else {
            // if the window is not full screen now .
            if self.full_screen_windows
                .clone()
                .iter()
                .map(|w| w.0)
                .collect::<Vec<_>>()
                .contains(&window) == false {
                match self.windows.iter().position(|w| *w == window) {
                    None => Err(FloatscreenWMError::UnknownWindow(window)),
                    Some(i) => {
                        if self.tile_windows.contains(&window) {
                            let index =
                                self.tile_windows.iter().position(|w| *w == window).unwrap();
                            // get the poisiton of window in the tile vector
                            let temp_geo = self.get_window_info(window).unwrap().geometry;
                            // get the geometry of the window
                            self.full_screen_windows
                                .push((window, temp_geo, FloatOrTile::Tile, self.focused_window));
                            // set the foucsed window to new full screen one
                            self.focused_window = Some(window);
                            self.tile_windows.remove(index);
                            Ok(())

                        } else {
                            let index = self.float_windows
                                .clone()
                                .iter()
                                .map(|w| w.0)
                                .collect::<Vec<_>>()
                                .iter()
                                .position(|w| *w == window)
                                .unwrap();
                            // Even though , it is resized , and we close it, we still can get the resized window back.:)
                            let temp_geo = self.float_windows[index].1;
                            self.full_screen_windows
                                .push((window, temp_geo, FloatOrTile::Float, self.focused_window));
                            // set the foucsed window to new full screen one
                            self.focused_window = Some(window);
                            self.float_windows.remove(index);
                            Ok(())
                        }

                    }
                }
            } else {
                match self.windows.iter().position(|w| *w == window) {
                    None => Err(FloatscreenWMError::UnknownWindow(window)),
                    Some(i) => {
                        let index = self.full_screen_windows
                            .clone()
                            .iter()
                            .map(|w| w.0)
                            .collect::<Vec<_>>()
                            .iter()
                            .position(|w| *w == window)
                            .unwrap();
                        if self.full_screen_windows[index].2 == FloatOrTile::Tile {
                            let temp_full_screen = self.full_screen_windows[index].3;
                            self.full_screen_windows.remove(index);
                            self.tile_windows.push(window);
                            self.focused_window = temp_full_screen;
                            Ok(())

                        } else {
                            let temp_full_screen = self.full_screen_windows[index].3;
                            self.float_windows.push((window, self.full_screen_windows[index].1));
                            self.focused_window = temp_full_screen;
                            self.full_screen_windows.remove(index);
                            Ok(())
                        }
                    }
                }
            }
        }
    }
}

impl GapSupport for FloatscreenWM {
    /// Return the current gap size.
    fn get_gap(&self) -> GapSize {
        self.gap
    }

    /// Set the gap size.
    fn set_gap(&mut self, g: GapSize) {
        self.gap = g
    }
}


impl<WM: WindowManager> MultiWorkspaceSupport<WM> for FloatscreenWM {
    /// Return the current workspace index.
    ///
    /// When creating a new workspace this will return 0.
    ///
    /// **Invariant**: `0 <= get_current_workspace_index() <=
    /// MAX_WORKSPACE_INDEX`.
    fn get_current_workspace_index(&self) -> WorkspaceIndex {
        unimplemented!()
    }

    /// Get an immutable borrow of the workspace at the given index.
    ///
    /// This function *should* return an appropriate error when `0 <= index <=
    /// MAX_WORKSPACE_INDEX` is not true.
    fn get_workspace(&self, index: WorkspaceIndex) -> Result<&WM, Self::Error> {
        unimplemented!()
    }

    /// Get a mutable borrow of the workspace at the given index.
    ///
    /// This function *should* return an appropriate error when `0 <= index <=
    /// MAX_WORKSPACE_INDEX` is not true.
    fn get_workspace_mut(&mut self, index: WorkspaceIndex) -> Result<&mut WM, Self::Error> {
        unimplemented!()
    }
    /// Switch to the workspace at the given index.
    ///
    /// If `index == get_current_workspace_index()`, do nothing.
    ///
    /// **Invariant**: the window layout after switching to another workspace
    /// and then switching back to the original workspace should be the same
    /// as before.
    ///
    /// This function *should* return an appropriate error when `0 <= index <=
    /// MAX_WORKSPACE_INDEX` is not true.
    fn switch_workspace(&mut self, index: WorkspaceIndex) -> Result<(), Self::Error> {
        unimplemented!()
    }
}
#[cfg(test)]
mod tests {

    use super::FloatscreenWM;

    // Repeat the imports we did in the super module.
    use cplwm_api::wm::WindowManager;
    use cplwm_api::wm::GapSupport;
    use cplwm_api::wm::FloatSupport;
    use cplwm_api::types::*;

    // Static value

    static SCREEN: Screen = Screen {
        width: 800,
        height: 600,
    };


    static SOME_GEOM: Geometry = Geometry {
        x: 10,
        y: 10,
        width: 100,
        height: 100,
    };
    #[test]
    fn test_windows_sample() {
        // windows = []
        // add 1 as a floating window
        let mut wm = FloatscreenWM::new(SCREEN);
        wm.add_window(WindowWithInfo::new_float(1, SOME_GEOM)).unwrap();
        // add 2 as a tiled window
        wm.add_window(WindowWithInfo::new_tiled(2, SOME_GEOM)).unwrap();
        // add 3 as a floating window
        wm.add_window(WindowWithInfo::new_float(3, SOME_GEOM)).unwrap();
        // add 4 as a tiled window
        wm.add_window(WindowWithInfo::new_tiled(4, SOME_GEOM)).unwrap();
        // add 5 as a floating window
        wm.add_window(WindowWithInfo::new_float(5, SOME_GEOM)).unwrap();
        // add 6 as a tiled window
        wm.add_window(WindowWithInfo::new_tiled(6, SOME_GEOM)).unwrap();

        // toggle_floating(3)
        wm.toggle_floating(3).unwrap();
        // toggle_floating(6)
        wm.toggle_floating(6).unwrap();
        // toggle_floating(1)
        wm.toggle_floating(1).unwrap();
        // focus_window(Some(5))
        wm.focus_window(Some(5)).unwrap();

        let wl2 = wm.get_window_layout();
        assert_eq!(vec![2, 4, 3, 1, 6, 5],
                   wl2.windows
                       .clone()
                       .iter()
                       .map(|w| w.0)
                       .collect::<Vec<_>>());

        // So I have:
        // [(2, master_geometry),
        // (4, slave_geometry),
        // (3, slave_geometry),
        // (1, slave_geometry),
        // (6, float_geometry),
        // (5, float_geometry)]



    }

    #[test]
    fn test_windows_layout() {

        // when the gap is zeor
        let mut wm = FloatscreenWM::new(SCREEN);
        wm.add_window(WindowWithInfo::new_tiled(1, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(2, SOME_GEOM)).unwrap();
        wm.add_window(WindowWithInfo::new_tiled(3, SOME_GEOM)).unwrap();

        let wl1 = wm.get_window_layout();
        assert_eq!(vec![(1,
                         Geometry {
                            x: 0,
                            y: 0,
                            width: 400,
                            height: 600,
                        }),
                        (2,
                         Geometry {
                            x: 400,
                            y: 300,
                            width: 400,
                            height: 300,
                        }),
                        (3,
                         Geometry {
                            x: 400,
                            y: 0,
                            width: 400,
                            height: 300,
                        })],
                   wl1.windows);

        wm.add_window(WindowWithInfo::new_float(4, SOME_GEOM)).unwrap();

        let wl2 = wm.get_window_layout();
        assert_eq!(vec![(1,
                         Geometry {
                            x: 0,
                            y: 0,
                            width: 400,
                            height: 600,
                        }),
                        (2,
                         Geometry {
                            x: 400,
                            y: 300,
                            width: 400,
                            height: 300,
                        }),
                        (3,
                         Geometry {
                            x: 400,
                            y: 0,
                            width: 400,
                            height: 300,
                        }),
                        (4, SOME_GEOM)],
                   wl2.windows);

        // set the gap 5
        wm.set_gap(10);
        // get the gap
        assert_eq!(10, wm.get_gap());
        // test lay out
        let wl2 = wm.get_window_layout();
        assert_eq!(vec![(1,
                         Geometry {
                            x: 10,
                            y: 10,
                            width: 385,
                            height: 580,
                        }),
                        (2,
                         Geometry {
                            x: 405,
                            y: 305,
                            width: 385,
                            height: 290,
                        }),
                        (3,
                         Geometry {
                            x: 405,
                            y: 10,
                            width: 385,
                            height: 290,
                        }),
                        (4, SOME_GEOM)],
                   wl2.windows);

        wm.remove_window(2).unwrap();
        let wl2 = wm.get_window_layout();
        assert_eq!(vec![(1,
                         Geometry {
                            x: 10,
                            y: 10,
                            width: 385,
                            height: 580,
                        }),
                        (3,
                         Geometry {
                            x: 405,
                            y: 10,
                            width: 385,
                            height: 580,
                        }),
                        (4, SOME_GEOM)],
                   wl2.windows);

        wm.remove_window(3).unwrap();
        let wl2 = wm.get_window_layout();
        assert_eq!(vec![(1,
                         Geometry {
                            x: 10,
                            y: 10,
                            width: 780,
                            height: 580,
                        }),
                        (4, SOME_GEOM)],
                   wl2.windows);
    }



}
