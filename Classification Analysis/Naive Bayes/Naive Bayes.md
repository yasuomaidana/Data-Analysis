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

### Bayes Theorem for three events

A version of Bayes' theorem for 3 events results from the addition of a third event $C$, with $P(C)>0$, on which all
probabilities are conditioned:

$$P(A|B \cap C)= \frac{P(B| A \cap C)P(A|C)}{P(B| C)}$$

This is a generalization of the chain rule for 3 events:
$$
P(A \cap B \cap C) = P(A|B \cap C)P(B \cap C) = P(A|B \cap C)P(B|C)P(C)
$$
$$
P(A \cap B \cap C) = P(B|A \cap C)P(A \cap C) = P(B|A \cap C)P(A|C)P(C)
$$

as you can see, you can keep adding more events to a bayesian chain rule.

> ### Chain rule
>
> $P(A \cap B) = P(B|A)\cdot P(A)$ where $P(B|A)$ denotes the conditional probability of $B$ given $A$.
>
> ![Probability formulas](Probability%20formulas.png)
>
> #### Two events
>
> For two events $A$ and $B$, the chain rule states that:
> $$P(A \cap B) = P(B|A)\cdot P(A)$$
> Where $P(A \cap B)$ denotes the conditional probability of $B$ given $A$.
> #### Finitely many events
> For events $A_1, A_2, \ldots, A_n$ whose intersection has not probability zero, the chain rule states:
$$\begin{align}
P(A_1 \cap A_2 \cap \ldots \cap A_n) & = P(A_n | A_1 \cap \ldots \cap A_{n-1})\cdot P(A_1 \cap \dots \cap A_{n-1})
\\ & = P(A_n | A_1 \cap \ldots \cap A_{n-1})\cdot P(A_{n-1} | A_1 \cap \ldots \cap A_{n-2})\cdot P(A_1 \cap \dots \cap A_{n-2})
\\ & = P(A_n | A_1 \cap \ldots \cap A_{n-1})\cdot P(A_{n-1} | A_1 \cap \ldots \cap A_{n-2})\cdot \ldots \cdot P(A_3|A_1 \cap A_2) \cdot P(A_2|A_1) \cdot P(A_1)
\\ & = \prod_{k=1}^{n} P(A_k | \bigcap_{j=1}^{k-1} A_j)
\\ & = P(A_1) \prod_{j=2}^{n} P(A_j | A_1 \cap \ldots \cap A_{j-1})
\end{align}$$
> 