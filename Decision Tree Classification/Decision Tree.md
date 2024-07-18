# Decision Trees

## Decision Tree

* Basic Algorithm (a greedy one)
  * Top-down, recursive, divide and conquer
  * Attribute selection and split
    * Selection criteria: Information Gain, Gain Ratio, Gini Index
    * Discrete value, continues value, subset.
  * Stop condition.
    * Complete tree
    * Tree prunning

## Information Gain

* The purer the data set is, the less information we need to remember.
* The purer the data set is, the easier we can make prediction.
* Entropy is to measure the impurity of a data set.

### Entropy

**Entropy**: measures the *impurity* of the set $D$. It is computed between $0$ (pure) and $1$ for binary classification.

* Write as: $Info(D)$
  * $Info(D)=-\sum_{i=1}^{m}p_i\log\left(p_i\right)$, where $m$ is the number of classes.
  * $p_i=\frac{\left|C_{i,D}\right|}{\left|D\right|}$, where $\left|D\right|$ is the size of the set $D$, and $\left|C_{i,D}\right|$ is the size of the class $C_i$
* Shorthad write: $I\left(C_1,C_2,\dots,C_m\right)$

* Binary: $I(a, b)$ means $a$ data set has $a+b$ data points, and $a$ of them belong to class 1, and $b$ of them belong to class 2.
  * $I\left(a,b\right)=-(\frac{a}{a+b}\log_2\left(\frac{a}{a+n}\right)+\frac{b}{a+b}\log_2\left(\frac{b}{a+n}\right))$

### Tree using information gains

* Information *needed* using attribute $A$ to classify $D$:
  * Write as $info_A(D)$
  * $Info_A(D)=\sum_{j=1}^{v}\frac{\left|D_j\right|}{\left|D \right|}\times Info{D_j}$
* Information Gain: The *decrease* of
entropy using attribute $A$ to classify $D$:
  * Write as $Gain(A)$
  * $Gain(A)=Info(D)-Info_A(D)$
* Continuous-valued attribute $A$
* Determine the best split point for $A$
  * Sort $A$ values in increasing order
  * Consider the midpoint of adjacent values: $\frac{a_i + a_{i+ 1}}2$
  * Pick the midpoint $w/$ minimum $info_A(D)$
* Split: $D_1: A <= \text{split point; } D_2: A > \text{split Point}$
* Choose the feature having the highest value of
information gain to split

### Gain Ratio (C4.5)

Information gain measures biased towards attributes with a large number of values.
C4.5 (a successor of ID3)

* Select attribute with maximum gain ratio
* $GainRatio_A(D)=\frac{Gain_A(D)}{SplitInfo_A(D)}$
* $SplitInfo_A(D)=-\sum_{j=1}^v\frac{\left|D_j\right|}{\left|D\right|}\times\log_2{\left(\frac{\left|D_j\right|}{\left|D\right|}\right)}$
