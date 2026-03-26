use cosmic::widget::menu::key_bind::{KeyBind, Modifier};
use cosmic::{iced::keyboard::Key, iced_core::keyboard::key::Named};
use std::collections::HashMap;

use crate::Action;

//TODO: load from config
pub fn key_binds() -> HashMap<KeyBind, Action> {
    let mut key_binds = HashMap::new();

    macro_rules! bind {
        ([$($modifier:expr),* $(,)?], $key:expr, $action:ident) => {{
            key_binds.insert(
                KeyBind {
                    modifiers: vec![$($modifier),*],
                    key: $key,
                },
                Action::$action,
            );
        }};
    }

    // Standard key bindings
    #[cfg(target_os = "macos")]
    {
        bind!([Modifier::Super], Key::Character("a".into()), SelectAll);
        bind!([Modifier::Super], Key::Character("c".into()), Copy);
        bind!([Modifier::Super], Key::Character("f".into()), Find);
        bind!([Modifier::Super], Key::Character("n".into()), WindowNew);
        bind!([Modifier::Super], Key::Character("q".into()), WindowClose);
        bind!([Modifier::Super], Key::Character("t".into()), TabNew);
        bind!([Modifier::Super], Key::Character("v".into()), Paste);
        bind!([Modifier::Super], Key::Character("w".into()), TabClose);
        bind!([Modifier::Super], Key::Character(",".into()), Settings);
        bind!([Modifier::Super], Key::Character("k".into()), ClearScrollback);
    }

    #[cfg(not(target_os = "macos"))]
    {
        bind!([Modifier::Ctrl, Modifier::Shift], Key::Character("A".into()), SelectAll);
        bind!([Modifier::Ctrl, Modifier::Shift], Key::Character("C".into()), Copy);
        bind!([Modifier::Ctrl, Modifier::Shift], Key::Character("F".into()), Find);
        bind!([Modifier::Ctrl, Modifier::Shift], Key::Character("N".into()), WindowNew);
        bind!([Modifier::Ctrl, Modifier::Shift], Key::Character("Q".into()), WindowClose);
        bind!([Modifier::Ctrl, Modifier::Shift], Key::Character("T".into()), TabNew);
        bind!([Modifier::Ctrl, Modifier::Shift], Key::Character("V".into()), Paste);
        bind!([Modifier::Ctrl, Modifier::Shift], Key::Character("W".into()), TabClose);
        bind!([Modifier::Ctrl], Key::Character(",".into()), Settings);
    }
    
    bind!([], Key::Named(Named::Copy), Copy);
    bind!([Modifier::Ctrl], Key::Character("c".into()), CopyOrSigint);
    bind!([], Key::Named(Named::Paste), Paste);
    bind!([Modifier::Shift], Key::Named(Named::Insert), PastePrimary);
    bind!([], Key::Named(Named::F11), ToggleFullscreen);

    // Splits
    #[cfg(target_os = "macos")]
    {
        bind!([Modifier::Super, Modifier::Alt], Key::Character("d".into()), PaneSplitHorizontal);
        bind!([Modifier::Super, Modifier::Alt], Key::Character("r".into()), PaneSplitVertical);
        bind!([Modifier::Super, Modifier::Shift], Key::Character("x".into()), PaneToggleMaximized);
    }
    #[cfg(not(target_os = "macos"))]
    {
        bind!([Modifier::Ctrl, Modifier::Alt], Key::Character("d".into()), PaneSplitHorizontal);
        bind!([Modifier::Ctrl, Modifier::Alt], Key::Character("r".into()), PaneSplitVertical);
        bind!([Modifier::Ctrl, Modifier::Shift], Key::Character("X".into()), PaneToggleMaximized);
    }
    #[cfg(feature = "password_manager")]
    bind!([Ctrl, Alt], Key::Character("p".into()), PasswordManager);

    // Tabs navigation
    #[cfg(target_os = "macos")]
    {
        bind!([Modifier::Super], Key::Named(Named::Tab), TabNext);
        bind!([Modifier::Super, Modifier::Shift], Key::Named(Named::Tab), TabPrev);
    }
    #[cfg(not(target_os = "macos"))]
    {
        bind!([Modifier::Ctrl], Key::Named(Named::Tab), TabNext);
        bind!([Modifier::Ctrl, Modifier::Shift], Key::Named(Named::Tab), TabPrev);
    }

    // Tabs activation
    #[cfg(target_os = "macos")]
    let main_mod = Modifier::Super;
    #[cfg(not(target_os = "macos"))]
    let main_mod = Modifier::Ctrl;

    bind!([main_mod], Key::Character("1".into()), TabActivate0);
    bind!([main_mod], Key::Character("2".into()), TabActivate1);
    bind!([main_mod], Key::Character("3".into()), TabActivate2);
    bind!([main_mod], Key::Character("4".into()), TabActivate3);
    bind!([main_mod], Key::Character("5".into()), TabActivate4);
    bind!([main_mod], Key::Character("6".into()), TabActivate5);
    bind!([main_mod], Key::Character("7".into()), TabActivate6);
    bind!([main_mod], Key::Character("8".into()), TabActivate7);
    bind!([main_mod], Key::Character("9".into()), TabActivate8);

    // Zoom
    bind!([main_mod], Key::Character("0".into()), ZoomReset);
    bind!([main_mod], Key::Character("-".into()), ZoomOut);
    bind!([main_mod], Key::Character("=".into()), ZoomIn);
    bind!([main_mod], Key::Character("+".into()), ZoomIn);

    // Ctrl+Arrows and Ctrl+HJKL move between splits
    bind!([Modifier::Ctrl, Modifier::Shift], Key::Named(Named::ArrowLeft), PaneFocusLeft);
    bind!([Modifier::Ctrl, Modifier::Shift], Key::Character("H".into()), PaneFocusLeft);
    bind!([Modifier::Ctrl, Modifier::Shift], Key::Named(Named::ArrowDown), PaneFocusDown);
    bind!([Modifier::Ctrl, Modifier::Shift], Key::Character("J".into()), PaneFocusDown);
    bind!([Modifier::Ctrl, Modifier::Shift], Key::Named(Named::ArrowUp), PaneFocusUp);
    bind!([Modifier::Ctrl, Modifier::Shift], Key::Character("K".into()), PaneFocusUp);
    bind!([Modifier::Ctrl, Modifier::Shift], Key::Named(Named::ArrowRight), PaneFocusRight);
    bind!([Modifier::Ctrl, Modifier::Shift], Key::Character("L".into()), PaneFocusRight);

    // CTRL+Alt+L clears the scrollback.
    bind!([Modifier::Ctrl, Modifier::Alt], Key::Character("L".into()), ClearScrollback);

    key_binds
}
