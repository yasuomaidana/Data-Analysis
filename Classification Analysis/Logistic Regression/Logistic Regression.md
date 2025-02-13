## Logistic Regression

### Introduction

The logistic regression is a classification algorithm that is used to predict the probability of a categorical dependent
variable. In logistic regression, the dependent variable is a binary variable that contains data coded as 1 (yes,
success, etc.) or 0 (no, failure, etc.). In other words, the logistic regression model predicts $P(Y=1)$ as a function of
$X$.


This can be extended to model several classes of events such as determining whether an image contains a cat, dog, lion, etc.

Each object being detected would be assigned a probability between 0 and 1, with a sum of one.

## Sigmoid Function

We need something to convert any input to a probability, or $[0, 1]$.

Sigmoid function is the solution. 
$$Sigmoid(t) = \frac{1}{1+e^{-t}}$$


