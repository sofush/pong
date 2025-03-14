use tokio::{
    fs::OpenOptions,
    io::{AsyncBufReadExt, BufReader},
    sync::broadcast::Sender,
};

pub(crate) async fn start(tx_clone: Sender<String>) -> anyhow::Result<()> {
    let file = OpenOptions::new().read(true).open("/dev/ttyACM0").await?;
    let mut reader = BufReader::new(file);

    loop {
        let mut line = String::new();
        reader.read_line(&mut line).await?;

        log::debug!("Read new button state from microbit: {line}");
        tx_clone.send(line)?;
    }
}
