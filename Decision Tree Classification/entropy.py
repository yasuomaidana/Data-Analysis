import numpy as np

# Generate x and y values
x = np.linspace(0, 1000, 32)
y = np.linspace(0, 1000, 32)

# Create a grid of x and y values
X, Y = np.meshgrid(x, y)

# Assign z values by randomly choosing {1,0} with a distribution of 30% of 1
Z = np.random.choice([1, 0], size=X.shape, p=[0.3, 0.7])

# Check if the distribution of 1 is near 30% with a 5% tolerance
assert np.isclose(np.mean(Z == 1), 0.3, atol=0.05)

# Check if the size of Z is 1024
assert Z.size == 1024
