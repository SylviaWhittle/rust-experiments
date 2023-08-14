import time
import math

start = time.time()

max_number = 1000000
i = 3
latest_prime = 2
while i < max_number:
    for val in range(3, int(math.floor(math.sqrt(i)))):
        if i % val == 0:
            break
    else:
        latest_prime = i

    i += 2

elapsed = (time.time() - start) / 1e-3
print(f"Found primes up to {max_number} in {elapsed:.2f}ms. Last prime: {latest_prime}")