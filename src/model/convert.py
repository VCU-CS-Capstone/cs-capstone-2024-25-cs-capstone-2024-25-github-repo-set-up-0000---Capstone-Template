import h5py
import threading
import csv
import os
import numpy as np


def hdf5_to_dataset(hdf5_file):
    """
    Load the IQ data from the HDF5 file
    """
    source_data = h5py.File(hdf5_file, "r")
    source_keys = list(source_data.keys())
    return source_data, source_keys


def main():
    hdf5_file = "./data/source_data/2_4ghz_bluetooth.h5"
    output = "./data/convert"
    data, keys = hdf5_to_dataset(hdf5_file)
    for idx, key in enumerate(keys, start=143):
        source_data = data[key][()]
        source_data = np.array(source_data)
        np.save(f"{output}/data_{idx}.npy", source_data)
        print(f"Saved data_{idx}.npy")
    csv_output = f"{output}/data_index.csv"
    with open(csv_output, mode='w', newline='') as file:
        writer = csv.writer(file)
        writer.writerow(["idx", "wifi"])
        for idx, key in enumerate(keys, start=143):
            writer.writerow([idx, "bluetooth"])
    print(f"Saved data_index.csv")


if __name__ == "__main__":
    main()
