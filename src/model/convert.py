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
    hdf5_file = "./data/2_4ghz_indoor.h5"
    output = "./data/convert"
    data, keys = hdf5_to_dataset(hdf5_file)
    for idx, key in enumerate(keys, start=180):
        source_data = data[key][()]
        source_data = np.array(source_data)
        np.save(f"{output}/data_{idx}.npy", source_data)
        print(f"Saved data_{idx}.npy")


if __name__ == "__main__":
    main()
