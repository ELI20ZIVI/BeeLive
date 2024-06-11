import requests

# Note: this script will be better used in Linux by piping the output to the 'jq' tool,
# i.e. python3 get_events.py | jq

url = "http://localhost:8080/api/v3/events"
#params = { 'mode': '', 'addi': [], 'subi': [], 'addb': [], 'subb': []}
params = {}

response = requests.get(url, json=params)

print(response.request.url)
print(response.request.body)
print(response.request.headers)

if response.status_code == 200:
    print(response.text)
else:
    print("Error:", response.status_code)
