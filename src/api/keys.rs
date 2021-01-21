use crate::Device;
use crate::Error;

/// Describes the types of actions a key has.
enum KeyEvent {
    /// Pressing a key.
    Down,
    /// Releasing a key.
    Up,
    /// Pressing and releasing a key.
    Press,
}

impl Device {
    /// Sends a key press event for each character in the string
    pub async fn send_string(&self, text: &str) -> Result<(), Error> {
        for key in text.as_bytes() {
            self.key_down(Key::Lit(*key as char)).await?;
        }

        Ok(())
    }

    /// Equivalent to pressing the remote control key.
    pub async fn key_down(&self, key: Key) -> Result<(), Error> {
        self.send_key_cmd(KeyEvent::Down, key).await
    }

    /// Equivalent to releasing the remote control key.
    pub async fn key_up(&self, key: Key) -> Result<(), Error> {
        self.send_key_cmd(KeyEvent::Up, key).await
    }

    /// Equivalent to pressing down and releasing the remote control key.
    pub async fn key_press(&self, key: Key) -> Result<(), Error> {
        self.send_key_cmd(KeyEvent::Press, key).await
    }

    /// Retrieves device information similar to that returned by [roDeviceInfo](https://developer.roku.com/en-gb/docs/references/brightscript/components/rodeviceinfo.md#!) for BrightScript.
    async fn send_key_cmd(&self, event: KeyEvent, key: Key) -> Result<(), Error> {
        let key_event = match event {
            KeyEvent::Up => "keyup",
            KeyEvent::Down => "keydown",
            KeyEvent::Press => "keypress",
        };

        self.http
            .post(self.url.join(&format!("{}/{}", key_event, key))?)
            .recv_string()
            .await?;

        Ok(())
    }
}

// TODO: add docs for keys that aren't obvious.
/// Supported key values for Roku devices.
#[derive(Debug)]
pub enum Key {
    Home,
    Rev,
    Fwd,
    Play,
    Select,
    Left,
    Right,
    Down,
    Up,
    Back,
    InstantReplay,
    Info,
    Backspace,
    Search,
    Enter,
    FindRemote,
    VolumeDown,
    VolumeMute,
    VolumeUp,
    PowerOff,
    ChannelUp,
    ChannelDown,
    InputTuner,
    InputHDMI1,
    InputHDMI2,
    InputHDMI3,
    InputHDMI4,
    InputAV1,
    /// Any printable character.
    /// Officially, Roku only supports valid **UTF-8**.
    Lit(char),
}

use std::fmt;
impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Key::Lit(char) => write!(f, "Lit_{}", &char.to_string()),
            // default is to use the enum variant name as a string
            _ => write!(f, "{:?}", self),
        }
    }
}
