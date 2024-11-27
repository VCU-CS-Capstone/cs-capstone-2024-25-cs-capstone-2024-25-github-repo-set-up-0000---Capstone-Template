import numpy as np
import scipy.signal as signal
import matplotlib.pyplot as plt


def read_np_array(file_path):
    data = np.load(file_path)

    return data


def interleaved_to_complex(source_data):
    """
    Convert interleaved IQ data to complex numbers
    """
    source_data = np.array(source_data)
    source_data = source_data / 32768
    source_data = source_data.astype(np.float32).view(np.complex64)
    return source_data


def iq_to_psd(iq_data, sample_rate, fft):
    """
    # Compute the PSD of the input IQ data
    """
    freq, psd = signal.welch(
        iq_data, fs=sample_rate, nperseg=fft, return_onesided=False
    )
    return freq, psd


def main():
    data = read_np_array("./data/testing_data/data_0.npy")
    data = interleaved_to_complex(data)
    print(len(data))
    freq, psd = iq_to_psd(data, 30000000, 8096)
    np.save("test_psd.npy", psd)
    print(len(psd))
    plt.figure()
    plt.plot(freq, psd)
    plt.title("Power Spectral Density (PSD)")
    plt.xlabel("Frequency (Hz)")
    plt.ylabel("Power/Frequency (dB/Hz)")
    plt.grid()
    plt.show()


if __name__ == "__main__":
    main()
