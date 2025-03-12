"""
IO module for phylo2vec package. Convenience functions to read/write newick trees and phylo2vec vectors.
"""

import phylo2vec
import numpy as np
import phylo2vec.base
import phylo2vec.utils

def read_csv(path: str) -> np.ndarray:
    """
    Read a CSV file containing a phylo2vec vector.

    Parameters
    ----------
    path : str
        Path to the CSV file.

    Returns
    -------
    np.ndarray
        The data in the CSV file.
    """
    return np.loadtxt(path, delimiter=",")

def write_csv(data: np.ndarray, path: str) -> None:
    """"
    Write a CSV file containing a phylo2vec vector.

    Parameters
    ----------
    data : np.ndarray
        The data to write.
    path : str
        Path to the CSV file.

    Returns
    -------
    None
    """
    np.savetxt(path, data, delimiter=",")

def read_newick(path: str) -> str:
    """
    Read a Newick string from a file.

    E.g.: "((0,2)9,((1,3)7,(4,5)6)8)10;"

    Parameters
    ----------
    path : str
        Path to the Newick file.

    Returns
    -------
    str
        The data in the Newick file.
    """
    with open(path, "r") as f:
        return f.read()

# TODO: add docstring
def read_newick_labeled(path: str) -> tuple[str, dict]:
    data = read_newick(path)
    try:
        newick_int, labels = phylo2vec.utils.create_label_mapping(data)
    except ValueError as e:
        print(e)
        print("Integer based newick found in file. Use read_newick() instead.")
    return newick_int, labels



def write_newick(data: str, path: str) -> None:
    """
    Write a Newick string to a file.

    Parameters
    ----------
    data : str
        The data to write.
    path : str
        Path to the Newick file.

    Returns
    -------
    None
    """
    with open(path, "w") as f:
        f.write(data)

# TODO: add docstring
def write_newick_labeled(data: str, path: str, labels: dict) -> None:
    labeled_newick = phylo2vec.utils.apply_label_mapping(data, labels)
    write_newick(labeled_newick, path)
