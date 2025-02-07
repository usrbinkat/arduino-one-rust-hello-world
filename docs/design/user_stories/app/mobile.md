Gherkin user stories: **IoT Lightbulb App User Interface Design**

---

### **Feature: User Interface and Navigation**
#### **Scenario: User Navigates the App Efficiently**
```gherkin
Feature: UI/UX Navigation

  Scenario: User accesses the dashboard
    Given a user is authenticated
    When they log into the app
    Then they should be presented with a dashboard displaying device status, alerts, and controls
    And they should have access to menus for device management, automation, and access control
```

#### **Scenario: User Searches for a Device**
```gherkin
  Scenario: User searches for a registered IoT lightbulb
    Given a user has multiple registered lightbulbs
    When they enter a device name or ID in the search bar
    Then the app should display relevant results in real-time
    And allow direct access to the device’s control page
```

#### **Scenario: User Switches Between Dark and Light Mode**
```gherkin
  Scenario: User toggles dark mode
    Given a user prefers a dark UI theme
    When they enable dark mode in settings
    Then the app should update the entire UI to use a dark theme
    And the setting should persist across sessions and devices
```

---

### **Feature: Device Control and Management**
#### **Scenario: User Controls a Lightbulb from the App**
```gherkin
Feature: Device Control

  Scenario: User toggles light state via the app
    Given the user is authenticated
    And has a registered IoT lightbulb
    When they press the "Turn On" button in the app
    Then the backend should process the request
    And update the lightbulb’s state accordingly
    And confirm the change to the user in real-time
```

#### **Scenario: User Creates a Custom Lighting Profile**
```gherkin
  Scenario: User creates a new lighting profile
    Given the user is on the lighting profile settings page
    When they create a profile with a predefined brightness and color setting
    And save the profile
    Then the profile should be stored in the backend
    And be available for future use across all devices
```

---

### **Feature: User and Access Management**
#### **Scenario: Admin Manages User Roles**
```gherkin
Feature: User Management

  Scenario: Admin assigns roles to users
    Given an admin is authenticated
    When they access the user management panel
    Then they should be able to assign or revoke roles such as guest, user, or admin
    And changes should take effect immediately
```

#### **Scenario: User Updates Their Profile Information**
```gherkin
  Scenario: User modifies account settings
    Given a user is authenticated
    When they update their profile information (e.g., name, email, password)
    And save changes
    Then the app should validate and update the data in the backend
    And confirm the update to the user
```

---

### **Feature: Automation and Scheduling**
#### **Scenario: User Sets a Lighting Automation Rule**
```gherkin
Feature: Automation

  Scenario: User schedules an automation rule
    Given the user wants to automate their lighting
    When they create a schedule (e.g., turn lights on at sunset)
    Then the backend should store the automation settings
    And the app should confirm successful configuration
```

---

### **Feature: Voice Assistant Integration**
#### **Scenario: User Controls Lights via Voice Commands**
```gherkin
Feature: Voice Assistant Support

  Scenario: User issues a voice command
    Given the user has enabled voice assistant integration
    When they say "Turn off all lights" to a supported assistant
    Then the backend should process the command
    And update the devices accordingly
    And the app should reflect the updated states in real-time
```

#### **Scenario: User Configures Voice Assistant Preferences**
```gherkin
  Scenario: User links their voice assistant account
    Given a user wants to control devices via voice commands
    When they navigate to voice assistant settings
    And link their preferred assistant
    Then the integration should be confirmed
    And voice control should be enabled
```

---

### **Feature: Alerts and Notifications**
#### **Scenario: User Receives a Critical Notification**
```gherkin
Feature: Notifications

  Scenario: User receives an alert for a malfunctioning device
    Given a registered IoT lightbulb is reporting an error
    When the backend detects a malfunction
    Then the app should immediately notify the user via push notification
    And provide diagnostic information
```

#### **Scenario: User Manages Notification Preferences**
```gherkin
  Scenario: User configures notification settings
    Given the user wants to manage alert preferences
    When they enable or disable specific notifications in settings
    Then the app should store their preferences centrally
    And apply changes across all platforms
```

