# Regularized Linear Models

Regularized linear models are a class of linear regression techniques that include a penalty term in the loss function
to prevent overfitting and improve generalization. The two most common types of regularization are Lasso (L1) and
Ridge (L2) regularization.

- Since overfitting is a real problem, we may stay with a small degree of freedom.
- We can also constrain the weight of the model (reduce the variance of the result):
- Regularized Cost Function = MSE + Regularization term
    - Ridge regression (L2): Regularization $term = λ * ||w||^2$
    - Lasso regression (L1): Regularization $term = λ * ||w||_1$
    - Elastic Net: Regularization $term = λ_1 * ||w||^2 + λ_2 * ||w||_1$

## Ridge Regression

Ridge regression (Tikhonov Regularization) adds a penalty term to the loss function that is proportional to the square
of the magnitude of the coefficients.
$$\text{Regularization Term: }\alpha\frac12\sum_{i=1}^nw_i^2$$

- Forces the learning algorithm to fit the data and keep the model weights as small as possible.
- If $w$ is the vector of feature weights, this regularization term is $\frac12$ of $L_2$ norm of $w$.
- For Gradient Descent, add $\alpha w$ to the MSE gradient vector.

This helps shrink the coefficients to zero, reducing their variance and improving the model's generalization.

## Lasso Regression

Lasso regression (The Least Absolute Shrinkage and Selection Operator) adds a penalty term to the loss function that is
proportional to the absolute value of the coefficients.

$$\text{Regularization Term: }\alpha\sum_{i=1}^n|w_i|$$

- Forces the learning algorithm to fit the data and keep the model weights as small as possible.
- If $w$ is the vector of feature weights, this regularization term is $L_1$ norm of $w$.
- It will try to completely eliminate the weights of the least important features (set the weights to zero) - automatic
  feature
  selection

## Ridge vs Lasso

|                             | Ridge                                                             | Lasso                                                 |
|-----------------------------|-------------------------------------------------------------------|-------------------------------------------------------|
| Regularization/Penalty Term | $$\alpha\frac12\sum_{i=1}^nw_i^2$$                                | $$\alpha\sum_{i=1}^n \lvert w_i \rvert$$              |
| Approach                    | Shrinks the coefficients but doesn't set any coefficient to zero. | Can shrink some coefficients to zero                  |
| How to help?                | Shrinkage                                                         | Shrinkage + Feature selection                         |
| Scenario                    | Works well when there is a small number of features.              | Works well when there are a large number of features. |

## Elastic Net

Elastic Net is a combination of Lasso and Ridge regression. It includes both L1 and L2 regularization terms in the
regularization term.

- A middle ground between Ridge and Lasso.

$$\text{Regularization Term: }\frac{1-r}2\alpha_1\sum_{i=1}^nw_i^2 + r\alpha_2\sum_{i=1}^n|w_i|$$

- If $r=0$, it is equivalent to Ridge regression.
- If $r=1$, it is equivalent to Lasso regression.

## Ridge, Lasso, or Elastic Net?

- Ridge is a good default.
- High-dimensional data or data that needs feature selection, then Lasso or Elastic Net (Elastic is preferred over
  Lasso).

## Additional material

- IBM article [What is ridge regression?](https://www.ibm.com/think/topics/ridge-regression)
