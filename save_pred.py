"""Generate input data"""
import pandas as pd
import numpy as np

X_test = np.random.rand(1000, 100)

df = pd.DataFrame({"X": X_test.tolist()})
df.to_parquet("data/predict.parquet")
print(df)