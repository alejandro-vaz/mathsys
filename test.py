import numpy as np
import matplotlib.pyplot as plt
from scipy.optimize import curve_fit

# Data
n = np.array([4318,11838,36478,124158,453118,1725438])

times = np.array([
    [0.711,0.593,0.551,0.503,0.542],
    [1.169,1.126,1.058,1.092,1.139],
    [3.112,2.899,3.265,3.113,3.416],
    [8.195,7.797,7.592,7.731,7.428],
    [17.855,17.419,16.333,16.821,16.853],
    [38.487,45.941,48.374,42.204,51.727]
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
params_power, _ = curve_fit(power_model, n, avg_times)

# Smooth curve
n_smooth = np.linspace(min(n), max(n), 500)

# Plot
plt.figure(figsize=(8,6))
plt.scatter(n, avg_times, color='black', label='Average Data')

plt.plot(n_smooth, linear_model(n_smooth, *params_linear),
         label=f'Fit: {params_linear[0]:.3}·n + {params_linear[1]:.3}')


plt.plot(n_smooth, power_model(n_smooth, *params_power),
         label=f'Fit: {params_power[0]:.3}·n^{params_power[1]:.3f}')

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
print("Power fit (alpha, k):", params_power)