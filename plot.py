import numpy as np
import matplotlib.pyplot as plt


def read_data_file(filename):
    data = []
    with open(filename, "r") as f:
        next(f)
        for line in f:
            if line.strip():
                access, size = map(int, line.strip().split(","))
                data.append((access, size))
    return data


def process_data(data):
    stash_sizes = np.array([size for _, size in data])
    total_samples = len(stash_sizes)

    max_stash = max(stash_sizes)
    print(f"max_stash={max_stash}")
    R_values = np.arange(-1, max_stash + 2)

    delta_R = []
    processed_data = []
    for R in R_values:
        exceeds_R = np.sum(stash_sizes > R)
        processed_data.append((R, exceeds_R))
        delta = exceeds_R / total_samples
        if delta > 0:
            delta_R.append((R, delta))

    return delta_R, processed_data[:-1]


def plot_stash_analysis(delta_R, title):
    R_values, deltas = zip(*delta_R)
    y_values = np.log2(1 / np.array(deltas))

    plt.plot(R_values, y_values, "o-", label=title)


configurations = [
    (2, "data/raw_2.txt", "N=2²⁰, Z=2, B=32"),
    (4, "data/raw_4.txt", "N=2²⁰, Z=4, B=32"),
    (6, "data/raw_6.txt", "N=2²⁰, Z=6, B=32"),
]

plt.figure(figsize=(15, 5))

for idx, (Z, filename, label) in enumerate(configurations, 1):
    plt.subplot(1, 3, idx)

    data = read_data_file(filename)
    if data:
        print(f"\nProcessing data for Z={Z} from {filename}")
        print(f"Number of data points: {len(data)}")

        delta_R, processed_data = process_data(data)
        plot_stash_analysis(delta_R, label)

        with open(f"data/{Z}.txt", "w") as f:
            for R, exceeds_R in processed_data:
                f.write(f"{R},{exceeds_R}\n")

        plt.title(f"Z={Z}")
        plt.xlabel("R")
        plt.ylabel("log₂(1/δ(R))")
        plt.grid(True, which="both", linestyle="--", alpha=0.7)
        plt.legend()

        ax = plt.gca()
        ax.xaxis.set_major_locator(plt.MaxNLocator(integer=True))

plt.suptitle("ORAM Stash Size Analysis", y=1.05)
plt.tight_layout()
plt.savefig("data/plot.png", bbox_inches="tight", dpi=300)
