import numpy as np
from numpy.random import random


def generate_distribution(x, y, mu_x, mu_y, sigma_x, sigma_y):
    # Create a grid of x and y values
    X, Y = np.meshgrid(x, y)

    # Calculate the Gaussian distribution
    gaussian = np.exp(-((X - mu_x) / sigma_x) ** 2 - ((Y - mu_y) / sigma_y) ** 2)

    # Assign z values based on the Gaussian probability distribution map
    Z = random(X.shape) < gaussian

    return X, Y, Z


def calculate_entropy(Z):
    # Calculate the probability of Z
    p = np.sum(Z) / Z.size

    # Calculate the entropy
    if p == 0 or p == 1:
        entropy = 0
    else:
        entropy = -p * np.log2(p) - (1 - p) * np.log2(1 - p)

    return entropy
