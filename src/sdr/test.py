import uhd
import numpy as np
import matplotlib.pyplot as plt
import scipy

usrp = uhd.usrp.MultiUSRP()

num_samps = 1000 * 1000 # number of samples received
num_samps_h = 2000 # number of samples received
num_samps_w = 2000 # number of samples received
center_freq = 2422e6 # Hz
sample_rate = 15e6 # Hz
gain = 50 # dB

usrp.set_rx_rate(sample_rate, 0)
usrp.set_rx_bandwidth(20e6)
usrp.set_rx_freq(uhd.libpyuhd.types.tune_request(center_freq), 0)
usrp.set_rx_gain(gain, 0)

print("bandwidth", usrp.get_rx_bandwidth())

# Set up the stream and receive buffer
st_args = uhd.usrp.StreamArgs("fc32", "sc16")
st_args.channels = [0]
metadata = uhd.types.RXMetadata()
streamer = usrp.get_rx_stream(st_args)
recv_buffer = np.zeros((1, num_samps_w), dtype=np.complex64)

# Start Stream
stream_cmd = uhd.types.StreamCMD(uhd.types.StreamMode.start_cont)
stream_cmd.stream_now = True
streamer.issue_stream_cmd(stream_cmd)

# Receive Samples
samples = np.zeros((num_samps_h, num_samps_w), dtype=np.complex64)
for i in range(num_samps_h):
    streamer.recv(recv_buffer, metadata)
    samples[i] = scipy.fft.fft(recv_buffer[0])

# Stop Stream
stream_cmd = uhd.types.StreamCMD(uhd.types.StreamMode.stop_cont)
streamer.issue_stream_cmd(stream_cmd)

print(len(samples))
print(samples[0:10])
abs_samples = np.absolute(samples)
print(abs_samples.max(), abs_samples.mean(), abs_samples.std())
max_display_value =  abs_samples.mean() + 2 * abs_samples.std()
abs_samples[np.where(abs_samples > max_display_value)] = max_display_value
plt.imshow(abs_samples)
plt.show()
