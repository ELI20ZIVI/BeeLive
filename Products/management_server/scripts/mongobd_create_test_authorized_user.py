from pymongo import MongoClient

# MongoDB connection string
client = MongoClient("mongodb://BeeLive:BeeLive@localhost:27017")

# Database and collection
db = client.beelive_test
collection = db.authorized_users

# User data
user = {
    "id": "0b26ece6-33d1-45b3-a4f0-ba69d218a531",
    "username": "BeeLive",
    "email": "example@example.com",
    "categories": []
}

# Insert the user document into the collection
result = collection.insert_one(user)

# Print the ID of the inserted document
print(f"Inserted document ID: {result.inserted_id}")
