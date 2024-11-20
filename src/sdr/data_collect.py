import uhd
import numpy as np
import time

def configure_sdr(sdr, center_freq=2.4e9, sample_rate=1e6, gain=30):
    """
    Configures the SDR with the specified parameters.

    :param sdr: UHD USRP source object.
    :param center_freq: Center frequency in Hz (default 2.4 GHz).
    :param sample_rate: Sampling rate in samples per second (default 1 MHz).
    :param gain: RF gain in dB (default 30 dB).
    """
    sdr.set_rx_rate(sample_rate)
    print(f"Sampling rate set to {sample_rate} Hz")
    
    sdr.set_rx_freq(uhd.types.TuneRequest(center_freq))
    print(f"Center frequency set to {center_freq} Hz")
    
    sdr.set_rx_gain(gain)
    print(f"Gain set to {gain} dB")

def receive_iq_data(sdr, num_samples=1024, duration=1.0):
    """
    Receives IQ data from the SDR.

    :param sdr: UHD USRP source object.
    :param num_samples: Number of samples to receive per buffer.
    :param duration: Duration of reception in seconds.
    :return: IQ data as a NumPy array.
    """
    # Create stream arguments with positional arguments for cpu_format and otw_format
    stream_args = uhd.usrp.StreamArgs("fc32", "sc16")
    rx_streamer = sdr.get_rx_stream(stream_args)

    # Allocate a buffer to hold the samples
    samples = np.zeros((num_samples,), dtype=np.complex64)
    metadata = uhd.types.RXMetadata()

    # Configure streaming
    rx_streamer.issue_stream_cmd(
        uhd.types.StreamCMD(uhd.types.StreamMode.start_cont)
    )

    print("Receiving IQ data...")
    iq_data = []

    start_time = time.time()
    while time.time() - start_time < duration:
        num_received = rx_streamer.recv(samples, metadata)
        if metadata.error_code != uhd.types.RXMetadataErrorCode.none:
            print(f"Error receiving data: {metadata.error_code}")
            continue

        # Store received samples
        iq_data.append(samples[:num_received].copy())

    # Stop streaming
    rx_streamer.issue_stream_cmd(
        uhd.types.StreamCMD(uhd.types.StreamMode.stop_cont)
    )

    print("Reception complete.")
    return np.concatenate(iq_data)

def main():
    try:
        # Create a USRP source
        sdr = uhd.usrp.MultiUSRP()

        # Configure the SDR
        configure_sdr(sdr, center_freq=2.4e9, sample_rate=1e6, gain=30)

        # Receive IQ data for 5 seconds
        duration = 5.0
        iq_data = receive_iq_data(sdr, num_samples=1024, duration=duration)

        # Save the IQ data to a file
        np.save("iq_data.npy", iq_data)
        print(f"IQ data saved to 'iq_data.npy'")

    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":
    main()