import numpy as np
import matplotlib.pyplot as plt
from scipy.optimize import curve_fit

# Data
n = np.array([53,110,233,515,1223,3215,9503,31295,111743,420095])

times = np.array([
    [0.019,0.015,0.013,0.008,0.011],
    [0.026,0.021,0.017,0.01,0.015],
    [0.072,0.04,0.035,0.031,0.028],
    [0.063,0.068,0.059,0.051,0.047],
    [0.153,0.104,0.089,0.083,0.077],
    [0.174,0.210,0.162,0.161,0.133],
    [0.481,0.499,0.520,0.483,0.499],
    [1.282,1.349,1.393,1.293,1.38],
    [2.774,2.804,2.918,2.837,2.959],
    [6.401,6.467,6.312,6.596,6.733]
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
plt.title('Runtime Fit Comparison (Non-ambiguous)')
plt.legend()
plt.grid(True, which="both", ls="--")
plt.show()

# Print parameters
print("Linear fit (a, b):", params_linear)
print("n log n fit (c, d):", params_nlogn)
print("Power fit (alpha, k):", params_power)