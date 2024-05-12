import numpy as np
import matplotlib.pyplot as plt
from numpy import linspace, meshgrid
from numpy.random import choice

# Generate x and y values
x = linspace(0, 1000, 32)
y = linspace(0, 1000, 32)

# Create a grid of x and y values
X, Y = meshgrid(x, y)

# Define the Gaussian distribution parameters
mu_x = 600
mu_y = 700
sigma_x = 200
sigma_y = 400

# Calculate the Gaussian distribution
gaussian = np.exp(-((X - mu_x) / sigma_x) ** 2 - ((Y - mu_y) / sigma_y) ** 2)

# Assign z values by randomly choosing {1,0} with the custom distribution
Z = choice([1, 0], size=X.shape, p=[0.5, 0.5]) * gaussian > 0.05

# Check if the distribution of 1 is near 30% with a 5% tolerance
assert np.isclose(np.mean(Z == 1), 0.3, atol=0.05)

# Check if the size of Z is 1024
assert Z.size == 1024

# Check if the distribution from x = (0,400) is less than 10% with a 5% tolerance
assert np.isclose(Z[:, :13].sum()/Z[:, :13].size, 0.1, atol=0.05)

# Plot the map
plt.scatter(X, Y, c=Z)
plt.show()
