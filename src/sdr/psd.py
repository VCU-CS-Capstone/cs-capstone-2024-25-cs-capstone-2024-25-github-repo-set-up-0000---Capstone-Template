import uhd
import numpy as np
import time
from scipy.signal import welch
import matplotlib.pyplot as plt

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
    stream_args = uhd.usrp.StreamArgs("fc32", "sc16")
    rx_streamer = sdr.get_rx_stream(stream_args)

    samples = np.zeros((num_samples,), dtype=np.complex64)
    metadata = uhd.types.RXMetadata()

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

        iq_data.append(samples[:num_received].copy())

    rx_streamer.issue_stream_cmd(
        uhd.types.StreamCMD(uhd.types.StreamMode.stop_cont)
    )

    print("Reception complete.")
    return np.concatenate(iq_data)

def calculate_psd(iq_data, sample_rate):
    """
    Calculate the Power Spectral Density (PSD) from IQ data.

    :param iq_data: IQ samples as a NumPy array.
    :param sample_rate: Sampling rate in Hz.
    :return: Frequencies and PSD values.
    """
    frequencies, psd = welch(
        iq_data,
        fs=sample_rate,
        nperseg=1024,
        return_onesided=False,
        scaling='density'
    )
    return frequencies, psd

def plot_psd(frequencies, psd, center_freq):
    """
    Plot the Power Spectral Density (PSD).

    :param frequencies: Frequencies array.
    :param psd: PSD values.
    :param center_freq: Center frequency in Hz.
    """
    # Adjust frequencies to account for center frequency
    adjusted_frequencies = frequencies + center_freq

    plt.figure(figsize=(10, 6))
    plt.plot(adjusted_frequencies / 1e6, 10 * np.log10(psd), label="PSD")
    plt.title("Power Spectral Density (PSD)")
    plt.xlabel("Frequency (MHz)")
    plt.ylabel("Power Density (dB/Hz)")
    plt.grid(True)
    plt.legend()
    plt.show()

def main():
    try:
        # Create a USRP source
        sdr = uhd.usrp.MultiUSRP()

        # Configure the SDR
        sample_rate = 1e6
        center_freq = 2.4e9
        configure_sdr(sdr, center_freq=center_freq, sample_rate=sample_rate, gain=76)

        # Receive IQ data for 5 seconds
        duration = 5.0
        iq_data = receive_iq_data(sdr, num_samples=2048, duration=duration)

        # Save the IQ data to a file
        np.save("iq_data.npy", iq_data)
        print(f"IQ data saved to 'iq_data.npy'")

        # Calculate and plot PSD
        frequencies, psd = calculate_psd(iq_data, sample_rate)
        plot_psd(frequencies, psd, center_freq)

    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":
    main()