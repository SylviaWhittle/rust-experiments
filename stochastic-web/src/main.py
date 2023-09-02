import numpy as np
import matplotlib.pyplot as plt

axes_col = [50, 50, 50]
axes_col = [col / 255 for col in axes_col]

x0 = 1.7679
p0 = 1.7679

r = 1
q = 3
k = -1.1
tau = 2 * np.pi * r / q
endtime = 10

x = np.zeros(endtime)
p = np.zeros(endtime)

x[0] = x0
p[0] = p0

cos_tau = np.cos(tau)
sin_tau = np.sin(tau)
sqrt_2 = np.sqrt(2)
print(f"cos tau: {np.cos(tau)}")
print(f"sin tau: {np.sin(tau)}")
print(f"sqrt 2: {np.sqrt(2)}")

for i in range(1, endtime):
    x_prev = x[i-1]
    p_prev = p[i-1]
    x[i] = x_prev * cos_tau + (p_prev + k * np.sin(sqrt_2 * x_prev)) * sin_tau
    p[i] = (p_prev + k * np.sin(sqrt_2 * x_prev)) * cos_tau - x_prev * sin_tau

# Set the linewidth of the axes lines
plt.gca().tick_params(width=2)

plt.plot(x, p, '.', color=axes_col, markersize=0.2)
plt.xlabel('$x$', fontsize=18)
plt.ylabel('$p$', fontsize=18)
plt.savefig("./plot.png")

print(f"x: {x}")
print("---")
print(f"p: {p}")
