import uhd
import numpy as np
import matplotlib.pyplot as plt
import scipy

usrp = uhd.usrp.MultiUSRP()

num_samps = 2048 # number of samples received
center_freq = 2.45e9 # Hz
sample_rate = 40e6 # Hz
gain = 76 # dB

usrp.set_rx_rate(sample_rate, 0)
usrp.set_rx_freq(uhd.libpyuhd.types.tune_request(center_freq), 0)
usrp.set_rx_gain(gain, 0)

# Set up the stream and receive buffer
st_args = uhd.usrp.StreamArgs("fc32", "sc16")
st_args.channels = [0]
metadata = uhd.types.RXMetadata()
streamer = usrp.get_rx_stream(st_args)
recv_buffer = np.zeros((1, 1000), dtype=np.complex64)

# Start Stream
stream_cmd = uhd.types.StreamCMD(uhd.types.StreamMode.start_cont)
stream_cmd.stream_now = True
streamer.issue_stream_cmd(stream_cmd)

# Receive Samples
samples = np.zeros(num_samps, dtype=np.complex64)
for i in range(num_samps//1000):
    streamer.recv(recv_buffer, metadata)
    samples[i*1000:(i+1)*1000] = recv_buffer[0]

# Stop Stream
stream_cmd = uhd.types.StreamCMD(uhd.types.StreamMode.stop_cont)
streamer.issue_stream_cmd(stream_cmd)

avg_pwr = np.var(samples) # (signal should have roughly zero mean)

Fs = 40e6 # lets say we sampled at 1 MHz
# assume x contains your array of IQ samples
N = 2048
samples = samples[0:N] # we will only take the FFT of the first 1024 samples, see text below
PSD = np.abs(np.fft.fft(samples))**2 / (N*Fs)
PSD_log = 10.0*np.log10(PSD)
PSD_shifted = np.fft.fftshift(PSD_log)
center_freq = 2.45e9 # frequency we tuned our SDR to
f = np.arange(Fs/-2.0, Fs/2.0, Fs/N) # start, stop, step.  centered around 0 Hz
f += center_freq # now add center frequency
plt.plot(f, PSD_shifted)
plt.show()