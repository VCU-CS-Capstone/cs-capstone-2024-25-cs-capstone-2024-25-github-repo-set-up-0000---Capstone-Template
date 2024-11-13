import torch
import torch.nn as nn
import torch.optim as optim
import torch.nn.functional as F

class RFSignalClassifier(nn.Module):
    def __init__(self, input_size=2048, hidden_size=1024, num_classes=3):
        super(RFSignalClassifier, self).__init__()
        # Define the layers
        self.fc1 = nn.Linear(input_size, hidden_size)  # Input layer to hidden layer
        self.fc2 = nn.Linear(hidden_size, hidden_size)  # Hidden layer to hidden layer
        self.fc3 = nn.Linear(hidden_size, num_classes)  # Hidden layer to output layer

    def forward(self, x):
        # Define forward pass with ReLU activation for hidden layers
        x = F.relu(self.fc1(x))  # First hidden layer with ReLU
        x = F.relu(self.fc2(x))  # Second hidden layer with ReLU
        x = self.fc3(x)  # Output layer (no activation here, we'll apply softmax later)
        return x

def train_model(model, train_loader, criterion, optimizer, num_epochs=20):
    model.train()
    for epoch in range(num_epochs):
        running_loss = 0.0
        correct_preds = 0
        total_preds = 0
        for inputs, labels in train_loader:
            optimizer.zero_grad()  # Zero the gradients
            
            # Forward pass
            outputs = model(inputs)
            
            # Compute the loss
            loss = criterion(outputs, labels)
            
            # Backward pass
            loss.backward()
            optimizer.step()
            
            running_loss += loss.item()
            
            # Compute accuracy
            _, predicted = torch.max(outputs, 1)
            correct_preds += (predicted == labels).sum().item()
            total_preds += labels.size(0)
        
        avg_loss = running_loss / len(train_loader)
        accuracy = 100 * correct_preds / total_preds
        print(f"Epoch [{epoch+1}/{num_epochs}], Loss: {avg_loss:.4f}, Accuracy: {accuracy:.2f}%")

def evaluate_model(model, test_loader):
    model.eval()
    correct_preds = 0
    total_preds = 0
    with torch.no_grad():
        for inputs, labels in test_loader:
            outputs = model(inputs)
            _, predicted = torch.max(outputs, 1)
            correct_preds += (predicted == labels).sum().item()
            total_preds += labels.size(0)
    
    accuracy = 100 * correct_preds / total_preds
    print(f"Test Accuracy: {accuracy:.2f}%")

# Example usage:
# Assuming train_loader and test_loader are PyTorch DataLoader objects with your training/testing data.

input_size = 2048  # Length of your input data (PSD array)
hidden_size = 1024  # Number of units in the hidden layers
num_classes = 3  # Output classes: Bluetooth, WiFi, None

# Initialize the model, loss function, and optimizer
model = RFSignalClassifier(input_size, hidden_size, num_classes)
criterion = nn.CrossEntropyLoss()  # For multi-class classification
optimizer = optim.Adam(model.parameters(), lr=0.001)

# Train the model
train_model(model, train_loader, criterion, optimizer, num_epochs=20)

# Evaluate the model
evaluate_model(model, test_loader)
