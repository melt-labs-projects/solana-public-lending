import json

COLLECTION_NAME = 'stoned-frogs'

data = json.load(open(f'./collections/{COLLECTION_NAME}/{COLLECTION_NAME}.json'))
result = list()

print(len(data))
# for item in data:
#     result.append({'address': item})

# with open(f"./collections/{COLLECTION_NAME}/processed.json", "w") as f:
#     f.write(json.dumps(result, indent = 4))