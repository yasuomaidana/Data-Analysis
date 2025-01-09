# Decision Trees

## Decision Tree

* Basic Algorithm (a greedy one)
    * Top-down, recursive, divide and conquer
    * Attribute selection and split
        * Selection criteria: Information Gain, Gain Ratio, Gini Index
        * Discrete value, continues value, subset.
    * Stop condition.
        * Complete tree
        * Tree pruning

## Information Gain

* The purer the data set is, the less information we need to remember.
* The purer the data set is, the easier we can make prediction.
* Entropy is to measure the impurity of a data set.

### Entropy

**Entropy**: measures the *impurity* of the set $D$. It is computed between $0$ (pure) and $1$ for binary
classification.

* Write as: $Info(D)$ $$Info(D)=-\sum_{i=1}^{m}p_i\log\left(p_i\right)$$ Where $m$ is the number of classes.
  $$p_i=\frac{\left|C_{i,D}\right|}{\left|D\right|}$$
  Where $\left|D\right|$ is the size of the set $D$, and $\left|C_{i,D}\right|$ is the size of the class $C_i$
* Shorthand write: $I\left(C_1,C_2,\dots,C_m\right)$

* Binary: $I(a, b)$ means $a$ data set has $a+b$ data points, and $a$ of them belong to class 1, and $b$ of them belong
  to class 2.
  $$I\left(a,b\right)=-\left(\frac{a}{a+b}\log_2\left(\frac{a}{a+b}\right)+\frac{b}{a+b}\log_2\left(\frac{b}{a+b}\right)\right)$$

### Tree using information gains

* Information *needed* using attribute $A$ to classify $D$:
    * Write as $$info_A(D)$$
      $$Info_A(D)=\sum_{j=1}^{v}\frac{\left|D_j\right|}{\left|D \right|}\times Info{D_j}$$
* Information Gain: The *decrease* of
  entropy using attribute $A$ to classify $D$:
    * Write as $Gain(A)$
      $$Gain(A)=Info(D)-Info_A(D)$$
* Continuous-valued attribute $A$
* Determine the best split point for $A$
    * Sort $A$ values in increasing order
    * Consider the midpoint of adjacent values: $$\frac{a_i + a_{i+ 1}}2$$
    * Pick the midpoint $w/$ minimum $info_A(D)$
* Split: $D_1: A <= \text{split point; } D_2: A > \text{split Point}$
* Choose the feature having the highest value of
  information gain to split

### Gain Ratio (C4.5)

Information gain measures biased towards attributes with a large number of values.

* What is $Info_{CID}(D)$ and $Gain(CID)$ for previous example?

* Select attribute with maximum gain ratio
  $$GainRatio_A(D)=\frac{Gain_A(D)}{SplitInfo_A(D)}$$
  $$SplitInfo_A(D)=-\sum_{j=1}^v\frac{\left|D_j\right|}{\left|D\right|}\times\log_2{\left(\frac{\left|D_j\right|}{\left|D\right|}\right)}$$

> [Gain Ratio (C4.5)](https://en.wikipedia.org/wiki/C4.5_algorithm) (a successor
> of [ID3](https://en.wikipedia.org/wiki/ID3_algorithm)) is used to overcome the bias of information gain towards
> attributes with a large number

## Gini index

* Gini index $$Gini(D)=1-\sum_{i=1}^mp_i^2$$
* Binary split with attribute $A$
  $$Gini_A(D)=\frac{\left|D_1\right|}{\left|D\right|}Gini(D_1)+\frac{\left|D_2\right|}{\left|D\right|}Gini(D_2)$$
* Reduction in impurity $$\Delta Gini(A)=Gini(D)-Gini_A(D)$$

### Attribute Selection Measures

* Comparison of the three measures
    * Good results in general but some biases
    * Information gain: multivalued attributes
    * Gain ratio: unbalanced splits
    * Gain ratio: unbalanced splits
    * Gini index: multivalued, equal-sized & pure partitions, not good when number of classes is large

## Considerations

| Method           | Consideration                                                                         |
|------------------|---------------------------------------------------------------------------------------|
| Information gain | Multi-valued attributes                                                               |
| Gain ratio       | Unbalanced splits                                                                     |
| Gini index       | Multi-valued, equal-sized & pure partitions, not good when number of classes is large |

## Splitting Attributes

* Discrete-valued
    * Each value may lead to a subtree
* Continuous-valued: *split_point*
    * Each midpoint of two values may lead to a subtree
* Subsets

## Decision Tree inductions

* Stopping conditions
    * All samples belong to the same class
    * No remaining attributes: majority voting
    * No sample left

## Overfitting & Tree Pruning

* Overfitting of the training data
    * Too many branches, reflect anomalies due to noise or outliers
    * Poor accuracy for unseen data
* Tree pruning to avoid over fitting
    * Pre-pruning: halt tree construction early
    * Post-pruning: remove branches from a "fully-grown" tree
