import uhd
import numpy as np
import numpy.typing as npt
import time
from scipy.signal import welch
import matplotlib.pyplot as plt
import keyboard
import time
import threading

usrp = uhd.usrp.MultiUSRP()

num_samps = 1024 # number of samples received
center_freq = 2425e6 # Hz
sample_rate = 50e6 # Hz
gain = 76 # dB
Fs = sample_rate*2 # lets say we sampled at 1 MHz
N = 1024
collects=15000

def receive_iq_data(center_freq):
    usrp.set_rx_rate(sample_rate, 0)
    usrp.set_rx_freq(uhd.libpyuhd.types.tune_request(center_freq), 0)
    usrp.set_rx_gain(gain, 0)

    # Set up the stream and receive buffer
    st_args = uhd.usrp.StreamArgs("fc32", "sc16")
    st_args.channels = [0]
    metadata = uhd.types.RXMetadata()
    streamer = usrp.get_rx_stream(st_args)
    recv_buffer = np.zeros((1, 1024), dtype=np.complex64)

    # Start Stream
    stream_cmd = uhd.types.StreamCMD(uhd.types.StreamMode.start_cont)
    stream_cmd.stream_now = True
    streamer.issue_stream_cmd(stream_cmd)

    # Receive Samples
    samples = np.zeros(num_samps, dtype=np.complex64)
    for i in range(num_samps//1024):
        streamer.recv(recv_buffer, metadata)
        samples[i*1024:(i+1)*1024] = recv_buffer[0]

    # Stop Stream
    stream_cmd = uhd.types.StreamCMD(uhd.types.StreamMode.stop_cont)
    streamer.issue_stream_cmd(stream_cmd)

    #print(len(samples))
    #print(samples[0:10])
    return(samples)


def calculate_psd(x):
    x = x * np.hamming(len(x)) # apply a Hamming window
    PSD = np.abs(np.fft.fft(x))**2 / ((N*2)*Fs)
    PSD_log = 10.0*np.log10(PSD)
    PSD_shifted = np.fft.fftshift(PSD_log)
    center_freq = 2.45e9 # frequency we tuned our SDR to
    f = np.arange(Fs/-2.0, Fs/2.0, Fs/(N)) # start, stop, step.  centered around 0 Hz
    f += center_freq # now add center frequency
    #plt.plot(f, PSD_shifted)
    #plt.show()
    return(PSD_shifted)
    

def main():
    try:
        PSD= []
        for i in range(collects):
            iq_data = receive_iq_data(center_freq=2.425e9)
            x=iq_data
            iq_data = receive_iq_data(center_freq=2.475e9)
            x+=iq_data
            PSD.append(calculate_psd(x))
            
            
        np.save("psd.npy", PSD)

    except Exception as e:
        print(f"An error occurred: {e}")

if __name__ == "__main__":
    main()