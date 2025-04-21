# Ensemble Methods

- What is the best? It is hard to answer. Since we don't know the future data.
- We can do model tuning, so try all hyperparameters.
- Or, we can combine multiple models - Ensemble! Rather than rely on one expert, you hire a team!
- Depending on how the team is built, we have:
    - Bagging
    - Boosting
    - Stacking

## Bootstrap Aggregating (Bagging)

Bagging is a method that combines the predictions of multiple models to improve the overall performance.

1. Step 1: Generate bootstrapped data sets. [See example](#bootstrap-example)
    - Size is the same as the original dataset
    - Random draw with replacement (we may have duplicates)
    - We have an out-of-bag dataset for each bootstrapped dataset, which contains the data points in the original
      dataset but not in the bootstrapped dataset.
2. Step 2: Train the model on the bootstrapped dataset.
    - Since the training data is bootstrapped, it represents a different perspective of the original training set.
    - The learning process can limit the features to be learned. Some features may be explored otherwise less likely to
      be considered.
    - The instance of the learning model represents a different perspective as well.
3. Step 3: Aggregate multiple instances of the learning model together.
    - The bootstrapped datasets and limited features to learn lead to variance and diversity among the ensemble's
      individuals.
    - Each learning model provides an individual assessment, and a mechanism is used to achieve a collective assessment.
        - Majority Voting for classifications (simple, weighted)
        - Averaging for regressions (simple, weighted)
        - > Example Random Forest

## Random Forest

Random datasets (Bagging): A bootstrapped dataset is a random subset of the original dataset with replacement.

Random subspaces (Feature limit): A dataset with limited features, as a random subset of all features.

Random patches: A learning model is trained using random datasets with random subspaces.


> Scikit-learn:
>- Random Forest Regressor
>- Random Forest Classifier
>
>Hyperparameters:
>- n_estimators: How many trees to build? The higher, the better, but slower. Default 100
>- bootstrap and max_samples: What is the size of the subset of samples?
>- max_features: What is the size of the subset of features?
>> - Regression: All
>> - Classification: sqrt(#features)
>- max_depth: How complete is each tree?

### Random Nearest Neighbor

How about we create ensembles by:

- Generating random datasets
- Using random features
- Averaging the assessments

## Boosting

Based on a list of conditions, we get to the posterior probability.
> Example
> For Spam emails, it's easy to find rules of thumb (weak learner) that are general and correct:
> - If title contains keywords1, predict spam
> - If title contains keywords2, predict spam
> - If content contains sale, predict spam, and so on.

It is hard to find a single rule that is very highly accurate.

Boosting involves incrementally building an ensemble.

Each new model instance emphasizes the training data that the previous instance didn't do well.

- In the beginning, all data points are weighted equally.
- Once a model instance learned, the training data didn't fit well will increase the weight (boosted).
- Thus, the next model instance will pay more attention to these training data (to focus on fitting errors, solving
  troubles)
- It tends to be more likely to overfit.

After repeating T times, all weak learners are combined into a single highly accurate rule.
> Example AdaBoost (Adaptive Boosting)

### AdaBoost Overfitting

Since the AdaBoost algorithm pays more attention to errors, it tends to fit the training set as much as possible.

> Similar to decision trees.

Focusing on errors leads to a complex final assessment.

Causes overfitting.

Hard to know when to stop training.

#### AdaBoost Advantages

- Fast, simple, and easy
- Only T needs to tune
- Flexible with any learning algorithms
- Effective
- No prior knowledge
- versatile: can be used with textual, numeric, and discrete data

### Gradient Boost

The main idea of boosting is to add new models to the ensemble sequentially.

Gradient Boost is a robust machine learning algorithm comprising Gradient descent and Boosting.

Gradient Boost can use any differentiable loss functions, thus it is more robust to outliers than AdaBoost and more
flexible.

Gradient Boost can be used for both classification and regression problems. AdaBoost was mainly designed for binary
classification problems.

## Bagging or Boosting?

Bagging works best for strong and complex models (e.g., fully developed decision trees)

- Often significantly better than a single classifier
- Noise: not considerably worse, more robust
- Prediction: provide improved accuracy

Boosting works best for weak models (shallow decision trees)

- Generally better than bagging
- May overfit the model to misclassify data

## Stacking

Stacking, or stacked generalization, involves training a learning algorithm to combine the predictions of several other
learning algorithms.

- Step 1: Train all other algorithms using the data.
- Step 2: Training the combiner algorithm using the prediction of the other algorithms as inputs.

In practice, a logistic regression model is often used as the combiner.

### Bootstrap example

Suppose we have a set of data.
$$x_{\text{original}}={x_1, x_2, x_3, x_4, x_5, x_6, x_7, x_8, x_9}$$

One possible bootstrapped data set could be
$$x_{b1} = {x_1, x_1, x_1, x_4, x_4, x_6, x_8, x_8, x_9}$$

> Note that $|x_{b1}| = |x_{\text{original}}|$

The associated out-of-bag data set would be
$$x_{\text{oob1}} = {x_2, x_3, x_5, x_7} =x_{\text{original}}-x_{b1}$$

The out-of-bag data set evaluates the model's performance trained on the bootstrapped data set.


