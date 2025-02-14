- Accuracy: classification vs. prediction
- Speed: time to construct/use the model
- Robustness: handling noise and missing values
- Scalability: large amounts of data
- Interpretability: understanding and insight
- Goodness of rules: e.g., decision tree size, compactness of classification rules
## Classification evaluation

Confusion matrix:
![[Confusion matrix.jpeg]]

- **Accuracy**:
	- It measures your model performance in **general**.
	- The **higher** the accuracy, the **better** the classifier is in **general**.
$$\frac{ \# accurate}{\#total} = \frac{TP+TN}{P+N}$$
<hr>
- **Precision**: 
	- How accurate is the predicted positive?
	$$\frac{ \# true\ positive}{\#predicted\ positive} = \frac{TP}{TP+FP}$$
> If it is 1, there is no FP. All predicted positive cases are actually positive.
 >If it is 0, there is no TP. All predicted positive cases are actually negative.
<hr>
- **Recall (Sensitivity or True Positive Rate)**: 
	- How accurate for actual positive
	- From all the positive cases, how many did the model predict correctly?
	$$\frac{ \# true\ positive}{\#positive} = \frac{TP}{TP+FN} =\frac{TP}{P}$$
> If it is 1, there is no FN, and all positive cases are predicted as positive.
 > If it is 0, there is no TP, and all positive cases are predicted as negative.
<hr>
**[F1-Score](https://encord.com/blog/f1-score-in-machine-learning/)**: The F1-Score strikes a balance between precision and recall, making it useful when false positives and false negatives are similar in importance.
$$F1=2\frac{Sensitivity\cdot Precision}{Sensitivity+Precision}$$
 <hr>
- **Specificity (True Negative Rate)**:
	- How accurate for actual I negative?
	- This metric is vital when the emphasis is on accurately identifying negative instances.
$$\frac{ \# true\ negative}{\#negative} = \frac{TN}{TN+FP} =\frac{TN}{N}$$
<hr>
* ROC Curve
Receiver Operating Characteristics Curve

- TPR: True positive rate (sensitivity)
- FPR: False positive rate
	- How many actual negative cases are predicted as positive?
	$$\frac{ \# false\ positive}{\#negative} = \frac{FP}{TP+FP} =\frac{FP}{N}$$

Think about a threshold for classifying positive cases.
* The higher the threshold, the lower false positive rate, the lower the true positive rate as well.
* The lower the threshold, the higher false positive rate, the higher the true positive as well.
* 
![[Roc_curve.png]]

## Sensitivity and precision trade-off

![[precision vs recall.png]]
### Precision vs recall curve

![[precision vs recall curve.png.jpeg]]
## Evaluation metrics for different

| Task                         | Evaluation metrics                                                                                  |
| ---------------------------- | --------------------------------------------------------------------------------------------------- |
| **Classification**           | Accuracy, precision, recall, F1 score, and AUC-ROC.                                                 |
| **Regression**               | Mean Squared Error (MSE), Root Mean Squared Error (RMSE), Mean Absolute Error (MAE), and R-squared. |
| **Clustering**               | Silhouette score, Dunn index, and Rand index.                                                       |
| **Ranking & recommendation** | MAP, NDCG, and precision at K.                                                                      |

### References
[Confusion Matrix](https://encord.com/glossary/confusion-matrix/)
[F1 Score in Machine Learning](https://encord.com/blog/f1-score-in-machine-learning/)
