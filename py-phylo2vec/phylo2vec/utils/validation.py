"""Utilities for Phylo2Vec vector validation."""

import numpy as np

from phylo2vec import _phylo2vec_core


def check_v(v: np.ndarray) -> None:
    """Input validation of a Phylo2Vec vector

    The input is checked to satisfy the Phylo2Vec constraints

    Parameters
    ----------
    v : numpy.ndarray
        Phylo2Vec vector
    """
    _phylo2vec_core.check_v(v.tolist())


def check_m(m):
    """Input validation of a Phylo2Mat matrix

    The input is checked for the Phylo2Vec constraints and positive branch lengths

    Parameters
    ----------
    m : numpy.ndarray
        Phylo2Mat matrix
    """

    check_v(m[:, 0])

    assert np.all(m[:, 1:] > 0)
