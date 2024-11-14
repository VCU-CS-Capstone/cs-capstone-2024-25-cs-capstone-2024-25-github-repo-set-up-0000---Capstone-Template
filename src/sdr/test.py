import uhd
import numpy as np
import matplotlib.pyplot as plt

usrp = uhd.usrp.MultiUSRP()

N = 2048
center_freq = 2.45e9 # Radio Center Frequency in Hz
rate = 40e6         # Sample Rate in S/s or Hz
gain = 50          # dB

usrp.set_rx_rate(rate, 0)
usrp.set_rx_freq(uhd.types.TuneRequest(center_freq), 0)
usrp.set_rx_antenna('RX2', 0)
usrp.set_rx_gain(gain, 0)

# Set up the stream and receive buffer
st_args = uhd.usrp.StreamArgs('fc32', 'sc16')
st_args.channels = [0]

metadata = uhd.types.RXMetadata()
streamer = usrp.get_rx_stream(st_args)
recv_buffer = np.zeros((1, N), dtype=np.complex64)

# Start Stream
stream_cmd = uhd.types.StreamCMD(uhd.types.StreamMode.start_cont)
stream_cmd.stream_now = True
streamer.issue_stream_cmd(stream_cmd)

# Receive Samples
samples = np.zeros(N, dtype=np.complex64)
streamer.recv(recv_buffer, metadata)
samples[0:N] = recv_buffer[0]

# Stop Stream
stream_cmd = uhd.types.StreamCMD(uhd.types.StreamMode.stop_cont)
streamer.issue_stream_cmd(stream_cmd)

fft = np.abs(np.fft.fft(samples))**2 / (N*rate)
log = 10.0*np.log10(fft)
shifted = np.fft.fftshift(log)
f = np.arange(center_freq + rate/-2.0, center_freq+rate/2.0, rate/N)

plt.plot(f, shifted)
plt.xlabel("Frequency [Hz]")
plt.ylabel("Magnitude [dB]")
plt.grid(True)
plt.show()
