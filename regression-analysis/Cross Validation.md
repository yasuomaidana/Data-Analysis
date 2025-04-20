# Cross Validation

Cross-validation is a statistical method used to estimate the skill of machine learning models. It is primarily used to
assess how the results of a statistical analysis will generalize to an independent data set. The basic idea is to
partition the data into subsets, train the model on some subsets (training set), and validate it on the remaining
subsets (validation set). This process is repeated multiple times, and the results are averaged to provide a more
reliable estimate of model performance.

## Rules

- **Never ever** use the test set for training or validation.

## Data representation

- Training data and the test data should represent the future data.
- How shall we know?
- At least we can know if training and test data represent the same pattern by observing the learning curve.

See example in this [article](https://www.soapboxlabs.com/blog/common-machine-learning-problems/)

### Unrepresentative Training Data

- Training data is not enough for learning.
    - Too few examples
    - Too few patterns
- A learning curve with:
    - Keep decreasing training
    - Keep decreasing testing
    - A gap is still there

### Unrepresentative Test Data

- Test data is not sufficient for evaluation.
    - Too few examples
    - Too few patterns
- A learning curve with:
    - Keep decreasing training
    - The testing curve has noise
- Details: ML Diagnosing

## Dilemma

- We hope the training data represents all the data.
- We hope testing data represents all data.
- It might not be that ideal.
- How can we get a sense of all the data without breaking Rule#1?

## Cross-validation

### k-fold method:

- Partition the dataset into $k$ folds (randomly).
- Train your model $k$ times. Use a fold as testing data each time, and the others as training data.
- The Result is an array of k evaluation scores.
- $K = 5 - 10$ is most common in practice.
- If we have $n$ data points, how about $k = n$?

| Pros                             | Cons                      |
|----------------------------------|---------------------------|
| Better understanding of all data | Costly                    |
| Smaller bias, higher variance    | May result in overfitting |

> Scikit-learn: cross val score

