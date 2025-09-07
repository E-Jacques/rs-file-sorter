# Semantic grouping

This strategy group multiple text document base on their semantic

## How does it work ?

We represent each document using a (100,) shaped array where each item is a number in `[0, 1[`. Those item are extracted from the top 100 values after running a [tf-idf](https://fr.wikipedia.org/wiki/TF-IDF) over every document. If the `tf-idf` output less than 100 items, we set the other value to 0.

> Note: all documents are expected to be in the same language.

Each document can now be grouped using a clustering algorithm.

> The algorithm needs to be determined
