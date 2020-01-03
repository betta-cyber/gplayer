use std::io;
use std::time::Duration;
use std::thread;

pub trait Open {
    fn open() -> Self;
}

pub trait Sink {
    fn start(&mut self) -> io::Result<()>;
    fn stop(&mut self) -> io::Result<()>;
    fn write(&mut self, data: &[i16]) -> io::Result<()>;
    fn append(&mut self, path: &str);
}

fn mk_sink<S: Sink + Open + 'static>(device: Option<String>) -> Box<dyn Sink> {
    debug!("{}", device.unwrap());
    Box::new(S::open())
}

pub const BACKENDS: &'static [(&'static str, fn(Option<String>) -> Box<dyn Sink>)] = &[
    ("rodio", mk_sink::<RodioSink>),
];

pub fn find(name: Option<String>) -> Option<fn(Option<String>) -> Box<dyn Sink>> {
    if let Some(name) = name {
        BACKENDS
            .iter()
            .find(|backend| name == backend.0)
            .map(|backend| backend.1)
    } else {
        Some(
            BACKENDS
                .first()
                .expect("No backends were enabled at build time")
                .1,
        )
    }
}

pub struct RodioSink {
    rodio_sink: rodio::Sink,
}

impl Open for RodioSink {
    fn open() -> RodioSink {
        debug!("Using rodio sink");

        let rodio_device = rodio::default_output_device().expect("no output device available");
        let sink = rodio::Sink::new(&rodio_device);

        RodioSink {
            rodio_sink: sink,
        }
    }
}

impl Sink for RodioSink {
    fn start(&mut self) -> io::Result<()> {
        // More similar to an "unpause" than "play". Doesn't undo "stop".
        // self.rodio_sink.play();
        Ok(())
    }

    fn stop(&mut self) -> io::Result<()> {
        // This will immediately stop playback, but the sink is then unusable.
        // We just have to let the current buffer play till the end.
        // self.rodio_sink.stop();
        Ok(())
    }

    fn write(&mut self, data: &[i16]) -> io::Result<()> {
        let source = rodio::buffer::SamplesBuffer::new(2, 44100, data);
        self.rodio_sink.append(source);

        // Chunk sizes seem to be about 256 to 3000 ish items long.
        // Assuming they're on average 1628 then a half second buffer is:
        // 44100 elements --> about 27 chunks
        while self.rodio_sink.len() > 26 {
            // sleep and wait for rodio to drain a bit
            thread::sleep(Duration::from_millis(10));
        }
        Ok(())
    }

    fn append(&mut self, path: &str) {
        let b = std::fs::File::open(&path).unwrap();
        let source = rodio::Decoder::new(
            std::io::BufReader::with_capacity(100, b)
        ).unwrap();

        self.rodio_sink.append(source);
        self.rodio_sink.sleep_until_end();
    }
}
