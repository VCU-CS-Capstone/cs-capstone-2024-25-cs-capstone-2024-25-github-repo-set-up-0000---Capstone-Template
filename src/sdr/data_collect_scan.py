import uhd
import numpy as np
import time
from scipy.signal import welch
import matplotlib.pyplot as plt

def configure_sdr(sdr, center_freq, sample_rate, gain):
    sdr.set_rx_rate(sample_rate)
    print(f"Sampling rate set to {sample_rate} Hz")
    
    sdr.set_rx_freq(uhd.types.TuneRequest(center_freq))
    print(f"Center frequency set to {center_freq} Hz")
    
    sdr.set_rx_gain(gain)
    print(f"Gain set to {gain} dB")

def receive_iq_data(sdr, num_samples, duration):
    stream_args = uhd.usrp.StreamArgs("fc32", "sc16")
    rx_streamer = sdr.get_rx_stream(stream_args)

    samples = np.zeros((num_samples,), dtype=np.complex64)
    metadata = uhd.types.RXMetadata()

    rx_streamer.issue_stream_cmd(uhd.types.StreamCMD(uhd.types.StreamMode.start_cont))

    print("Receiving IQ data...")
    iq_data = []

    start_time = time.time()
    while time.time() - start_time < duration:
        num_received = rx_streamer.recv(samples, metadata)
        if metadata.error_code != uhd.types.RXMetadataErrorCode.none:
            print(f"Error receiving data: {metadata.error_code}")
            continue

        iq_data.append(samples[:num_received].copy())

    rx_streamer.issue_stream_cmd(uhd.types.StreamCMD(uhd.types.StreamMode.stop_cont))
    print("Reception complete.")
    return np.concatenate(iq_data)

def compute_psd(iq_data, sample_rate):
    f, psd = welch(iq_data, fs=sample_rate, nperseg=1024, return_onesided=False)
    return f, psd

def main():
    try:
        # Create a USRP source
        sdr = uhd.usrp.MultiUSRP()

        # Parameters
        sample_rate = 5e6
        gain = 30
        num_samples = 1024
        duration = 5.0
        freq_ranges = [2.425e9, 2.475e9]  # Two 50 MHz windows

        for idx, center_freq in enumerate(freq_ranges):
            print(f"Processing window {idx + 1} with center frequency {center_freq} Hz")

            # Configure SDR for the current center frequency
            configure_sdr(sdr, center_freq=center_freq, sample_rate=sample_rate, gain=gain)

            # Receive IQ data
            iq_data = receive_iq_data(sdr, num_samples=num_samples, duration=duration)

            # Compute PSD
            f, psd = compute_psd(iq_data, sample_rate=sample_rate)

            # Save PSD data to file
            output_file = f"psd_window_{idx + 1}.npy"
            np.save(output_file, {"frequency": f, "psd": psd})
            print(f"PSD data saved to '{output_file}'")

        # Load and plot the PSD data
        plt.figure(figsize=(10, 6))
        for idx in range(len(freq_ranges)):
            data = np.load(f"psd_window_{idx + 1}.npy", allow_pickle=True).item()
            f = data["frequency"]
            psd = data["psd"]
            plt.plot(f, 10 * np.log10(psd), label=f"Window {idx + 1}")

        plt.xlabel("Frequency (Hz)")
        plt.ylabel("Power Spectral Density (dB/Hz)")
        plt.title("Power Spectral Density (PSD)")
        plt.legend()
        plt.grid()
        plt.show()

    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":
    main()
