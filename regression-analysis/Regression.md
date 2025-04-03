# Regression

Regression is a statistical method used to model the relationship between a dependent variable (target value) and one or
more independent variables (predictors). It is commonly used for prediction and forecasting.

- Dependant variable $Y$
- Independent variable $X$

Regression techniques differ based on:

- Number of predictors, or independent variables.
- Type of relationship between $X$ and $Y$.

## Types of Regression

### Linear Regression

- The true relationship is linear
- Errors are normally distributed
- Homoscedasticity of errors (or, equal variance around the line)
- Independence of the observations

### Simple Linear Regression

- One independent variable
- Linear relationship
- Equation: $Y = a_0 + a_1x + e_i$

## Error measurement

In regression, it is the difference between the predicted value and the actual value. The error is also called the
residual.

$$e_i=\hat{y}_i-y_i$$

Where:

- $e_i$ is the error
- $\hat{y}_i$ is the predicted value
- $y_i$ is the actual value

## Cost measurement

The cost function is used to measure the performance of the regression model. This can be done using sum of squares of
errors, or SSE.

$$SSE=\sum_{i=1}^n e_i^2$$

Where:

- $SSE$ is the sum of squares of errors
- $e_i$ is the error

The mean squared error (MSE) is the average of the sum of squares of errors.
$$MSE=\frac{1}{n}\sum_{i=1}^n e_i^2$$

Where:

- $MSE$ is the mean squared error
- $n$ is the number of observations

## Optimization

This is how the model learns. The goal is to minimize the cost function. This is done using optimization

Minimize SE, LSSE: Least Sum of Squared Error, also called Ordinary Least Squared (OLS)
$$\min(SSE)=\min\left(\sum_{i=1}^n e_i^2\right)$$

Minimize MSE, MMSE: Minimal Mean of Squared Error
$$\min(MSE)=\min\left(\frac{1}{n}\sum_{i=1}^n e_i^2\right)$$

> Mean removes the effect of the number of observations. It is used to compare different models with different number of
> observations.

## How to reduce the cost?

Gradient Descent
- Updating the coefficients of the line to reduce the cost
- Use the knowledge of partial derivatives to know the direction to change
- Decide how much to change
  - Learning rate: Big or Small?

### Gradient Descent 

- Problems:
  - Local VS Global
  - Plateau
- Scale of attribute:
  - All features must have similar scales
  - Normalization
  - StandardScaler from Scikit-Learn
- SGD:
  - Training data must be independent and identically distributed