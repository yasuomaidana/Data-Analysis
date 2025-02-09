# Naive Bayes Classification

Naive Bayes is a classification algorithm that is based on the Bayes theorem. It is called "naive" because it makes the
assumption that the features are independent of each other. This assumption simplifies the computation of the
probabilities and makes the algorithm very efficient.

* A statistical classifier:
    * Predicts class membership probabilities
* Foundation: based on Bayes theorem
* Performance (naive bayes classifier):
    * Comparable to decision trees and logistic regression
    * Some neural networks classifiers
* Incremental learning: can be updated easily with new data

## Bayes Theorem

* $X$: a data sample (evidence), class unknown
  * e.g., $X: \text{age}=35, \ \text{income}=\$40,000$
* $H$: a hypothesis (class label) that $X$ belongs to class $C$
  * e.g., $H$: buys a computer
* Classification: determine $P(H|X)$

![Bayes theorem visual help](Bayes_theorem_visual_proof.png)

> $P(H|X) = \frac{P(X|H)P(H)}{P(X)}$
> * $P(H)$: prior probability (known), the initial probability of the hypothesis $H$ before observing the data
> * $P(X)$: marginalization (known), the total probability of observing the data sample $X$ under all poss
> * $P(X|H)$: likelihood (known), the probability of observing the data sample $X$ given that the hypothesis $H$ is true
> * $P(H|X)$: posterior probability (unknown), the probability of the hypothesis $H$ given the data sample $X$

* $A,B$ are two events:
    * $P(A \& B)$: The probability of both events $A$ and $B$ occurring
    * $P(A|B)$: The probability of event $A$ occurring given that $B$ has occurred
    * $P(A \& B) = P(A|B)P(B),\ P(B)>0$
