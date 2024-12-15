# DS210_Final_Project
# **Transaction Network Analysis: LUNA Crash Dataset**

## **Context**

This project analyzes a cryptocurrency transaction dataset(mostly stable coins) centered around the crash of the Terra LUNA coin. The dataset includes Ethereum blockchain transactions involving stablecoins and WLUNA (Wrapped LUNA) before, during, and after the Terra LUNA crash in May 2022. Each transaction records details such as the sender, receiver, timestamp, contract address, and transaction value. I'm intended to use this dataset to explore the market behavior and structural changes in the transaction network surrounding this critical event in the cryptocurrency space.
However, the original dataset is too large to do analysis, so I select a range of timestamp to minimize the data used in this project, as the result, there are total of 30000 transactions being considered and analyze in this project(10000 for each graph, the timestamps are one week before LUNA crashed, one week during LUNA crashed, and one week after LUNA crashed).

### **Dataset Details**
The original dataset was provided by Chartlist that sponsored by Canadian NSERC. The total amount of transaction in "Stablecoin ERC20 transactions dataset" includes more than 70 millions transactions. The project is using one version of it that includes over 1 million transactions. This dataset "Stablecoin ERC20 transactions dataset" is split into three periods:
1. **Before the Crash**: One month prior to the LUNA crash.
2. **During the Crash**: The peak of the crash, when UST (TerraUSD) lost its 1 USD peg.
3. **After the Crash**: Six months following the collapse.

However, in this project I onlyt select 30000 rows from the original dataset and also splitted iunto three periods:
1. **Before the Crash**: One week prior to the LUNA crash.
2. **During the Crash**: One week during the crash, when the LUNA crashed until lower than 1 USD .
3. **After the Crash**: One week after the LUNA crash.
---

## **Project Goals**

The project investigates several key questions and trends in the transaction network to understand how the crash impacted the ecosystem:
1. **Graph Density**:
   - How does the overall connectivity of the transaction network evolve across the three periods?

2. **Degree Distribution**:
   - How are the nodes (addresses) connected, and does the distribution of connections (degrees) shift during the crash?

3. **Centrality Metrics**:
   - Identifying the most influential addresses before, during, and after the crash using degree and closeness centrality.

4. **Largest Connected Component**:
   - How does the size of the largest group of interconnected addresses change, and does the network become more fragmented during the crash?

---

## **Results**

### **1. Graph Density**
- **Before Crash**: 0.00044240038646614237
- **During Crash**: 0.0004227988524721431
- **After Crash**: 0.0006173681697250836

**Interpretation**:
- The network's density remained low throughout the periods, reflecting a sparse transaction network where only a small proportion of possible connections exist.
- However, there is a slight increase in density after the crash, possibly indicating tighter connections among fewer active participants.

---

### **2. Degree Distribution**
- Before Crash: The majority of nodes had very few connections (degree 1, with 3,823 nodes), but there were moderately connected nodes (degrees like 139 and 199) and a few highly connected hubs (maximum degree: 262).
- During Crash: A similar pattern persisted, with most nodes having degree 1 (3,966 nodes). Some hubs appeared with degrees up to 261, showing slightly more higher-degree nodes.
- After Crash: There were fewer high-degree nodes, with the majority still having degree 1 (2,975 nodes). The maximum degree dropped to 228, indicating reduced connectivity.


**Interpretation**:
- The degree distribution indicates that the network was highly decentralized before and during the crash, dominated by many low-degree nodes and a few high-degree hubs.
- After the crash, the reduction in high-degree nodes suggests that key players might have reduced activity or left the network.

---

### **3. Centrality Analysis**
#### **Top Degree Nodes (Most Connected Addresses):**
*Note: The centrality also implement the betweeness centraility but with too large runtime, so I did not include this part in analysis, the implementation is in the code with comment, remove the comment to check the result if you wish*
- **Before Crash**:
  - `0xb5d85cbf7cb3ee0d56b3bb207d5fc4b82f43f511`: 5.98% of total connections.
  - Other high-degree nodes had a similar share of the network.
- **During Crash**:
  - Slight increases in centrality for the top nodes (e.g., `0xb5d85cbf7cb3ee0d56b3bb207d5fc4b82f43f511`: 5.80%).
- **After Crash**:
  - A new address, `0x0000000000000000000000000000000000000000`, emerged as the top node (6.66% of connections), possibly reflecting a centralized recovery mechanism or significant single-entity involvement.

**Interpretation**:
- The emergence of a dominant node post-crash suggests a centralization trend, possibly due to concentrated rescue efforts or reduced network activity.

---

### **4. Largest Connected Component**
- **Before Crash**: 2,713 nodes.
- **During Crash**: 2,708 nodes.
- **After Crash**: 2,253 nodes.

**Interpretation**:
- The largest connected component shrank over time, reflecting increased fragmentation in the transaction network after the crash.
- This could indicate reduced trust and fewer active participants in the ecosystem.

---

## **Conclusion**

The project result shows some trend of the Ethereum transaction network during the Terra LUNA crash(since this project only select 30000 transactions from the dataset, it may be too small to show obvious trend):
1. **Decentralization to Fragmentation**:
   - The network started as highly decentralized, with few hubs playing a critical role.
   - Post-crash, the network became more fragmented, with a smaller largest connected component and fewer high-degree nodes.

2. **Slight Centralization Post-Crash**:
   - The rise of a dominant node after the crash indicates a shift toward centralization, likely a response to the chaotic market conditions.

3. **Low but Increasing Density**:
   - The slight increase in density post-crash reflects tighter activity among fewer remaining participants.

These findings highlight how crises have the influence on transaction networks, reducing overall participation while consolidating influence among key players.

---

## **How to Run the Project**
1. Clone the repository:
   ```bash
   git clone https://github.com/your-repo/transaction-network-analysis.git
2. Navigate to the directory: "crypto_coins_trend_analysis"
   ```bash
   cd crypto_coins_trend_analysis
3. Execute the command:
   ```bash
   cargo run
   cargo run --release   #For faster compilation
   cargo test            #Execute the utility tests module
5. The result of the project is also included into the github, it could be used for comparison.
Note: If the dataset is unavailable to directly clone from github, here's the link to downlaod the dataset:
   - https://chartalist.org/eth/StablecoinAnalysis.html
   - In the tab: Transaction graphs of the six networks, select the Version 1 that has 822 MB with descrioption "From Apr-28-2022 To May-24-2022"
   - Move the downloaded csv file to the parent directory of "crypto_coins_trend_analysis"
