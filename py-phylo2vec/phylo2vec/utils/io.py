"""
IO module for phylo2vec package. Convenience functions to read/write Newick trees and phylo2vec vectors.
"""

import numpy as np

import phylo2vec


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


def write_csv(v: np.ndarray, path: str) -> None:
    """
    Write a CSV file containing a phylo2vec vector.

    Parameters
    ----------
    v : np.ndarray
        The phylo2vec vector to write.
    path : str
        Path to the CSV file.

    Returns
    -------
    None
    """
    np.savetxt(path, v, delimiter=",")


def read_newick(path: str) -> str:
    """
    Read a Newick string from a file.

    E.g.: "((0,2)9,((1,3)7,(4,5)6)8)10;"

    Parameters
    ----------
    path : str
        Path to the text file.

    Returns
    -------
    str
        The data in the Newick file.
    """
    with open(path, "r") as f:
        return f.read()


def read_newick_labeled(path: str) -> tuple[str, dict]:
    """
    Read a Newick string with string labels from a file.

    Converts the string labels to integers so that the Newick string can be converted to a phylo2vec vector.
    Uses `phylo2vec.utils.create_label_mapping` to create a mapping of the string labels to integers.
    E.g. "((A,B),(D,E));" --> ("((0,1),(2,3));", {"A": "0", "B": "1", "D": "2", "E": "3"})

    Parameters
    ----------
    path : str
        Path to the text file.

    Returns
    -------
    tuple[str, dict]
        The data in the Newick file and the labels: (newick_int, labels)

    """
    data = read_newick(path)
    newick_int, labels = phylo2vec.utils.create_label_mapping(data)
    return newick_int, labels


def write_newick(newick: str, path: str) -> None:
    """
    Write a Newick string to a file.

    Parameters
    ----------
    newick : str
        The Newick string to write.
    path : str
        Path to the text file.

    Returns
    -------
    None
    """
    with open(path, "w") as f:
        f.write(newick)


def write_newick_labeled(newick: str, labels: dict, path: str) -> None:
    """
    Write a Newick string with string labels to a file.

    Converts the integer labels back to string labels using the provided mapping.
    E.g. ("((0,1),(2,3));", {"A": "0", "B": "1", "D": "2", "E": "3"}) --> "((A,B),(D,E));"

    Parameters
    ----------
    newick : str
        The labeled Newick string to write.
    labels : dict
        Mapping of leaf labels (integers converted to string) to taxa.
    path : str
        Path to the text file.

    Returns
    -------
    None
    """
    labeled_newick = phylo2vec.utils.apply_label_mapping(newick, labels)
    write_newick(labeled_newick, path)
