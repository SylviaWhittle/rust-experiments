from pathlib import Path
import re

import numpy as np
import matplotlib.pyplot as plt

DATA_DIR = Path("./data")
if not DATA_DIR.exists():
    raise ValueError("No data directory")
PLOT_DIR = Path("./plots")
if not PLOT_DIR.exists():
    PLOT_DIR.mkdir()

files = Path("./data/").glob("data_*.bin")
pattern = r'\d+'

for file in files:
    print(f"file: {file.name}")
    k = re.findall(pattern=pattern, string=file.name)[0]
    print(f"k: {k}")

    data = np.fromfile(file, dtype='<f8')
    n = len(data) // 2
    sampling_step_size = 100
    x = data[:n:sampling_step_size]
    p = data[n::sampling_step_size]
    print(f"len x : {len(x)}")
    print(f"len p : {len(p)}")
    plt.scatter(x, p, marker='.')
    # plt.title(f"k = {-k/100}")
    plt.savefig(fname=f"./plots/plot_{k}.png")
    plt.close()
