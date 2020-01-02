use futures;
use futures::channel::oneshot;
use futures::{future, Future};

use std::thread;
use std::time::Duration;

enum PlayerCommand {
    Load(String, oneshot::Sender<()>),
    Play,
    Pause,
    Stop,
    Seek(u32),
}

enum PlayerState {
    Stopped,
    Paused {
        end_of_track: oneshot::Sender<()>,
        normalisation_factor: f32,
        // stream_loader_controller: StreamLoaderController,
        bytes_per_second: usize,
    },
    Playing {
        end_of_track: oneshot::Sender<()>,
        normalisation_factor: f32,
        // stream_loader_controller: StreamLoaderController,
        bytes_per_second: usize,
    },
    EndOfTrack {
        url: String,
    },
    Invalid,
}

pub struct Player {
    commands: Option<std::sync::mpsc::Sender<PlayerCommand>>,
    thread_handle: Option<thread::JoinHandle<()>>,
}

struct PlayerInternal {
    commands: std::sync::mpsc::Receiver<PlayerCommand>,

    state: PlayerState,
    // event_sender: futures::channel::mpsc::UnboundedSender<PlayerEvent>,
}

#[derive(Debug, Clone)]
pub enum PlayerEvent {
    Started {
        track_url: String,
    },

    Changed {
        old_track_url: String,
        new_track_url: String,
    },

    Stopped {
        track_url: String,
    },
}

type PlayerEventChannel = futures::channel::mpsc::UnboundedReceiver<PlayerEvent>;

impl Player {
    pub fn new<F>(
        // audio_filter: Option<Box<AudioFilter + Send>>,
        // sink_builder: F,
    ) -> (Player, PlayerEventChannel)
    where
        // F: FnOnce() -> Box<Sink> + Send + 'static,
    {
        let (cmd_tx, cmd_rx) = std::sync::mpsc::channel();
        let (event_sender, event_receiver) = futures::channel::mpsc::unbounded();

        let handle = thread::spawn(move || {
            // debug!("new Player[{}]", session.session_id());

            let internal = PlayerInternal {
                commands: cmd_rx,

                state: PlayerState::Stopped,
                // sink: sink_builder(),
                // sink_running: false,
                // audio_filter: audio_filter,
                // event_sender: event_sender,
            };

            internal.run();
        });

        (
            Player {
                commands: Some(cmd_tx),
                thread_handle: Some(handle),
            },
            event_receiver,
        )
    }
}

impl PlayerInternal {
    fn run(mut self) {
        loop {
            let cmd = if self.state.is_playing() {
                println!("loop");
            };
        }
    }
}

impl PlayerState {
    fn is_playing(&self) -> bool {
        use self::PlayerState::*;
        match *self {
            Stopped | EndOfTrack { .. } | Paused { .. } => false,
            Playing { .. } => true,
            Invalid => panic!("invalid state"),
        }
    }
}
