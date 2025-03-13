use serde::Deserialize;
use tokio::{
    fs::OpenOptions,
    io::{AsyncBufReadExt, BufReader},
    sync::broadcast::Sender,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct ButtonState {
    left_pressed: bool,
    right_pressed: bool,
}

pub(crate) async fn start(tx_clone: Sender<ButtonState>) -> anyhow::Result<()> {
    let file = OpenOptions::new().read(true).open("/dev/ttyACM0").await?;
    let mut reader = BufReader::new(file);

    loop {
        let mut line = String::new();
        reader.read_line(&mut line).await?;

        let button_state = serde_json::from_str(&line)?;

        log::debug!("Read new button state from microbit: {button_state:?}");
        tx_clone.send(button_state)?;
    }
}
