Gherkin user stories: **IoT lightbulb endpoint device and button interactions**:

---

### **Feature: IoT Lightbulb Device User Control**
#### **Scenario: User Enrolls IoT Lightbulb via Mobile App**
```gherkin
Feature: IoT Lightbulb Enrollment

  Scenario: Successful enrollment of a new IoT lightbulb
    Given a user has a registered account in the IoT platform
    And the mobile app is open
    When the user scans the QR code on the lightbulb
    And confirms the enrollment
    Then the IoT lightbulb should be registered in the system
    And associated with the user’s account
    And display as "Online" in the app

  Scenario: Unsuccessful enrollment due to network failure
    Given a user attempts to enroll a new IoT lightbulb
    And the network connection is unavailable
    When the enrollment request is sent
    Then the system should notify the user of a network error
    And prompt to retry once connectivity is restored
```

---

### **Feature: IoT Lightbulb State Control**
#### **Scenario: User Manually Controls Lightbulb State**
```gherkin
Feature: Lightbulb State Control

  Scenario: User turns the lightbulb on and off
    Given the user is authenticated in the mobile app
    And the IoT lightbulb is online
    When the user taps the "Power On" button
    Then the lightbulb should turn on
    And its state should update in the mobile app

    When the user taps the "Power Off" button
    Then the lightbulb should turn off
    And its state should update in the mobile app

  Scenario: User adjusts brightness level
    Given the user has the IoT lightbulb turned on
    When the user selects a brightness level of 50%
    Then the lightbulb should dim to 50% brightness
    And the updated state should be reflected in the app

  Scenario: User sets RGBW color mix
    Given the IoT lightbulb supports RGBW control
    When the user selects a color combination of warm white with 20% blue
    Then the lightbulb should emit the specified color
    And save this as the current state
```

---

### **Feature: IoT Lightbulb Smart Mode and Network Outages**
#### **Scenario: IoT Lightbulb Handles Network Outages Gracefully**
```gherkin
Feature: Network Outage Handling

  Scenario: IoT lightbulb retains last known state during network loss
    Given the IoT lightbulb is set to a brightness of 75%
    And a network outage occurs
    When the user attempts to toggle the state manually
    Then the lightbulb should retain the last known state
    And allow local control using the physical button

  Scenario: IoT lightbulb auto-adjusts based on time of day
    Given the IoT lightbulb is set to auto mode
    When sunset occurs based on the stored geolocation
    Then the lightbulb should automatically turn on at 50% brightness
    And adjust brightness gradually as ambient light decreases
```

---

### **Feature: IoT Lightbulb Manual Control via Physical Button, NFC, and QR Code**
#### **Scenario: User Uses the Physical Button for State Changes**
```gherkin
Feature: Physical Button, NFC, and QR Code State Control

  Scenario: User toggles the lightbulb using the button
    Given the lightbulb is powered on
    When the user presses the physical button once
    Then the lightbulb should toggle between on and off

  Scenario: User changes brightness via button press sequences
    Given the lightbulb is on
    When the user double presses the physical button
    Then the brightness should increase to the next preset level

  Scenario: User resets the lightbulb to factory settings
    Given the lightbulb is powered on
    When the user presses and holds the reset button for 10 seconds
    Then the device should reset to factory defaults
    And prompt for re-enrollment via the mobile app

  Scenario: User controls lightbulb via NFC tap
    Given the lightbulb is NFC-enabled
    And the user has an authorized NFC tag or smartphone
    When the user taps the NFC tag on the lightbulb
    Then a captive portal should open on their device
    And present preset lighting options
    And if the user has elevated access, allow write privileges to the centralized repository

  Scenario: User controls lightbulb via QR code scan
    Given the lightbulb has a QR code displayed
    And the user scans the QR code using a smartphone or tablet
    When the system validates the user’s access level
    Then the user should be directed to a captive portal with lighting control options
    And if the user has the correct RBAC permissions, they should be able to modify global settings
```

---

### **Feature: IoT Lightbulb Smart Learning and Adaptive Control**
#### **Scenario: IoT Lightbulb Learns User Preferences**
```gherkin
Feature: Smart Learning Mode

  Scenario: IoT lightbulb adapts to user behavior over time
    Given the lightbulb is in self-teaching mode
    When the user adjusts brightness manually every evening at 7 PM
    Then the system should learn the pattern
    And automatically set the brightness at 7 PM in the future
```

