import json

data = json.load(open('./metadata.json'))
result = list()

print(len(data['collection']))
for item in data['collection']:

    # result.append({
    #     'address': item['address'],
    #     'membership': item['attributes'][0]['value']
    # })
    result.append(item['address'])


with open("cubesA.json", "w") as f:
    f.write(json.dumps(result, indent = 4))