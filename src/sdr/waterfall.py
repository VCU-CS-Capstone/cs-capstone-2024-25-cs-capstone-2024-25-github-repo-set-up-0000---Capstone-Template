import uhd
import numpy as np
import time
from scipy.signal import welch
import matplotlib.pyplot as plt

def configure_sdr(sdr, center_freq, sample_rate=50e6, gain=30):
    """
    Configures the SDR with the specified parameters.
    """
    sdr.set_rx_rate(sample_rate)
    sdr.set_rx_freq(uhd.types.TuneRequest(center_freq))
    sdr.set_rx_gain(gain)

def receive_iq_data(sdr, num_samples=1024, duration=1.0):
    """
    Receives IQ data from the SDR.
    """
    stream_args = uhd.usrp.StreamArgs("fc32", "sc16")
    rx_streamer = sdr.get_rx_stream(stream_args)
    samples = np.zeros((num_samples,), dtype=np.complex64)
    metadata = uhd.types.RXMetadata()

    rx_streamer.issue_stream_cmd(
        uhd.types.StreamCMD(uhd.types.StreamMode.start_cont)
    )

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
    return np.concatenate(iq_data)

def calculate_psd(iq_data, sample_rate):
    """
    Calculate the Power Spectral Density (PSD) from IQ data.
    """
    frequencies, psd = welch(
        iq_data,
        fs=sample_rate,
        nperseg=1024,
        return_onesided=False,
        scaling='density'
    )
    return frequencies, psd

def plot_waterfall(waterfall_data, frequencies, base_center_freq):
    """
    Plot the waterfall plot for the full 100 MHz spectrum.
    """
    adjusted_frequencies = frequencies + base_center_freq - 25e6  # Adjust to full spectrum
    waterfall_array = np.array(waterfall_data)
    
    plt.figure(figsize=(10, 6))
    plt.imshow(
        10 * np.log10(waterfall_array),
        extent=[
            adjusted_frequencies[0] / 1e6,
            adjusted_frequencies[-1] / 1e6,
            0,
            len(waterfall_array)
        ],
        aspect='auto',
        cmap='viridis',
        origin='lower'
    )
    plt.title("100 MHz Waterfall Plot")
    plt.xlabel("Frequency (MHz)")
    plt.ylabel("Time (slices)")
    plt.colorbar(label="Power (dB)")
    plt.show()

def main():
    try:
        # Create a USRP source
        sdr = uhd.usrp.MultiUSRP()

        # Base configuration
        base_center_freq = 2.4e9  # Example: 2.4 GHz
        sample_rate = 10e6
        duration = 1.0  # Duration for each bin
        num_iterations = 10  # Number of waterfall rows

        waterfall_data = []

        print("Starting reception and processing...")
        for _ in range(num_iterations):
            combined_psd = []

            # Process Bin 1: 2.4 GHz to 2.45 GHz
            configure_sdr(sdr, base_center_freq, sample_rate=sample_rate, gain=30)
            iq_data_bin1 = receive_iq_data(sdr, num_samples=1024, duration=duration)
            frequencies_bin1, psd_bin1 = calculate_psd(iq_data_bin1, sample_rate)
            combined_psd.extend(psd_bin1)

            # Process Bin 2: 2.45 GHz to 2.5 GHz
            configure_sdr(sdr, base_center_freq + 50e6, sample_rate=sample_rate, gain=30)
            iq_data_bin2 = receive_iq_data(sdr, num_samples=1024, duration=duration)
            frequencies_bin2, psd_bin2 = calculate_psd(iq_data_bin2, sample_rate)
            combined_psd.extend(psd_bin2)

            # Update the waterfall plot
            waterfall_data.append(combined_psd)

        # Combine frequency ranges for the full spectrum
        combined_frequencies = np.concatenate((frequencies_bin1, frequencies_bin2 + 50e6))
        plot_waterfall(waterfall_data, combined_frequencies, base_center_freq)

    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":
    main()
