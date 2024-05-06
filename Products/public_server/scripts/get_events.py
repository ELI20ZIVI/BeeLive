import requests

url = "http://localhost:8080/events"
params = { 'mode': '', 'addi': [], 'subi': [], 'addb': [], 'subb': []}

response = requests.get(url, json=params)

if response.status_code == 200:
    print(response.text)
else:
    print("Error:", response.status_code)
