use crate::packet::Packet;

pub trait RTDStateDataSource: Send {
    type Error;

    fn read_packet(&mut self) -> Option<Result<Packet, Self::Error>>;

    #[cfg(feature = "async")]
    fn read_packet_async(
        &mut self,
    ) -> impl std::future::Future<Output = Option<Result<Packet, Self::Error>>> + Send;
}

pub use serial_stream_data_source::SerialStreamDataSource;

#[cfg(feature = "async")]
mod serial_stream_data_source;
