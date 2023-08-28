import pandas as pd


def food_delivery(delivery: pd.DataFrame) -> pd.DataFrame:
    n = delivery[delivery['order_date'] ==
                 delivery['customer_pref_delivery_date']].size
    p = round(n * 100 / delivery.size, 2)
    return pd.DataFrame({'immediate_percentage': [p]})
