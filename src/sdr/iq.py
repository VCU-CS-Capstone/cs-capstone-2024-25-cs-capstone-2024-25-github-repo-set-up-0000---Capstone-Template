import uhd
import numpy as np
import matplotlib.pyplot as plt

num_symbols = 2048000

# x_symbols array will contain complex numbers representing the QPSK symbols. Each symbol will be a complex number with a magnitude of 1 and a phase angle corresponding to one of the four QPSK constellation points (45, 135, 225, or 315 degrees)
x_int = np.random.randint(0, 4, num_symbols) # 0 to 3
x_degrees = x_int*360/4.0 + 45 # 45, 135, 225, 315 degrees
x_radians = x_degrees*np.pi/180.0 # sin() and cos() takes in radians
x_symbols = np.cos(x_radians) + 1j*np.sin(x_radians) # this produces our QPSK complex symbols
n = (np.random.randn(num_symbols) + 1j*np.random.randn(num_symbols))/np.sqrt(2) # AWGN with unity power
r = x_symbols + n * np.sqrt(0.01) # noise power of 0.01
print(r)
plt.plot(np.real(r), np.imag(r), '.')
plt.grid(True)
plt.show()

# Now save to an IQ file
print(type(r[0])) # Check data type.  Oops it's 128 not 64!
r = r.astype(np.complex64) # Convert to 64
print(type(r[0])) # Verify it's 64
r.tofile('qpsk_in_noise.iq') # Save to file