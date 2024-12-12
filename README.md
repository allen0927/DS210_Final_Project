# DS210_Final_Project
# **Transaction Network Analysis: LUNA Crash Dataset**

## **Context**

This project analyzes a cryptocurrency transaction dataset(mostly stable coins) centered around the crash of the Terra LUNA coin. The dataset includes Ethereum blockchain transactions involving stablecoins and WLUNA (Wrapped LUNA) before, during, and after the Terra LUNA crash in May 2022. Each transaction records details such as the sender, receiver, timestamp, contract address, and transaction value. 

### **Dataset Details**
The dataset was provided by Jason Zhu and colleagues from U Manitoba and UT Dallas. It includes over 70 million transactions, split into three periods:
1. **Before the Crash**: One month prior to the LUNA crash.
2. **During the Crash**: The peak of the crash, when UST (TerraUSD) lost its 1 USD peg.
3. **After the Crash**: Six months following the collapse.

This dataset allows us to explore the market behavior and structural changes in the transaction network surrounding this critical event in the cryptocurrency space.

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
