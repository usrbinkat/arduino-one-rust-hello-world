Gherkin user stories: **IoT Lightbulb Backend and Universal App Features**

---

### **Feature: User Customization and Device Grouping**
#### **Scenario: User Creates and Manages Device Groups for Bulk Actions**
```gherkin
Feature: Device Grouping and Customization

  Scenario: User creates a custom device group
    Given a user is authenticated in the system
    And has one or more registered IoT lightbulbs
    When the user selects multiple lightbulbs in the app
    And assigns them to a new group with a custom name
    Then the backend should store the group metadata centrally
    And associate the selected devices with the new group
    And allow the user to apply state changes to the entire group
```

#### **Scenario: User Updates Device Group State**
```gherkin
  Scenario: User updates the state of a device group
    Given the user has an existing device group
    And the devices in the group are online
    When the user sets a new power state, brightness, or color for the group
    Then the backend should propagate the update to all group members
    And ensure state consistency across all grouped devices
```

#### **Scenario: User Deletes a Device Group**
```gherkin
  Scenario: User removes a device group
    Given the user has an existing device group
    When the user selects the group and chooses "Delete"
    Then the backend should disassociate all devices from the group
    And remove the group metadata from the knowledge graph
```

---

### **Feature: User Enrollment and Access Management**
#### **Scenario: User Grants Temporary Access to a Guest**
```gherkin
Feature: User Access and Permissions

  Scenario: User grants guest access via QR or NFC
    Given a user owns an IoT lightbulb
    And wants to allow temporary control for a guest
    When the user generates a time-limited guest access token
    Then the backend should create a record linking the guest to the device
    And enforce expiration and access limits on guest permissions
```

#### **Scenario: User Revokes Guest Access**
```gherkin
  Scenario: User revokes guest access
    Given a guest was previously granted access
    When the user selects the guest access entry and revokes permissions
    Then the backend should immediately terminate guest control
    And remove the guest’s ability to interact with the lightbulb
```

---

### **Feature: Universal App Interface for Device Control**
#### **Scenario: User Controls Lightbulb from Any Device**
```gherkin
Feature: Universal App Control

  Scenario: User controls IoT lightbulb from any interface
    Given a user is authenticated via any platform (web, mobile, voice assistant, AI chatbot)
    When the user sends a command to change device state
    Then the backend should update the device state centrally
    And synchronize the change across all user interfaces
    And provide real-time feedback of the updated state
```

#### **Scenario: User Schedules Automated Light Adjustments**
```gherkin
  Scenario: User sets an automation schedule
    Given a user owns one or more IoT lightbulbs
    When the user schedules an automation rule (e.g., "Turn on at sunset")
    Then the backend should store the rule in the knowledge graph
    And execute the rule based on contextual triggers
    And allow the user to modify or delete the schedule at any time
```

#### **Scenario: User Reviews Device Activity Logs**
```gherkin
  Scenario: User checks historical device activity
    Given a user is authenticated
    When the user requests activity logs for a device
    Then the backend should return a chronological log of state changes
    And include timestamps, user actions, and automation triggers
```

---

### **Feature: Voice and AI Assistant Integration**
#### **Scenario: User Controls Lights via Voice Assistant**
```gherkin
Feature: AI and Voice Assistant Support

  Scenario: User issues a voice command to control a light
    Given the user is authenticated via a supported assistant (Alexa, Siri, Google Assistant, ChatGPT)
    When the user says "Turn off the living room light"
    Then the backend should process the command
    And update the IoT lightbulb state centrally
    And return confirmation to the user through the assistant
```

#### **Scenario: User Queries Device Status via AI**
```gherkin
  Scenario: User requests the current state of a lightbulb
    Given the user has one or more IoT lightbulbs registered
    When the user asks an AI assistant "Is the bedroom light on?"
    Then the backend should retrieve the latest state from the knowledge graph
    And provide a natural language response with the status
```

---

### **Feature: Device Identity Security & Pairing**
#### **Scenario: User Pairs a New Lightbulb via NFC and QR Code**
```gherkin
Feature: Secure Device Pairing

  Scenario: User pairs a new IoT lightbulb securely
    Given a new IoT lightbulb is powered on
    And the user is authenticated
    When the user taps their phone on the NFC tag
    Or scans the QR code on the device
    Then the backend should verify the device’s unique identity signature
    And register the device to the user’s account
    And store an event record of the pairing attempt
```

#### **Scenario: Unauthorized Pairing Attempt is Blocked**
```gherkin
  Scenario: A non-owner attempts to pair an enrolled device
    Given an IoT lightbulb is already registered
    When an unauthenticated user attempts to pair the device
    Then the backend should reject the pairing request
    And log the unauthorized attempt in the system
```

---

### **Feature: Centralized User Experience & Synchronization**
#### **Scenario: User Preferences Sync Across All Interfaces**
```gherkin
Feature: Unified User Experience

  Scenario: User updates preferences on one platform and sees them on all others
    Given a user modifies their device settings in the mobile app
    When they log in via a different interface (e.g., web, voice assistant, AI client)
    Then the backend should ensure all preferences are consistent
    And synchronize changes instantly
```

