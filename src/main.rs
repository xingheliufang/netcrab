use arg_parser::{
    Cli, ConnectCli, ListenCli,
    Mode::{Connect, Listen},
};
use clap::Parser;
use tokio::{
    io::{AsyncWriteExt, AsyncRead, AsyncWrite},
    net::{TcpListener, TcpStream},
};
use tracing::debug;

mod arg_parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.mode {
        Connect(c) => {
            debug!("connect mode, {:?}", c);
            connect_cycle(c).await?;
        }
        Listen(l) => {
            debug!("listen mode, {:?}", l);
            listen_cycle(l).await?;
        }
    }

    std::process::exit(0)
}

async fn listen_cycle(l: ListenCli) -> anyhow::Result<()> {
    let listener = TcpListener::bind(l.addr).await?;

    let (mut socket, addr) = listener.accept().await?;
    debug!("{:?}", addr);

    let (mut si, mut so) = socket.split();

    if l.cmd.len() != 0 {
        let mut child = tokio::process::Command::new(l.cmd[0].clone())
            .args(l.cmd.iter().skip(1))
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        let (mut stdin, mut stdout) = (child.stdout.take().unwrap(), child.stdin.take().unwrap());
        copy(&mut si, &mut so, &mut stdin, &mut stdout).await;
    } else {
        let (mut stdin, mut stdout) = (tokio::io::stdin(), tokio::io::stdout());
        copy(&mut si, &mut so, &mut stdin, &mut stdout).await;
    };

    so.shutdown().await?;

    Ok(())
}

async fn connect_cycle(c: ConnectCli) -> anyhow::Result<()> {
    let mut socket = TcpStream::connect(c.addr).await?;

    let (mut si, mut so) = socket.split();
    let (mut stdin, mut stdout) = (tokio::io::stdin(), tokio::io::stdout());

    copy(&mut si, &mut so, &mut stdin, &mut stdout).await;

    Ok(())
}

async fn copy(
    ir: &mut (impl AsyncRead + Unpin + ?Sized),
    iw: &mut (impl AsyncWrite + Unpin + ?Sized),
    or: &mut (impl AsyncRead + Unpin + ?Sized),
    ow: &mut (impl AsyncWrite + Unpin + ?Sized),
) {
    loop {
        tokio::select! {
            _ = tokio::io::copy(ir, ow) => {
                break;
            }

            _ = tokio::io::copy(or, iw) => {
                break;
            }
        }
    }
}