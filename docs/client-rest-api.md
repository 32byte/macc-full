List of currently supported rest endpoints:
```
GET "/"
  Will return a link to this web page

GET "/blockchain"
  Expects: Nothing
  Returns: the whole blockchain in json format
GET "/height"
  Expects: Nothing
  Returns: the current block height
GET "/txstore"
  Expects: Nothing
  Returns: the transaction store in json format

POST "/transaction"
  Expects: Transaction struct in json format
  Returns: Status 200 if transaction could be deserialized
POST "/block"
  Expects: Block struct in json format
  Returns: Status 200 if block could be deserialized
```