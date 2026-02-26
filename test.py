import numpy as np
import matplotlib.pyplot as plt
from scipy.optimize import curve_fit

# Data
n = np.array([70,142,292,616,1360,3232,8512,25216,83200,297472])

times = np.array([
    [0.032,0.098,0.026,0.021,0.019],
    [0.068,0.052,0.044,0.037,0.035],
    [0.124,0.081,0.071,0.063,0.055],
    [0.232,0.136,0.104,0.103,0.089],
    [0.277,0.201,0.167,0.153,0.144],
    [0.423,0.453,0.353,0.384,0.302],
    [0.83,0.858,0.887,0.85,0.852],
    [2.125,2.112,2.104,2.128,2.066],
    [4.618,4.798,4.69,4.589,4.548],
    [10.457,10.474,10.345,10.402,10.496]
])

avg_times = times.mean(axis=1)

# Models
def linear_model(n, a, b):
    return a*n + b

def nlogn_model(n, c, d):
    return c*n*np.log2(n) + d

def power_model(n, alpha, k):
    return alpha * n**k

# Fit
params_linear, _ = curve_fit(linear_model, n, avg_times)
params_nlogn, _ = curve_fit(nlogn_model, n, avg_times)
params_power, _ = curve_fit(power_model, n, avg_times)

# Smooth curve
n_smooth = np.linspace(min(n), max(n), 500)

# Plot
plt.figure(figsize=(8,6))
plt.scatter(n, avg_times, color='black', label='Average Data')

plt.plot(n_smooth, linear_model(n_smooth, *params_linear),
         label='Fit: a·n + b')

plt.plot(n_smooth, nlogn_model(n_smooth, *params_nlogn),
         label='Fit: c·n·log₂(n) + d')

plt.plot(n_smooth, power_model(n_smooth, *params_power),
         label=f'Fit: α·n^{params_power[1]:.3f}')

plt.xscale('log')
plt.yscale('log')

plt.xlabel('n (number of backpointers)')
plt.ylabel('Time (ms)')
plt.title('Runtime Fit Comparison (Ambiguous)')
plt.legend()
plt.grid(True, which="both", ls="--")
plt.show()

# Print parameters
print("Linear fit (a, b):", params_linear)
print("n log n fit (c, d):", params_nlogn)
print("Power fit (alpha, k):", params_power)