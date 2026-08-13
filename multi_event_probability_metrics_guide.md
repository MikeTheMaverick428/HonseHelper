# Guide to Multi-Event Probability Metrics & Table Presentation

When analyzing systems, processes, or workflows involving multiple independent events—especially when some events are guaranteed ($100\%$ probability) or recurring—relying solely on a single probability metric can obscure critical nuances.

This guide explains how to calculate and present four complementary metrics that preserve frequency, impact, and joint likelihood:
1. **Expected Value ($E[X]$)** — Measures expected total occurrences.
2. **Weighted Rating Score ($S_w$)** — Measures normalized process quality/reliability.
3. **Joint Occurrence Rate ($P(\text{All})$)** — Measures the exact likelihood that *all* events succeed concurrently.
4. **At-Least-One Probability ($P(\ge 1)$)** — Measures the probability that *at least one* event occurs successfully.

---

## 1. Metric Definitions & Mathematical Formulas

### A. Expected Value ($E[X]$)
* **Concept:** The total expected count of successful events across $n$ independent trials.
* **Formula:**
  $$E[X] = \sum_{i=1}^{n} P(A_i) = P(A_1) + P(A_2) + \dots + P(A_n)$$
* **Key Insight:** Because addition is linear, a $100\%$ event ($1.0$) contributes a full $+1.0$ to the total count, making $100\% + 80\%$ ($E[X] = 1.8$) clearly distinct from $100\% + 90\%$ ($E[X] = 1.9$).

### B. Weighted Rating Score ($S_w$)
* **Concept:** An overall process health or reliability score scaled from $0\%$ to $100\%$, incorporating individual event weights ($w_i$).
* **Formula:**
  $$S_w = \frac{\sum_{i=1}^{n} w_i \cdot P(A_i)}{\sum_{i=1}^{n} w_i} \times 100\%$$
  *(When all events have equal importance, $w_i = 1$, this simplifies to the simple average probability $\bar{P} = \frac{1}{n} \sum P(A_i)$).*
* **Key Insight:** Allows critical events (e.g., primary database transaction) to carry greater weight than secondary events (e.g., sending a notification email).

### C. Joint Occurrence Rate ($P(\text{All})$)
* **Concept:** The probability that *every single event* in the sequence occurs successfully.
* **Formula:**
  $$P(\text{All}) = \prod_{i=1}^{n} P(A_i) = P(A_1) \times P(A_2) \times \dots \times P(A_n)$$
* **Key Insight:** Identifies end-to-end process success rate for pipelines where any single failure causes the whole process to fail.

### D. At-Least-One Probability ($P(\ge 1)$)
* **Concept:** The likelihood that at least one event in the set occurs successfully.
* **Formula:**
  $$P(\ge 1) = 1 - P(\text{None}) = 1 - \prod_{i=1}^{n} (1 - P(A_i))$$
* **Key Insight:** If *any* event in the set has a $100\%$ probability ($1.0$), then $1 - P(A_1) = 0$, making $P(\ge 1) = 1 - 0 = \mathbf{100\%}$. It measures fallback safety/redundancy.

---

## 2. Master Comparison Table

The table below demonstrates how these four metrics effectively differentiate between scenarios that would otherwise be flattened into identical values under simple event counts:

| Scenario ID | Event Set Description | Probabilities Breakdown | Expected Value ($E[X]$) | Weighted Score ($S_w$, equal weights) | Joint Rate ($P(\text{All})$) | At-Least-One Rate ($P(\ge 1)$) | Recommended Table Indicator Badge |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: | :--- |
| **Scenario A** | 1 Guaranteed + 1 High Chance | `[ 100% \| 80% ]` | **1.80** | **90.0%** | **80.0%** | **100.0%** | 🟡 `1.8x \| 80.0% Joint \| 100% Any` |
| **Scenario B** | 1 Guaranteed + 1 Very High Chance | `[ 100% \| 90% ]` | **1.90** | **95.0%** | **90.0%** | **100.0%** | 🟢 `1.9x \| 90.0% Joint \| 100% Any` |
| **Scenario C** | 2 Guaranteed + 1 Coin Flip | `[ 100% \| 100% \| 50% ]` | **2.50** | **83.3%** | **50.0%** | **100.0%** | 🔵 `2.5x \| 50.0% Joint \| 100% Any` |
| **Scenario D** | 3 High Chance Events | `[ 90% \| 90% \| 90% ]` | **2.70** | **90.0%** | **72.9%** | **99.9%** | 🟠 `2.7x \| 72.9% Joint \| 99.9% Any` |
| **Scenario E** | 2 Unreliable Fallbacks | `[ 50% \| 40% ]` | **0.90** | **45.0%** | **20.0%** | **70.0%** | 🔴 `0.9x \| 20.0% Joint \| 70.0% Any` |

---

## 3. Step-by-Step Calculation Walkthrough

Consider a multi-stage data processing pipeline with 3 events:
* **Event 1 (Ingestion):** $P(A_1) = 100\%$ ($1.0$), Weight $w_1 = 5$ (Critical)
* **Event 2 (Transformation):** $P(A_2) = 90\%$ ($0.9$), Weight $w_2 = 3$ (High)
* **Event 3 (Export):** $P(A_3) = 80\%$ ($0.8$), Weight $w_3 = 1$ (Low)

### Step 1: Calculate Expected Value ($E[X]$)
$$E[X] = 1.0 + 0.9 + 0.8 = \mathbf{2.70 \text{ expected successful events}}$$

### Step 2: Calculate Weighted Rating Score ($S_w$)
$$\text{Weighted Sum} = (1.0 \times 5) + (0.9 \times 3) + (0.8 \times 1) = 5.0 + 2.7 + 0.8 = 8.5$$
$$\text{Total Weights} = 5 + 3 + 1 = 9$$
$$S_w = \frac{8.5}{9} \times 100\% = \mathbf{94.44\%}$$

### Step 3: Calculate Joint Occurrence Rate ($P(\text{All})$)
$$P(\text{All}) = 1.0 \times 0.9 \times 0.8 = \mathbf{0.720 \rightarrow 72.0\%}$$

### Step 4: Calculate At-Least-One Probability ($P(\ge 1)$)
$$P(\text{None}) = (1 - 1.0) \times (1 - 0.9) \times (1 - 0.8) = 0 \times 0.1 \times 0.2 = 0.0$$
$$P(\ge 1) = 1 - 0.0 = \mathbf{1.0 \rightarrow 100.0\%}$$

---

## 4. Implementation Code & Formulas

### Excel & Google Sheets
Assuming probabilities are in cells `B2:D2` and weights are in `B1:D1`:
* **Expected Value ($E[X]$):** `=SUM(B2:D2)`
* **Weighted Rating Score ($S_w$):** `=SUMPRODUCT(B2:D2, B$1:D$1) / SUM(B$1:D$1)`
* **Joint Occurrence Rate ($P(\text{All})$):** `=PRODUCT(B2:D2)`
* **At-Least-One Probability ($P(\ge 1)$):** `=1 - PRODUCT(INDEX(1 - B2:D2, ))`

### Python Implementation
```python
def calculate_event_metrics(probabilities, weights=None):
    # 1. Expected Value (E[X])
    expected_value = sum(probabilities)
    
    # 2. Joint Occurrence Rate (P(All))
    joint_occurrence = 1.0
    for p in probabilities:
        joint_occurrence *= p
        
    # 3. Weighted Rating Score (Sw)
    if weights is None:
        weighted_score = (sum(probabilities) / len(probabilities)) * 100.0
    else:
        weighted_score = (sum(p * w for p, w in zip(probabilities, weights)) / sum(weights)) * 100.0

    # 4. At-Least-One Probability (P(>=1))
    prob_none = 1.0
    for p in probabilities:
        prob_none *= (1.0 - p)
    at_least_one = 1.0 - prob_none
        
    return {
        "Expected Value (E[X])": round(expected_value, 2),
        "Weighted Score (Sw %)": round(weighted_score, 2),
        "Joint Rate (P(All) %)": round(joint_occurrence * 100.0, 2),
        "At-Least-One Rate (P(>=1) %)": round(at_least_one * 100.0, 2)
    }

# Example Usage
probs = [1.0, 0.9, 0.8]
weights = [5, 3, 1]
print(calculate_event_metrics(probs, weights))
```

---

## 5. Summary Guidelines for Dashboards
1. **Use Expected Value ($E[X]$)** when tracking total volume, resource usage, or retry attempts.
2. **Use Weighted Rating Score ($S_w$)** when summarizing overall system quality or SLA compliance.
3. **Use Joint Occurrence Rate ($P(\text{All})$)** when evaluating strict end-to-end pipeline reliability.
4. **Use At-Least-One Rate ($P(\ge 1)$)** when evaluating redundant systems, fallbacks, or safety nets where a single success is sufficient.
