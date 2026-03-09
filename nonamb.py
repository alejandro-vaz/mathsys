import numpy as np
import matplotlib.pyplot as plt
from scipy.optimize import curve_fit

# Data
n = np.array([5096,14996,49196,175196,657596,2543996])

times = np.array([
    [0.268,0.223,0.195,0.176,0.271],
    [0.768,0.74,0.819,0.674,0.642],
    [1.859,1.597,1.588,1.625,1.635],
    [4.901,4.715,5.1,4.867,5.773],
    [12.077,13.14,12.195,14.056,13.571],
    [34.489,30.897,32.388,36.298,33.166]
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
plt.title('Runtime Fit Comparison (Unambiguous)')
plt.legend()
plt.grid(True, which="both", ls="--")
plt.show()

# Print parameters
print("Linear fit (a, b):", params_linear)
print("Power fit (alpha, k):", params_power)