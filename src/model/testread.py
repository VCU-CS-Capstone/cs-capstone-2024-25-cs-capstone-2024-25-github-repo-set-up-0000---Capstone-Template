import numpy as np
import csv


def main():
    read = "./convert/bluetooth/175900.npy"
    data = np.load(read)
    print(data)
    output_file = "./data_dump.csv"
    data = np.squeeze(data)
    np.savetxt(output_file, data, delimiter=",")
    print(f"Data has been written to {output_file}")


if __name__ == "__main__":
    main()