import json

# Read the input file
with open('input.json', 'r', encoding='utf-8') as f:
    data = json.load(f)

# Convert to simplified format
simplified = [{"sales_url": item["sales_url"]} for item in data]

# Write to output file
with open('output.json', 'w', encoding='utf-8') as f:
    json.dump(simplified, f, indent=2)

# Print the result
print(json.dumps(simplified, indent=2))