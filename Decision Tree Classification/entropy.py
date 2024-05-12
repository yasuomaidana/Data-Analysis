import numpy as np
from numpy.random import random
import pandas as pd
from pandas import Series


def generate_distribution(x, y, mu_x, mu_y, sigma_x, sigma_y):
    # Create a grid of x and y values
    X, Y = np.meshgrid(x, y)

    # Calculate the Gaussian distribution
    gaussian = np.exp(-((X - mu_x) / sigma_x) ** 2 - ((Y - mu_y) / sigma_y) ** 2)

    # Assign z values based on the Gaussian probability distribution map
    Z = random(X.shape) < gaussian

    return X, Y, Z


def calculate_entropy(series: Series):
    # Calculate the probability of each unique value in the series
    probabilities = series.value_counts(normalize=True)
    # Calculate the entropy
    entropy = -np.sum(probabilities * np.log2(probabilities))
    return entropy


def calculate_gain(data, target, column, binning_func=pd.qcut, num_bins=8):
    # Calculate the initial entropy of the target
    initial_entropy = calculate_entropy(data[target])

    # Check if the column is categorical
    if not isinstance(data[column].dtype, pd.CategoricalDtype):
        # Bin the column
        data[f"{column}_tiles"] = binning_func(data[column], num_bins)
        column = f"{column}_tiles"

    column_entropy = calculate_entropy(data[column])

    # Calculate the gain
    gain = initial_entropy - column_entropy

    return gain
